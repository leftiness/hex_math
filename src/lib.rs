//! Useful stuff for working with a bunch of hexagons
//!
//! # Example
//!
//! ```
//! use hex_math::{distance, Point};
//!
//! let point: Point = Point::new(1, 2, 5);
//! let other: Point = Point::new(3, 4, 5);
//! let dist: i32 = distance(&point, &other);
//!
//! assert_eq!(4, dist);
//! ```
//!
//! ```
//! use std::collections::HashSet;
//!
//! use hex_math::{Point, range};
//!
//! let point: Point = Point::new(1, 2, 5);
//! let other: Point = Point::new(3, 4, 5);
//! let set: HashSet<Point> = range(&point, 4);
//!
//! assert!(set.contains(&other));
//! ```

mod travel;
mod distance;
mod line;
mod range;
mod rotate;
mod ring;

mod enums;
mod structs;
mod traits;

pub use travel::travel;
pub use distance::{distance, distance_2d};
pub use line::{line, line_through, ray, ray_through};
pub use range::{range, range_2d, flood, flood_2d};
pub use rotate::rotate_2d;
pub use ring::{ring, ring_2d};

pub use enums::Direction;
pub use structs::{FloatPoint, PixelPoint, Point, Prism};
pub use traits::{HasValues, HasWalls, IsPointMap};

