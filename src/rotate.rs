//! Useful stuff for rotating points

use traits::has_values::HasValues;
use point::Point;

/// Rotate the point around a provided center, keeping the same height
///
/// Positive rotations are clockwise. Six rotations bring a point back to the
/// starting position.
///
/// # Example
///
/// ```
/// use hex_math::{Point, rotate_2d};
///
/// let point: Point = Point::new(1, 2, 5);
/// let zero: Point = Point::new(0, 0, 0);
///
/// assert_eq!(rotate_2d(&point, &zero, 1), Point::new(-2, 3, 5));
/// assert_eq!(rotate_2d(&point, &zero, 2), Point::new(-3, 1, 5));
/// assert_eq!(rotate_2d(&point, &zero, 3), Point::new(-1, -2, 5));
/// assert_eq!(rotate_2d(&point, &zero, 4), Point::new(2, -3, 5));
/// assert_eq!(rotate_2d(&point, &zero, 5), Point::new(3, -1, 5));
/// assert_eq!(rotate_2d(&point, &zero, 6), point);
/// assert_eq!(rotate_2d(&point, &zero, -1), Point::new(3, -1, 5));
/// assert_eq!(rotate_2d(&point, &zero, -2), Point::new(2, -3, 5));
/// assert_eq!(rotate_2d(&point, &zero, -3), Point::new(-1, -2, 5));
/// assert_eq!(rotate_2d(&point, &zero, -4), Point::new(-3, 1, 5));
/// assert_eq!(rotate_2d(&point, &zero, -5), Point::new(-2, 3, 5));
/// assert_eq!(rotate_2d(&point, &zero, -6), point);
/// assert_eq!(rotate_2d(&point, &zero, -12), point);
/// ```
///
/// ```
/// use hex_math::{Point, rotate_2d};
///
/// let point: Point = Point::new(1, 2, 5);
/// let center: Point = Point::new(1, 1, 5);
///
/// assert_eq!(rotate_2d(&point, &center, 2), Point::new(0, 1, 5));
/// ```
pub fn rotate_2d<T: HasValues>(point: &T, center: &T, mut times: i32) -> Point {

  let point: Point = point.to_point();
  let center: Point = center.to_point();

  if &point == &center {
    return point;
  }

  let relative_point: Point = &point - &center;
  let (q, r, s, t) = relative_point.values_cube();

  times %= 6;

  if times < 0 {
    times += 6;
  }

  let rotated_point: Point = match times {
    0 => relative_point,
    1 => Point::new(-r, -s, t),
    2 => Point::new(s, q, t),
    3 => Point::new(-q, -r, t),
    4 => Point::new(r, s, t),
    5 => Point::new(-s, -q, t),
    _ => panic!(),
  };

  let result: Point = &rotated_point + &center;

  result

}

