use std::ops::{Add, Sub};

use structs::CubePoint;

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

/// Convert to a float point
impl From<Point> for Point<f32> {
  fn from(point: Point) -> Point<f32> {
    let Point(q, r, t) = point;

    Point(q as f32, r as f32, t as f32)
  }
}

impl Point<f32> {
  /// Round a float point back to a standard point
  pub fn round(&self) -> Point {
    let CubePoint(q, r, s, t) = CubePoint::from(*self);

    let mut rq = q.round();
    let mut rr = r.round();

    let rs = s.round();
    let rt = t.round();

    let dq = (rq - q).abs();
    let dr = (rr - r).abs();
    let ds = (rs - s).abs();

    if (dq > ds) && (dq > dr) {
      rq = -rs - rr;
    } else if ds < dr {
      rr = -rq - rs;
    }

    Point(rq as i32, rr as i32, rt as i32)
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

  #[test]
  fn from_point() {
    let point = Point(1, 2, 5);
    let Point::<f32>(q, r, t) = point.into();

    assert!(1f32 == q);
    assert!(2f32 == r);
    assert!(5f32 == t);
  }

  #[test]
  fn round() {
    let Point(q, r, t) = Point(1.6f32, 1.6f32, 2.5f32).round();

    assert!(2 == q);
    assert!(1 == r);
    assert!(3 == t);
  }
}
