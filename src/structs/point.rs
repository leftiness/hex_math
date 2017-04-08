use std::convert::From;
use std::ops::{Add, Sub};

use traits::HasValues;

/// Basic point on a coordinate plane
///
/// The point contains two coordinates QR to describe its position in
/// two dimensions and a third T to describe its third dimension.
///
/// Q can also be considered X on a cube, and R is Z.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Point(pub i32, pub i32, pub i32);

/// Add one point to another
impl<'a, 'b> Add<&'b Point> for &'a Point {
  type Output = Point;

  fn add(self, other: &'b Point) -> Point {
    let &Point(q0, r0, t0) = self;
    let &Point(q1, r1, t1) = other;

    Point(q0 + q1, r0 + r1, t0 + t1)
  }
}

/// Subtract one point from another
impl<'a, 'b> Sub<&'b Point> for &'a Point {
  type Output = Point;

  fn sub(self, other: &'b Point) -> Point {
    let &Point(q0, r0, t0) = self;
    let &Point(q1, r1, t1) = other;

    Point(q0 - q1, r0 - r1, t0 - t1)
  }
}

/// Access the point's coordinate values
impl HasValues for Point {
  fn values(&self) -> (i32, i32, i32) {
    let &Point(q, r, t) = self;

    (q, r, t)
  }
}

/// Conveniently convert a values tuple into a point
impl From<(i32, i32, i32)> for Point {
  fn from((q, r, t): (i32, i32, i32)) -> Point {
    Point(q, r, t)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let Point(q, r, t) = Point(1, 2, 5);

    assert!(1 == q);
    assert!(2 == r);
    assert!(5 == t);
  }

  #[test]
  fn add() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(3, 4, 10);
    let Point(q, r, t) = &point + &other;

    assert!(4 == q);
    assert!(6 == r);
    assert!(15 == t);
  }

  #[test]
  fn sub() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(3, 4, 10);
    let Point(q, r, t)= &point - &other;

    assert!(-2 == q);
    assert!(-2 == r);
    assert!(-5 == t);
  }

  #[test]
  fn values() {
    let (q, r, t) = Point(1, 2, 5).values();

    assert!(1 == q);
    assert!(2 == r);
    assert!(5 == t);
  }

  #[test]
  fn from() {
    let Point(q, r, t) = Point::from((1, 2, 5));

    assert!(1 == q);
    assert!(2 == r);
    assert!(5 == t);
  }
}
