use std::ops::{Add, Sub};

/// Basic point on a coordinate plane
///
/// The point contains two coordinates QR to describe its position in
/// two dimensions and a third T to describe its third dimension.
///
/// Q can also be considered X on a cube, and R is Z.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point(pub i32, pub i32, pub i32);

impl Point {
  /// Calculate the S coordinate.
  ///
  /// S represents the Y axis on a cube.
  pub fn s(&self) -> i32 {
    let &Point(q, r, _) = self;
    let s = -q - r;

    s
  }
}

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn s() {
    let point: Point = Point(1, 2, 5);

    assert!(-3 == point.s())
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
}
