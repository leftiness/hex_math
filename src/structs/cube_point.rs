use std::ops::{Neg, Sub};

use structs::{Point, FloatPoint};

/// Point on a coordinate plane including calculated S coordinate
///
/// The S coordinate is like the Y axis on a cube.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubePoint<T: Neg + Sub>(pub T, pub T, pub T, pub T);

impl From<Point> for CubePoint<i32> {
  /// Convert from a Point to a CubePoint
  fn from(point: Point) -> Self {
    let Point(q, r, t) = point;

    CubePoint(q, r, -q - r, t)
  }
}

impl From<FloatPoint> for CubePoint<f32> {
  /// Convert from a FloatPoint to a CubePoint
  fn from(point: FloatPoint) -> Self {
    let FloatPoint(q, r, t) = point;

    CubePoint(q, r, -q - r, t)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_point() {
    let point = Point(1, 2, 5);
    let CubePoint(q, r, s, t) = point.into();

    assert!( 1 == q);
    assert!( 2 == r);
    assert!(-3 == s);
    assert!( 5 == t);
  }

  #[test]
  fn from_float_point() {
    let point = FloatPoint(1f32, 2f32, 5f32);
    let CubePoint(q, r, s, t) = point.into();

    assert!( 1f32 == q);
    assert!( 2f32 == r);
    assert!(-3f32 == s);
    assert!( 5f32 == t);
  }
}
