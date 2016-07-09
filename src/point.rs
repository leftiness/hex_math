//! Basic point on a coordinate plane

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
  /// Those using two-dimensional points may simply ignore a value.
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

