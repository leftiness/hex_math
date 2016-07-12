//! Basic point on a coordinate plane

use std::ops::{Add, Sub};

/// Basic point on a coordinate plane
///
/// The point contains three coordinates (QRS) to describe its position in
/// two dimensions and a fourth (T) to describe its third dimension.
/// The first three are "cube" coordinates as they describe a 2D hexagon as
/// if it were a cube in 3D space, making several algorithms easier to use.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {

  /// This can also be considered axis X on a cube.
  pub q: i32,

  /// This can also be considered axis Z on a cube.
  pub r: i32,

  /// This can also be considered axis Y on a cube.
  pub s: i32,

  /// This is the height of the point in 3D space.
  pub t: i32,
}

impl Point {

  /// Factory function for making new points
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// ```
  pub fn new(q: i32, r: i32, t: i32) -> Point {
    Point {q: q, r: r, s: -q - r, t: t}
  }

  /// Convenience function for making two-dimensional points
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new_2d(1, 2);
  /// ```
  pub fn new_2d(q: i32, r: i32) -> Point {
    Point::new(q, r, 0)
  }

  /// Convenient getter for the point's axial coordinate values
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  ///
  /// assert_eq!((1, 2, 5), spot.values());
  /// ```
  ///
  /// Those using two-dimensional points may simply ignore a value.
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new_2d(1, 2);
  /// let (q, r, _) = spot.values();
  ///
  /// assert_eq!((1, 2), (q, r));
  /// ```
  pub fn values(&self) -> (i32, i32, i32) {
    (self.q, self.r, self.t)
  }

  /// Convenient getter for the point's cube coordinate values
  ///
  /// # Exampes
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  ///
  /// assert_eq!((1, 2, -3, 5), spot.values_cube());
  /// ```
  pub fn values_cube(&self) -> (i32, i32, i32, i32) {
    (self.q, self.r, self.s, self.t)
  }

}

/// Add one point to another
///
/// # Example
///
/// ```
/// # use hex_math::point::Point;
/// #
/// let spot: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let result: Point = spot + other;
///
/// assert_eq!((4, 6, 15), result.values());
/// ```
impl Add for Point {

  type Output = Point;

  fn add(self, rhs: Point) -> Point {
    Point::new(self.q + rhs.q, self.r + rhs.r, self.t + rhs.t)
  }

}

/// Subtract one point from another
///
/// # Example
///
/// ```
/// # use hex_math::point::Point;
/// #
/// let spot: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let result: Point = spot - other;
///
/// assert_eq!((-2, -2, -5), result.values());
/// ```
impl Sub for Point {

  type Output = Point;

  fn sub(self, rhs: Point) -> Point {
    Point::new(self.q - rhs.q, self.r - rhs.r, self.t - rhs.t)
  }

}

