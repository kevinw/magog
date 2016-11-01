#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate num;
extern crate rand;
extern crate bincode;
extern crate serde;
extern crate vec_map;
extern crate image;
extern crate euclid;
#[macro_use]
extern crate lazy_static;
extern crate vitral;
extern crate vitral_atlas;
extern crate calx_alg;
extern crate calx_grid;
extern crate calx_color;
#[macro_use]
extern crate calx_ecs;
#[macro_use]
extern crate calx_resource;

use std::collections::HashSet;
use euclid::{Point2D, Rect, Size2D};

mod ability;

mod brush;
pub use brush::{Brush, BrushBuilder, Color, Frame, ImageRef, Splat};

mod command;
pub use command::Command;

mod components;
mod field;
mod flags;
mod fov;
mod item;

mod location;
pub use location::{Location, Portal};

mod location_set;
mod mutate;

mod query;
pub use query::Query;

mod spatial;
mod stats;

mod terraform;
pub use terraform::Terraform;

pub mod terrain;

mod world;
pub use world::World;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FovStatus {
    Seen,
    Remembered,
}

/// Return whether the given chart point is on the currently visible screen.
///
/// It is assumed that the chart point 0, 0 is at the center of the screen.
///
/// Since various bits of game logic are tied to the screen boundaries, the screen size is fixed as
/// a constant.
pub fn on_screen(chart_pos: Point2D<i32>) -> bool {
    const W: i32 = 39;
    const H: i32 = 22;
    let (x, y) = (chart_pos.x, chart_pos.y);

    x <= y + (W + 1) / 2 // east
        && x >= y - (W - 1) / 2 // west
        && x >= -H - y // north
        && x <= H - 1 - y // south
}

pub fn onscreen_locations() -> &'static HashSet<Point2D<i32>> {
    lazy_static! {
        static ref ONSCREEN_LOCATIONS: HashSet<Point2D<i32>> = {
            let mut m = HashSet::new();

            // XXX: Hardcoded limits, tied to W and H in on-screen but expressed differently here.
            for y in -20..21 {
                for x in -20..21 {
                    let point = Point2D::new(x, y);
                    if on_screen(point) {
                        m.insert(point);
                    }
                }
            }
            m
        };
    }

    &*ONSCREEN_LOCATIONS
}
