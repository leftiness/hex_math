use std::ops::{Add, Sub};

/// Basic point on a coordinate plane
///
/// The point contains two coordinates QR to describe its position in
/// two dimensions and a third T to describe its third dimension.
///
/// Q can also be considered X on a cube, and R is Z.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point<T = i32>(pub T, pub T, pub T);

/// Add one point to another
impl<'a, 'b, T> Add<&'b Point<T>> for &'a Point<T>
  where &'a T: Add<&'b T, Output = T> {

  type Output = Point<T>;

  fn add(self, other: &'b Point<T>) -> Point<T> {
    let &Point(ref q0, ref r0, ref t0) = self;
    let &Point(ref q1, ref r1, ref t1) = other;

    Point(q0 + q1, r0 + r1, t0 + t1)
  }
}

/// Subtract one point from another
impl<'a, 'b, T> Sub<&'b Point<T>> for &'a Point<T>
  where &'a T: Sub<&'b T, Output = T> {

  type Output = Point<T>;

  fn sub(self, other: &'b Point<T>) -> Point<T> {
    let &Point(ref q0, ref r0, ref t0) = self;
    let &Point(ref q1, ref r1, ref t1) = other;

    Point(q0 - q1, r0 - r1, t0 - t1)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
