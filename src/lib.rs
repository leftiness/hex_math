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

mod point;
mod prism;
mod travel;
mod distance;
mod line;
mod range;
mod rotate;
mod ring;

pub use point::Point;
pub use prism::Prism;
pub use travel::{Direction, travel};
pub use distance::{distance, distance_2d};
pub use line::{line, line_through, ray, ray_through};
pub use range::{range, range_2d, flood, flood_2d};
pub use rotate::rotate;
pub use ring::{ring, ring_2d};

