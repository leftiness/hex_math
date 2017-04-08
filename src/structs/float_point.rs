use std::convert::From;
use std::ops::{Add, Sub};

use structs::Point;
use traits::HasValues;

/// Point on a coordinate plane with floating point coordinate values
#[derive(Debug, PartialEq)]
pub struct FloatPoint {

  /// This can also be considered axis X on a cube.
  pub q: f32,

  /// This can also be considered axis Z on a cube.
  pub r: f32,

  /// This is the height of the point in 3D space.
  pub t: f32,

}

impl FloatPoint {

  /// Factory function for making new points
  pub fn new(q: f32, r: f32, t: f32) -> FloatPoint {
    FloatPoint {q: q, r: r, t: t}
  }

  /// Convenience function for making two-dimensional points
  pub fn new_2d(q: f32, r: f32) -> FloatPoint {
    FloatPoint::new(q, r, 0f32)
  }

  /// Round a float point back to a standard point
  pub fn round(&self) -> Point {
    let (q, r, s, t) = self.values_cube();

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

    let point: Point = Point(rq as i32, rr as i32, rt as i32);

    point
  }

}

/// Add one point to another
impl<'a, 'b> Add<&'b FloatPoint> for &'a FloatPoint {

  type Output = FloatPoint;

  fn add(self, rhs: &'b FloatPoint) -> FloatPoint {
    FloatPoint::new(self.q + rhs.q, self.r + rhs.r, self.t + rhs.t)
  }

}

/// Subtract one point from another
impl<'a, 'b> Sub<&'b FloatPoint> for &'a FloatPoint {

  type Output = FloatPoint;

  fn sub(self, rhs: &'b FloatPoint) -> FloatPoint {
    FloatPoint::new(self.q - rhs.q, self.r - rhs.r, self.t - rhs.t)
  }

}

/// Access the point's coordinate values
impl HasValues<f32> for FloatPoint {

  fn values(&self) -> (f32, f32, f32) {
    (self.q, self.r, self.t)
  }

}

/// Conveniently convert a values tuple into a point
impl From<(f32, f32, f32)> for FloatPoint {

  fn from((q, r, t): (f32, f32, f32)) -> FloatPoint {
    FloatPoint::new(q, r, t)
  }

}

/// Conveniently convert a values tuple into a point
impl From<(i32, i32, i32)> for FloatPoint {

  fn from((q, r, t): (i32, i32, i32)) -> FloatPoint {
    FloatPoint::new(q as f32, r as f32, t as f32)
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let point: FloatPoint = FloatPoint::new(1f32, 2f32, 5f32);

    assert!(1f32 == point.q);
    assert!(2f32 == point.r);
    assert!(5f32 == point.t);
  }

  #[test]
  fn new_2d() {
    let point: FloatPoint = FloatPoint::new_2d(1f32, 2f32);

    assert!(1f32 == point.q);
    assert!(2f32 == point.r);
    assert!(0f32 == point.t);
  }

  #[test]
  fn round() {
    let Point(q, r, t) = FloatPoint::new(1.6f32, 1.6f32, 2.5f32).round();

    assert!(2 == q);
    assert!(1 == r);
    assert!(3 == t);
  }

  #[test]
  fn add() {
    let point: FloatPoint = FloatPoint::new(1f32, 2f32, 5f32);
    let other: FloatPoint = FloatPoint::new(3f32, 4f32, 10f32);
    let result: FloatPoint = &point + &other;

    assert!(4f32 == result.q);
    assert!(6f32 == result.r);
    assert!(15f32 == result.t);
  }

  #[test]
  fn sub() {
    let point: FloatPoint = FloatPoint::new(1f32, 2f32, 5f32);
    let other: FloatPoint = FloatPoint::new(3f32, 4f32, 10f32);
    let result: FloatPoint = &point - &other;

    assert!(-2f32 == result.q);
    assert!(-2f32 == result.r);
    assert!(-5f32 == result.t);
  }

  #[test]
  fn values() {
    let (q, r, t) = FloatPoint::new(1f32, 2f32, 5f32).values();

    assert!(1f32 == q);
    assert!(2f32 == r);
    assert!(5f32 == t);
  }

  #[test]
  fn from_f32_tuple() {
    let point: FloatPoint = FloatPoint::from((1f32, 2f32, 5f32));

    assert!(1f32 == point.q);
    assert!(2f32 == point.r);
    assert!(5f32 == point.t);
  }

  #[test]
  fn from_i32_tuple() {
    let point: FloatPoint = FloatPoint::from((1, 2, 5));

    assert!(1f32 == point.q);
    assert!(2f32 == point.r);
    assert!(5f32 == point.t);
  }
}
