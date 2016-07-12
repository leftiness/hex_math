//! Useful stuff for rotating points

use point::Point;

/// Rotate the point a specified number of times
///
/// Positive rotations are clockwise.
///
/// # Example
///
/// ```
/// use hex_math::Point;
/// use hex_math::rotate;
///
/// let point: Point = Point::new(1, 2, 5);
///
/// assert_eq!(rotate(&point, 1), Point::new(-2, 3, 5));
/// assert_eq!(rotate(&point, 2), Point::new(-3, 1, 5));
/// assert_eq!(rotate(&point, 3), Point::new(-1, -2, 5));
/// assert_eq!(rotate(&point, 4), Point::new(2, -3, 5));
/// assert_eq!(rotate(&point, 5), Point::new(3, -1, 5));
/// assert_eq!(rotate(&point, 6), point);
/// assert_eq!(rotate(&point, -1), Point::new(3, -1, 5));
/// assert_eq!(rotate(&point, -2), Point::new(2, -3, 5));
/// assert_eq!(rotate(&point, -3), Point::new(-1, -2, 5));
/// assert_eq!(rotate(&point, -4), Point::new(-3, 1, 5));
/// assert_eq!(rotate(&point, -5), Point::new(-2, 3, 5));
/// assert_eq!(rotate(&point, -6), point);
/// assert_eq!(rotate(&point, -12), point);
/// ```
pub fn rotate(point: &Point, mut times: i32) -> Point {

  let (q, r, s, t) = point.values_cube();

  times %= 6;

  if times < 0 {
    times += 6;
  }

  return match times {
    1 => Point::new(-r, -s, t),
    2 => Point::new(s, q, t),
    3 => Point::new(-q, -r, t),
    4 => Point::new(r, s, t),
    5 => Point::new(-s, -q, t),
    _ => point.clone(),
  }

}

