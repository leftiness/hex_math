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
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::FloatPoint;
  ///
  /// let point: FloatPoint = FloatPoint::new(1f32, 2f32, 5f32);
  /// ```
  pub fn new(q: f32, r: f32, t: f32) -> FloatPoint {
    FloatPoint {q: q, r: r, t: t}
  }

  /// Convenience function for making two-dimensional points
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::FloatPoint;
  ///
  /// let point: FloatPoint = FloatPoint::new_2d(1f32, 2f32);
  /// ```
  pub fn new_2d(q: f32, r: f32) -> FloatPoint {
    FloatPoint::new(q, r, 0f32)
  }

  /// Round a float point back to a standard point
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Point, FloatPoint};
  ///
  /// let float: FloatPoint = FloatPoint::new(1.6f32, 1.6f32, 2.5f32);
  /// let point: Point = float.round();
  ///
  /// assert_eq!(point, Point::new(2, 1, 3));
  /// ```
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

    let point: Point = Point::new(rq as i32, rr as i32, rt as i32);

    point

  }

}

/// Add one point to another
///
/// # Example
///
/// ```
/// use hex_math::{FloatPoint, HasValues};
///
/// let point: FloatPoint = FloatPoint::new(1f32, 2f32, 5f32);
/// let other: FloatPoint = FloatPoint::new(3f32, 4f32, 10f32);
/// let result: FloatPoint = &point + &other;
///
/// assert_eq!((4f32, 6f32, 15f32), result.values());
/// ```
impl<'a, 'b> Add<&'b FloatPoint> for &'a FloatPoint {

  type Output = FloatPoint;

  fn add(self, rhs: &'b FloatPoint) -> FloatPoint {
    FloatPoint::new(self.q + rhs.q, self.r + rhs.r, self.t + rhs.t)
  }

}

/// Subtract one point from another
///
/// # Example
///
/// ```
/// use hex_math::{FloatPoint, HasValues};
///
/// let point: FloatPoint = FloatPoint::new(1f32, 2f32, 5f32);
/// let other: FloatPoint = FloatPoint::new(3f32, 4f32, 10f32);
/// let result: FloatPoint = &point - &other;
///
/// assert_eq!((-2f32, -2f32, -5f32), result.values());
/// ```
impl<'a, 'b> Sub<&'b FloatPoint> for &'a FloatPoint {

  type Output = FloatPoint;

  fn sub(self, rhs: &'b FloatPoint) -> FloatPoint {
    FloatPoint::new(self.q - rhs.q, self.r - rhs.r, self.t - rhs.t)
  }

}

/// Access the point's coordinate values
///
/// # Example
///
/// ```
/// use hex_math::{FloatPoint, HasValues};
///
/// let point: FloatPoint = FloatPoint::new(1f32, 2f32, 5f32);
///
/// assert_eq!((1f32, 2f32, 5f32), point.values());
/// ```
impl HasValues<f32> for FloatPoint {

  fn values(&self) -> (f32, f32, f32) {
    (self.q, self.r, self.t)
  }

}

/// Conveniently convert a values tuple into a point
///
/// # Example
///
/// ```
/// use hex_math::{FloatPoint, HasValues};
///
/// let point: FloatPoint = FloatPoint::new(1f32, 2f32, 5f32);
/// let other: FloatPoint = FloatPoint::from(point.values());
///
/// assert_eq!((1f32, 2f32, 5f32), other.values());
/// ```
impl From<(f32, f32, f32)> for FloatPoint {

  fn from((q, r, t): (f32, f32, f32)) -> FloatPoint {
    FloatPoint::new(q, r, t)
  }

}

/// Conveniently convert a values tuple into a point
///
/// # Example
///
/// ```
/// use hex_math::{Point, FloatPoint, HasValues};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: FloatPoint = FloatPoint::from(point.values());
///
/// assert_eq!((1f32, 2f32, 5f32), other.values());
/// ```
impl From<(i32, i32, i32)> for FloatPoint {

  fn from((q, r, t): (i32, i32, i32)) -> FloatPoint {
    FloatPoint::new(q as f32, r as f32, t as f32)
  }

}

