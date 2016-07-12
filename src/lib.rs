//! Useful stuff for working with a bunch of hexagons

mod point;
mod travel;
mod distance;
mod line;
mod range;
mod rotate;

pub use point::Point;
pub use travel::{Direction, travel};
pub use distance::{distance, distance_2d};
pub use line::line;
pub use range::{range, range_2d, flood, flood_2d};
pub use rotate::rotate;

