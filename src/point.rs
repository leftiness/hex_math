//! Basic point on a coordinate plane
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

use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
  pub q: i32,
  pub r: i32,
  pub s: i32,
  pub t: i32,
}

impl Point {

  /// Factory function for making new points
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// ```
  pub fn new(q: i32, r: i32, t: i32) -> Point {
    Point {q: q, r: r, s: -q - r, t: t}
  }

  /// Convenience function for making two-dimensional points
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new_2d(1, 2);
  /// ```
  pub fn new_2d(q: i32, r: i32) -> Point {
    Point::new(q, r, 0)
  }

  /// Convenient getter for the point's axial coordinate values
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  ///
  /// assert_eq!((1, 2, 5), spot.values());
  /// ```
  ///
  /// Those using two-dimensional points may simply ignore a value.
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new_2d(1, 2);
  /// let (q, r, _) = spot.values();
  ///
  /// assert_eq!((1, 2), (q, r));
  /// ```
  pub fn values(&self) -> (i32, i32, i32) {
    (self.q, self.r, self.t)
  }

  /// Convenient getter for the point's cuboid coordinate values
  ///
  /// # Exampes
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  ///
  /// assert_eq!((1, 2, -3, 5), spot.values_cuboid());
  /// ```
  pub fn values_cuboid(&self) -> (i32, i32, i32, i32) {
    (self.q, self.r, self.s, self.t)
  }

  /// Create a point which is relatively northwest a specified number of units
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.northwest(2);
  ///
  /// assert_eq!((1, 0, 5), other.values());
  /// ```
  pub fn northwest(&self, units: i32) -> Point {
    Point::new(self.q, self.r - units, self.t)
  }

  /// Create a point which is relatively west a specified number of units
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.west(2);
  ///
  /// assert_eq!((-1, 2, 5), other.values());
  /// ```
  pub fn west(&self, units: i32) -> Point {
    Point::new(self.q - units, self.r, self.t)
  }

  /// Create a point which is relatively southwest a specified number of units
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.southwest(2);
  ///
  /// assert_eq!((-1, 4, 5), other.values());
  /// ```
  pub fn southwest(&self, units: i32) -> Point {
    Point::new(self.q - units, self.r + units, self.t)
  }

  /// Create a point which is relatively southeast a specified number of units
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.southeast(2);
  ///
  /// assert_eq!((1, 4, 5), other.values());
  /// ```
  pub fn southeast(&self, units: i32) -> Point {
    Point::new(self.q, self.r + units, self.t)
  }

  /// Create a point which is relatively east a specified number of units
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.east(2);
  ///
  /// assert_eq!((3, 2, 5), other.values());
  /// ```
  pub fn east(&self, units: i32) -> Point {
    Point::new(self.q + units, self.r, self.t)
  }

  /// Create a point which is relatively northeast a specified number of units
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.northeast(2);
  ///
  /// assert_eq!((3, 0, 5), other.values());
  /// ```
  pub fn northeast(&self, units: i32) -> Point {
    Point::new(self.q + units, self.r - units, self.t)
  }

  /// Create a point which is relatively up a specified number of units
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.up(2);
  ///
  /// assert_eq!((1, 2, 7), other.values());
  /// ```
  pub fn up(&self, units: i32) -> Point {
    Point::new(self.q, self.r, self.t + units)
  }

  /// Create a point which is relatively down a specified number of units
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.down(2);
  ///
  /// assert_eq!((1, 2, 3), other.values());
  /// ```
  pub fn down(&self, units: i32) -> Point {
    Point::new(self.q, self.r, self.t - units)
  }

  /// Calculate the distance measured in hexes between two points
  ///
  /// Distance is rounded up to the next integer.
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new_2d(1, 2);
  /// let other: Point = Point::new_2d(3, 4);
  ///
  /// assert_eq!(4, spot.distance_to(other));
  /// ```
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = Point::new(3, 4, 10);
  ///
  /// assert_eq!(7, spot.distance_to(other));
  /// ```
  pub fn distance_to(self, other: Point) -> i32 {
    let diff: Point = self - other;
    let base = (diff.q.abs() + diff.r.abs() + diff.s.abs()) / 2;
    let height = diff.t.abs();

    if height == 0 {
      return base;
    }

    let hypot: f64 = (base.pow(2) + height.pow(2)) as f64;

    hypot.sqrt().ceil() as i32
  }

}

impl Add for Point {

  type Output = Point;

  /// Add one point to another
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = Point::new(3, 4, 5);
  /// let result: Point = spot + other;
  ///
  /// assert_eq!((4, 6, 10), result.values());
  /// ```
  fn add(self, rhs: Point) -> Point {
    Point::new(self.q + rhs.q, self.r + rhs.r, self.t + rhs.t)
  }

}

impl Sub for Point {

  type Output = Point;

  /// Subtract one point from another
  ///
  /// # Examples
  ///
  /// ```
  /// # use hex_math::point::Point;
  ///
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = Point::new(3, 4, 5);
  /// let result: Point = spot - other;
  ///
  /// assert_eq!((-2, -2, 0), result.values());
  /// ```
  fn sub(self, rhs: Point) -> Point {
    Point::new(self.q - rhs.q, self.r - rhs.r, self.t - rhs.t)
  }

}

