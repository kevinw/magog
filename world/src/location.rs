use calx_alg::{compact_bits_by_2, noise, spread_bits_by_2};
use calx_grid::{Dir6, GridNode, HexGeom};
use euclid::{Vector2D, vec2};
use std::num::Wrapping;
use std::ops::{Add, Sub};
use terraform::TerrainQuery;

pub const SECTOR_WIDTH: i32 = 40;
pub const SECTOR_HEIGHT: i32 = 22;

/// Unambiguous location in the game world.
#[derive(Copy, Eq, PartialEq, Clone, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Location {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

/// The type for a unique location in the game world.
///
/// IMPORTANT: Be careful where you use the simple "location + vec" algebra. That does not take
/// portals into account, and will usually cause unwanted effects near them in high-level code.
/// Most high-level logic should use `Location::jump` to displace locations. This will correctly
/// traverse portals.
impl Location {
    pub fn origin() -> Location { Location { x: 0, y: 0, z: 0 } }

    pub fn new(x: i8, y: i8, z: i8) -> Location { Location { x, y, z } }

    /// Construct a Location from a Morton code representation.
    ///
    /// Use the representation generated with `to_morton`. The odd low 16 bits are compacted to x
    /// value, the even low 16 bits to z and the first 8 of the high 16 bits become z.
    pub fn from_morton(morton_code: u32) -> Location {
        let xy = morton_code & 0xffff_ffff;
        let x = compact_bits_by_2(xy) as u8;
        let y = compact_bits_by_2(xy >> 1) as u8;
        let z = (morton_code >> 16) as u8;

        unsafe {
            Location {
                x: ::std::mem::transmute(x),
                y: ::std::mem::transmute(y),
                z: ::std::mem::transmute(z),
            }
        }
    }

    /// Turn the Location to a Morton code value.
    ///
    /// Spatially close locations are often numerically close in Morton codes, these are useful for
    /// quadtree-like structures.
    pub fn to_morton(&self) -> u32 {
        let mut ret = 0;
        let x: u8 = unsafe { ::std::mem::transmute(self.x) };
        let y: u8 = unsafe { ::std::mem::transmute(self.y) };
        let z: u8 = unsafe { ::std::mem::transmute(self.z) };
        ret ^= spread_bits_by_2(x as u32);
        ret ^= spread_bits_by_2(y as u32) << 1;
        ret ^= (z as u32) << 16;
        ret
    }

    /// Vector pointing from this location into the other one if the locations
    /// are on the same Euclidean plane.
    pub fn v2_at(&self, other: Location) -> Option<Vector2D<i32>> {
        if self.z != other.z {
            return None;
        }
        Some(
            vec2(other.x as i32, other.y as i32) - vec2(self.x as i32, self.y as i32),
        )
    }

    /// Hex distance from this location to the other one, if applicable.
    pub fn distance_from(&self, other: Location) -> Option<i32> {
        if let Some(v) = self.v2_at(other) {
            Some(v.hex_dist())
        } else {
            None
        }
    }

    /// Distance that defaults to max integer value for separate zones.
    ///
    /// Can be used for situations that want a straightforward metric function like A* search.
    pub fn metric_distance(&self, other: Location) -> i32 {
        self.distance_from(other).unwrap_or(i32::max_value())
    }

    pub fn dir6_towards(&self, other: Location) -> Option<Dir6> {
        if let Some(v) = self.v2_at(other) {
            Some(Dir6::from_v2(v))
        } else {
            None
        }
    }

    /// A pseudorandom value in [-1.0, 1.0] corresponding to this specific location.
    ///
    /// Is always the same for the same `Location`.
    pub fn noise(&self) -> f32 { noise(self.x as i32 + self.y as i32 * 59 + self.z as i32 * 919) }

    /// Offset location and follow any portals in target site.
    pub fn jump<T: TerrainQuery, V: Into<Vector2D<i32>> + Sized>(
        self,
        ctx: &T,
        offset: V,
    ) -> Location {
        let loc = self + offset.into();
        ctx.portal(loc).unwrap_or(loc)
    }

    /// Return `Sector` this location is in.
    pub fn sector(self) -> Sector {
        let (u, v) = self.to_rect_coords();

        Sector::new(
            (u as f32 / SECTOR_WIDTH as f32).floor() as i8,
            (v as f32 / SECTOR_HEIGHT as f32).floor() as i8,
            self.z,
        )
    }

    /// Map location's x, y to rectangular (offset) coordinates.
    pub fn to_rect_coords(self) -> (i32, i32) {
        let u = self.x as i32 - self.y as i32;
        let v = ((self.x as f32 + self.y as f32) / 2.0).floor() as i32;
        (u, v)
    }

    pub fn from_rect_coords(u: i32, v: i32, z: i8) -> Location {
        // Yeah I don't know either how you're supposed to come up with the right ceil/floor
        // juggling, just tweaked it around until it passed all the unit tests.
        let half_u = u as f32 / 2.0;
        Location::new(
            (half_u.ceil() as i32 + v) as i8,
            (v - half_u.floor() as i32) as i8,
            z,
        )
    }
}

