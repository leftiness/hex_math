//! Basic point on a coordinate plane

use std::convert::From;
use std::ops::{Add, Sub};

use traits::has_values::HasValues;

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

  /// This is the height of the point in 3D space.
  pub t: i32,

}

impl Point {

  /// Factory function for making new points
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::Point;
  ///
  /// let point: Point = Point::new(1, 2, 5);
  /// ```
  pub fn new(q: i32, r: i32, t: i32) -> Point {
    Point {q: q, r: r, t: t}
  }

  /// Convenience function for making two-dimensional points
  ///
  /// Example
  ///
  /// ```
  /// use hex_math::Point;
  ///
  /// let point: Point = Point::new_2d(1, 2);
  /// ```
  pub fn new_2d(q: i32, r: i32) -> Point {
    Point::new(q, r, 0)
  }

  /// Convenience function for making points from a values tuple
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Point, HasValues};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  /// let other: Point = Point::from_values(point.values());
  ///
  /// assert_eq!((1, 2, 5), other.values());
  /// ```
  pub fn from_values((q, r, t): (i32, i32, i32)) -> Point {
    Point::new(q, r, t)
  }

}

/// Add one point to another
///
/// # Example
///
/// ```
/// use hex_math::{Point, HasValues};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let result: Point = &point + &other;
///
/// assert_eq!((4, 6, 15), result.values());
/// ```
impl<'a, 'b> Add<&'b Point> for &'a Point {

  type Output = Point;

  fn add(self, rhs: &'b Point) -> Point {
    Point::new(self.q + rhs.q, self.r + rhs.r, self.t + rhs.t)
  }

}

/// Subtract one point from another
///
/// # Example
///
/// ```
/// use hex_math::{Point, HasValues};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let result: Point = &point - &other;
///
/// assert_eq!((-2, -2, -5), result.values());
/// ```
impl<'a, 'b> Sub<&'b Point> for &'a Point {

  type Output = Point;

  fn sub(self, rhs: &'b Point) -> Point {
    Point::new(self.q - rhs.q, self.r - rhs.r, self.t - rhs.t)
  }

}

/// Access the point's coordinate values
///
/// # Example
///
/// ```
/// use hex_math::{Point, HasValues};
///
/// let point: Point = Point::new(1, 2, 5);
///
/// assert_eq!((1, 2, 5), point.values());
/// assert_eq!((1, 2, -3, 5), point.values_cube());
/// assert_eq!((1, 2), point.values_2d());
/// assert_eq!((1, 2, -3), point.values_cube_2d());
/// ```
impl HasValues for Point {

  fn values(&self) -> (i32, i32, i32) {
    (self.q, self.r, self.t)
  }

}

/// Conveniently convert a values tuple into a point
///
/// # Example
///
/// ```
/// use hex_math::{Point, HasValues};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = From::from(point.values());
///
/// assert_eq!((1, 2, 5), other.values());
/// ```
impl From<(i32, i32, i32)> for Point {

  fn from((q, r, t): (i32, i32, i32)) -> Point {
    Point::new(q, r, t)
  }

}

