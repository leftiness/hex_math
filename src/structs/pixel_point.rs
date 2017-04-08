use std::convert::From;
use std::ops::{Add, Sub, Mul};

use structs::{FloatPoint, Point};
use traits::HasValues;

/// Translate 2D QRS coordinates to XY coordinates on a screen
#[derive(Debug, PartialEq)]
pub struct PixelPoint(pub f32, pub f32);

/// Add one point to another
impl<'a, 'b> Add<&'b PixelPoint> for &'a PixelPoint {
  type Output = PixelPoint;

  fn add(self, other: &'b PixelPoint) -> PixelPoint {
    let &PixelPoint(x0, y0) = self;
    let &PixelPoint(x1, y1) = other;

    PixelPoint(x0 + x1, y0 + y1)
  }
}

/// Subtract one point from another
impl<'a, 'b> Sub<&'b PixelPoint> for &'a PixelPoint {
  type Output = PixelPoint;

  fn sub(self, other: &'b PixelPoint) -> PixelPoint {
    let &PixelPoint(x0, y0) = self;
    let &PixelPoint(x1, y1) = other;

    PixelPoint(x0 - x1, y0 - y1)
  }
}

/// Multiply one point by another
///
/// This might most obviously be used to scale resolutions. However, consider
/// that pixel points are created from ordinary points at a scale where one
/// side is one pixel in length. Therefore, multiplying a pixel point by five
/// would result in the coordinate values at a scale where one side is five
/// pixels in length.
impl <'a, 'b> Mul<&'b PixelPoint> for &'a PixelPoint {
  type Output = PixelPoint;

  fn mul(self, other: &'b PixelPoint) -> PixelPoint {
    let &PixelPoint(x0, y0) = self;
    let &PixelPoint(x1, y1) = other;

    PixelPoint(x0 * x1, y0 * y1)
  }
}

/// Conveniently convert a point into a pixel point
impl<'a> From<&'a Point> for PixelPoint {
  fn from(point: &'a Point) -> PixelPoint {
    let point: FloatPoint = FloatPoint::from(point.values());
    let result: PixelPoint = PixelPoint::from(&point);

    result
  }
}

/// Conveniently convert a float point into a pixel point
impl <'a> From<&'a FloatPoint> for PixelPoint {
  fn from(point: &'a FloatPoint) -> PixelPoint {
    let &FloatPoint(q, r, _) = point;
    let x: f32 = 3f32.sqrt() * (q + (r / 2f32));
    let y: f32 = 1.5f32 * r;
    let result: PixelPoint = PixelPoint(x, y);

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add() {
    let point: PixelPoint = PixelPoint(1f32, 2f32);
    let other: PixelPoint = PixelPoint(3f32, 4f32);
    let PixelPoint(x, y) = &point + &other;

    assert!(4f32 == x);
    assert!(6f32 == y);
  }

  #[test]
  fn sub() {
    let point: PixelPoint = PixelPoint(1f32, 2f32);
    let other: PixelPoint = PixelPoint(3f32, 4f32);
    let PixelPoint(x, y) = &point - &other;

    assert!(-2f32 == x);
    assert!(-2f32 == y);
  }

  #[test]
  fn mul() {
    let point: PixelPoint = PixelPoint(1280f32, 720f32);
    let other: PixelPoint = PixelPoint(1.5f32, 1.5f32);
    let PixelPoint(x, y) = &point * &other;

    assert!(1920f32 == x);
    assert!(1080f32 == y);
  }

  #[test]
  fn mul_scale_coordinates() {
    let point: Point = Point(1, 2, 5);
    let other: PixelPoint = PixelPoint::from(&point);
    let scale: PixelPoint = PixelPoint(5f32, 5f32);
    let PixelPoint(x, y) = &other * &scale;

    assert!(3f32.sqrt() * 10f32 == x);
    assert!(15f32 == y);
  }

  #[test]
  fn from_point() {
    let point: Point = Point(1, 2, 5);
    let PixelPoint(x, y) = PixelPoint::from(&point);

    assert!(3f32.sqrt() * 2f32 == x);
    assert!(3f32 == y);
  }

  #[test]
  fn from_float_point() {
    let point: FloatPoint = FloatPoint(1f32, 2f32, 0f32);
    let PixelPoint(x, y) = PixelPoint::from(&point);

    assert!(3f32.sqrt() * 2f32 == x);
    assert!(3f32 == y);
  }
}
