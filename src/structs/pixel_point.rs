use std::convert::From;
use std::ops::{Add, Sub, Mul};

use structs::{FloatPoint, Point};
use traits::HasValues;

/// Translate 2D QRS coordinates to XY coordinates on a screen
#[derive(Debug, PartialEq)]
pub struct PixelPoint {

  /// Axis X on the screen
  pub x: f32,

  /// Axis Y on the screen
  pub y: f32,

}

impl PixelPoint {

  /// Factory function for making new points
  pub fn new(x: f32, y: f32) -> PixelPoint {
    PixelPoint {x: x, y: y}
  }

}

/// Add one point to another
impl<'a, 'b> Add<&'b PixelPoint> for &'a PixelPoint {

  type Output = PixelPoint;

  fn add(self, rhs: &'b PixelPoint) -> PixelPoint {
    PixelPoint::new(self.x + rhs.x, self.y + rhs.y)
  }

}

/// Subtract one point from another
impl<'a, 'b> Sub<&'b PixelPoint> for &'a PixelPoint {

  type Output = PixelPoint;

  fn sub(self, rhs: &'b PixelPoint) -> PixelPoint {
    PixelPoint::new(self.x - rhs.x, self.y - rhs.y)
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

  fn mul(self, rhs: &'b PixelPoint) -> PixelPoint {
    PixelPoint::new(self.x * rhs.x, self.y * rhs.y)
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
    let result: PixelPoint = PixelPoint::new(x, y);

    result
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let point: PixelPoint = PixelPoint::new(1f32, 2f32);

    assert!(point.x == 1f32);
    assert!(point.y == 2f32);
  }

  #[test]
  fn add() {
    let point: PixelPoint = PixelPoint::new(1f32, 2f32);
    let other: PixelPoint = PixelPoint::new(3f32, 4f32);
    let result: PixelPoint = &point + &other;

    assert!(4f32 == result.x);
    assert!(6f32 == result.y);
  }

  #[test]
  fn sub() {
    let point: PixelPoint = PixelPoint::new(1f32, 2f32);
    let other: PixelPoint = PixelPoint::new(3f32, 4f32);
    let result: PixelPoint = &point - &other;

    assert!(-2f32 == result.x);
    assert!(-2f32 == result.y);
  }

  #[test]
  fn mul() {
    let point: PixelPoint = PixelPoint::new(1280f32, 720f32);
    let other: PixelPoint = PixelPoint::new(1.5f32, 1.5f32);
    let result: PixelPoint = &point * &other;

    assert!(1920f32 == result.x);
    assert!(1080f32 == result.y);
  }

  #[test]
  fn mul_scale_coordinates() {
    let point: Point = Point(1, 2, 5);
    let other: PixelPoint = PixelPoint::from(&point);
    let scale: PixelPoint = PixelPoint::new(5f32, 5f32);
    let result: PixelPoint = &other * &scale;

    assert!(3f32.sqrt() * 10f32 == result.x);
    assert!(15f32 == result.y);
  }

  #[test]
  fn from_point() {
    let point: Point = Point(1, 2, 5);
    let other: PixelPoint = PixelPoint::from(&point);

    assert!(3f32.sqrt() * 2f32 == other.x);
    assert!(3f32 == other.y);
  }

  #[test]
  fn from_float_point() {
    let point: FloatPoint = FloatPoint(1f32, 2f32, 0f32);
    let other: PixelPoint = PixelPoint::from(&point);

    assert!(3f32.sqrt() * 2f32 == other.x);
    assert!(3f32 == other.y);
  }
}
