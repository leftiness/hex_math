use std::convert::From;
use std::ops::{Add, Sub};

use traits::HasValues;

/// Basic point on a coordinate plane
///
/// The point contains two coordinates QR to describe its position in
/// two dimensions and a third T to describe its third dimension.
#[derive(Debug, Eq, Hash, PartialEq)]
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
  pub fn new(q: i32, r: i32, t: i32) -> Point {
    Point {q: q, r: r, t: t}
  }

  /// Convenience function for making two-dimensional points
  pub fn new_2d(q: i32, r: i32) -> Point {
    Point::new(q, r, 0)
  }

}

/// Add one point to another
impl<'a, 'b> Add<&'b Point> for &'a Point {

  type Output = Point;

  fn add(self, rhs: &'b Point) -> Point {
    Point::new(self.q + rhs.q, self.r + rhs.r, self.t + rhs.t)
  }

}

/// Subtract one point from another
impl<'a, 'b> Sub<&'b Point> for &'a Point {

  type Output = Point;

  fn sub(self, rhs: &'b Point) -> Point {
    Point::new(self.q - rhs.q, self.r - rhs.r, self.t - rhs.t)
  }

}

/// Access the point's coordinate values
impl HasValues for Point {

  fn values(&self) -> (i32, i32, i32) {
    (self.q, self.r, self.t)
  }

}

/// Conveniently convert a values tuple into a point
impl From<(i32, i32, i32)> for Point {

  fn from((q, r, t): (i32, i32, i32)) -> Point {
    Point::new(q, r, t)
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let point: Point = Point::new(1, 2, 5);

    assert!(1 == point.q);
    assert!(2 == point.r);
    assert!(5 == point.t);
  }

  #[test]
  fn new_2d() {
    let point: Point = Point::new_2d(1, 2);

    assert!(1 == point.q);
    assert!(2 == point.r);
    assert!(0 == point.t);
  }

  #[test]
  fn add() {
    let point: Point = Point::new(1, 2, 5);
    let other: Point = Point::new(3, 4, 10);
    let result: Point = &point + &other;

    assert!(4 == result.q);
    assert!(6 == result.r);
    assert!(15 == result.t);
  }

  #[test]
  fn sub() {
    let point: Point = Point::new(1, 2, 5);
    let other: Point = Point::new(3, 4, 10);
    let result: Point = &point - &other;

    assert!(-2 == result.q);
    assert!(-2 == result.r);
    assert!(-5 == result.t);
  }

  #[test]
  fn values() {
    let (q, r, t) = Point::new(1, 2, 5).values();

    assert!(1 == q);
    assert!(2 == r);
    assert!(5 == t);
  }

  #[test]
  fn from() {
    let point: Point = Point::from((1, 2, 5));

    assert!(1 == point.q);
    assert!(2 == point.r);
    assert!(5 == point.t);
  }
}