impl<V: Into<Vector2D<i32>>> Add<V> for Location {
    type Output = Location;
    fn add(self, other: V) -> Location {
        let other = other.into();
        Location {
            x: (self.x as i32 + other.x) as i8,
            y: (self.y as i32 + other.y) as i8,
            z: self.z,
        }
    }
}

impl Add<Portal> for Location {
    type Output = Location;
    fn add(self, other: Portal) -> Location {
        Location {
            x: (Wrapping(self.x) + Wrapping(other.dx)).0,
            y: (Wrapping(self.y) + Wrapping(other.dy)).0,
            z: other.z,
        }
    }
}

impl<V: Into<Vector2D<i32>>> Sub<V> for Location {
    type Output = Location;
    fn sub(self, other: V) -> Location {
        let other = other.into();
        Location {
            x: (self.x as i32 - other.x) as i8,
            y: (self.y as i32 - other.y) as i8,
            z: self.z,
        }
    }
}

impl GridNode for Location {
    fn neighbors(&self) -> Vec<Location> { Dir6::iter().map(|d| *self + d.to_v2()).collect() }
}

#[derive(Copy, Eq, PartialEq, Clone, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Portal {
    pub dx: i8,
    pub dy: i8,
    pub z: i8,
}

impl Portal {
    pub fn new(from: Location, to: Location) -> Portal {
        Portal {
            dx: (Wrapping(to.x) - Wrapping(from.x)).0,
            dy: (Wrapping(to.y) - Wrapping(from.y)).0,
            z: to.z,
        }
    }
}

impl Add<Portal> for Portal {
    type Output = Portal;
    fn add(self, other: Portal) -> Portal {
        Portal {
            dx: (Wrapping(self.dx) + Wrapping(other.dx)).0,
            dy: (Wrapping(self.dy) + Wrapping(other.dy)).0,
            z: other.z,
        }
    }
}

/// Non-scrolling screen.
///
/// A sector represents a rectangular chunk of locations that fit on the visual screen. Sector
/// coordinates form their own sector space that tiles the location space with sectors.
#[derive(Copy, Eq, PartialEq, Clone, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Sector {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Sector {
    pub fn new(x: i8, y: i8, z: i8) -> Sector { Sector { x, y, z } }

    pub fn origin(self) -> Location { self.rect_coord_loc(0, 0) }

    pub fn rect_coord_loc(self, u: i32, v: i32) -> Location {
        Location::from_rect_coords(
            self.x as i32 * SECTOR_WIDTH + u,
            self.y as i32 * SECTOR_HEIGHT + v,
            self.z,
        )
    }

    /// Center location for this sector.
    ///
    /// Usually you want the camera positioned here.
    pub fn center(self) -> Location {
        // XXX: If the width/height are even (as they currently are), there isn't a centered cell.
        self.rect_coord_loc(SECTOR_WIDTH / 2 - 1, SECTOR_HEIGHT / 2 - 1)
    }

    // TODO: Use impl Trait instead of box for return type once it's stable.
    pub fn iter(self) -> Box<Iterator<Item = Location>> {
        let n = SECTOR_WIDTH * SECTOR_HEIGHT;
        let pitch = SECTOR_WIDTH;
        Box::new((0..n).map(
            move |i| self.rect_coord_loc(i % pitch, i / pitch),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::{Location, Sector};
    use euclid::vec2;

    #[test]
    fn test_wraparound() {
        let l1 = Location::new(0, 0, 0);
        let l2 = l1 + vec2(300, 300);
        assert_eq!((44, 44), (l2.x, l2.y));
    }

    #[test]
    fn test_morton() {
        use rand::{self, Rng};
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let x = rng.gen::<u32>() & 0xff_ffff;
            assert_eq!(x, Location::from_morton(x).to_morton());
        }
    }

    #[test]
    fn test_location_to_sector() {
        let s = Sector::new(0, 0, 0);
        assert_eq!(s.origin(), Location::new(0, 0, 0));

        // Sector division near origin
        assert_eq!(Location::new(0, 0, 0).sector(), Sector::new(0, 0, 0));
        assert_eq!(Location::new(-1, -1, 0).sector(), Sector::new(0, -1, 0));
        assert_eq!(Location::new(0, 1, 0).sector(), Sector::new(-1, 0, 0));
        assert_eq!(Location::new(-1, 0, 0).sector(), Sector::new(-1, -1, 0));

        for y in -100..100 {
            for x in -100..100 {
                let loc = Location::new(x, y, 0);
                let (u, v) = loc.to_rect_coords();
                assert_eq!(
                    loc,
                    Location::from_rect_coords(u, v, loc.z),
                    "u: {}, v: {}",
                    u,
                    v
                );

                assert!(
                    loc.sector().iter().find(|&x| x == loc).is_some(),
                    format!("{:?} not found in sector {:?}", loc, loc.sector())
                );
            }
        }
    }

    #[test]
    fn test_sector_iter() {
        let s = Sector::new(0, 0, 0);

        for loc in s.iter() {
            assert_eq!(s, loc.sector(), "Location: {:?}", loc);
        }
    }
}
