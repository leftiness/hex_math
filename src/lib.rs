//! Useful stuff for working with a bunch of hexagons
//!
//! # Examples
//!
//! Direction functions are chainable.
//!
//! ```
//! # use hex_math::point::Point;
//!
//! let spot: Point = Point::new(1, 2, 5);
//! let other: Point = spot.northwest(5).west(2).down(2);
//!
//! assert_eq!((-1, -3, 3), other.values());
//! ```

pub mod point;

