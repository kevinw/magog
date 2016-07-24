use num::Integer;
use euclid::Point2D;
use calx_grid::Dir6;

/// Field of view iterator for a hexagonal map.
///
/// Takes a function that
/// indicates opaque cells and yields the visible locations from around the
/// origin up to the given radius.
pub struct HexFov<F> {
    /// Predicate for whether a given point will block the field of view.
    is_opaque: F,
    range: u32,
    stack: Vec<Sector>,
    fake_isometric_hack: bool,
    /// Extra values generated by special cases.
    side_channel: Vec<Point2D<i32>>,
}

impl<F> HexFov<F>
    where F: Fn(Point2D<i32>) -> bool
{
    pub fn new(is_opaque: F, range: u32) -> HexFov<F> {
        let init_group = is_opaque(Dir6::from_int(0).to_v2());
        HexFov {
            is_opaque: is_opaque,
            range: range,
            stack: vec![Sector {
                            begin: PolarPoint::new(0.0, 1),
                            pt: PolarPoint::new(0.0, 1),
                            end: PolarPoint::new(6.0, 1),
                            group_opaque: init_group,
                        }],
            fake_isometric_hack: false,
            // The FOV algorithm will not generate the origin point, so we use
            // the side channel to explicitly add it in the beginning.
            side_channel: vec![Point2D::new(0, 0)],
        }
    }

    /// Make wall tiles in acute corners visible when running the algorithm so
    /// that the complete wall rectangle of fake-isometric rooms will appear
    /// in the FOV.
    pub fn fake_isometric(mut self) -> HexFov<F> {
        self.fake_isometric_hack = true;
        self
    }
}

impl<F> Iterator for HexFov<F>
    where F: Fn(Point2D<i32>) -> bool
{
    type Item = Point2D<i32>;
    fn next(&mut self) -> Option<Point2D<i32>> {
        if let Some(ret) = self.side_channel.pop() {
            return Some(ret);
        }

        if let Some(mut current) = self.stack.pop() {
            if current.pt.is_below(current.end) {
                let pos = current.pt.to_v2();
                let current_opaque = (self.is_opaque)(pos);

                // Terrain opacity changed, branch out.
                if current_opaque != current.group_opaque {
                    // Add the rest of this sector with the new opacity.
                    self.stack.push(Sector {
                        begin: current.pt,
                        pt: current.pt,
                        end: current.end,
                        group_opaque: current_opaque,
                    });

                    // If this was a visible sector and we're below range, branch
                    // out further.
                    if !current.group_opaque &&
                       current.begin.radius < self.range {
                        self.stack.push(Sector {
                            begin: current.begin.further(),
                            pt: current.begin.further(),
                            end: current.pt.further(),
                            group_opaque: (self.is_opaque)(current.begin
                                                                  .further()
                                                                  .to_v2()),
                        });
                    }
                    return self.next();
                }

                // Hack for making acute corner tiles of fake-isometric rooms
                // visible.

                // XXX: Side cells should only be visible with wallform tiles,
                // but the FOV API can't currently distinguish between
                // wallform and blockform FOV blockers.
                if self.fake_isometric_hack {
                    if let Some(side_pt) = current.pt.side_point() {
                        // Only do this if both the front tiles and the target
                        // tile are opaque.
                        let next = current.pt.next();
                        if next.is_below(current.end) && current.group_opaque &&
                           (self.is_opaque)(next.to_v2()) &&
                           (self.is_opaque)(side_pt) &&
                           current.begin.radius < self.range {
                            self.side_channel.push(side_pt);
                        }
                    }
                }

                // Proceed along the current sector.
                current.pt = current.pt.next();
                self.stack.push(current);
                return Some(pos);
            } else {
                // Hit the end of the sector.

                // If this was a visible sector and we're below range, branch
                // out further.
                if !current.group_opaque && current.begin.radius < self.range {
                    self.stack.push(Sector {
                        begin: current.begin.further(),
                        pt: current.begin.further(),
                        end: current.end.further(),
                        group_opaque: (self.is_opaque)(current.begin
                                                              .further()
                                                              .to_v2()),
                    });
                }

                self.next()
            }
        } else {
            None
        }
    }
}

struct Sector {
    /// Start point of current sector.
    begin: PolarPoint,
    /// Point currently being processed.
    pt: PolarPoint,
    /// End point of current sector.
    end: PolarPoint,
    /// Currently iterating through a sequence of opaque cells.
    group_opaque: bool,
}

/// Points on a hex circle expressed in polar coordinates.
#[derive(Copy, Clone, PartialEq)]
struct PolarPoint {
    pos: f32,
    radius: u32,
}

impl PolarPoint {
    pub fn new(pos: f32, radius: u32) -> PolarPoint {
        PolarPoint {
            pos: pos,
            radius: radius,
        }
    }
    /// Index of the discrete hex cell along the circle that corresponds to this point.
    fn winding_index(self) -> i32 {
        (self.pos + 0.5).floor() as i32
    }

    pub fn is_below(self, other: PolarPoint) -> bool {
        self.winding_index() < other.end_index()
    }
    fn end_index(self) -> i32 {
        (self.pos + 0.5).ceil() as i32
    }

    pub fn to_v2(self) -> Point2D<i32> {
        if self.radius == 0 {
            return Point2D::new(0, 0);
        }
        let index = self.winding_index();
        let sector = index.mod_floor(&(self.radius as i32 * 6)) /
                     self.radius as i32;
        let offset = index.mod_floor(&(self.radius as i32));

        let rod = Dir6::from_int(sector).to_v2();
        let tangent = Dir6::from_int((sector + 2) % 6).to_v2();

        rod * (self.radius as i32) + tangent * offset
    }

    /// If this point and the next point are adjacent vertically (along the xy
    /// axis), return a tuple of the point outside of the circle between the
    /// two points.
    ///
    /// This is a helper function for the FOV special case where acute corners
    /// of fake isometric rooms are marked visible even though strict hex FOV
    /// logic would keep them unseen.
    pub fn side_point(self) -> Option<Point2D<i32>> {
        let next = self.next();
        let a = self.to_v2();
        let b = next.to_v2();

        if b.x == a.x + 1 && b.y == a.y + 1 {
            // Going down the right rim.
            Some(Point2D::new(a.x + 1, a.y))
        } else if b.x == a.x - 1 && b.y == a.y - 1 {
            // Going up the left rim.
            Some(Point2D::new(a.x - 1, a.y))
        } else {
            None
        }
    }

    /// The point corresponding to this one on the hex circle with radius +1.
    pub fn further(self) -> PolarPoint {
        PolarPoint::new(self.pos * (self.radius + 1) as f32 /
                        self.radius as f32,
                        self.radius + 1)
    }

    /// The point next to this one along the hex circle.
    pub fn next(self) -> PolarPoint {
        PolarPoint::new((self.pos + 0.5).floor() + 0.5, self.radius)
    }
}

