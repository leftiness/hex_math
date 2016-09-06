//! Useful stuff for calculating the distance between points

use traits::has_values::HasValues;
use point::Point;

/// Calculate the manhattan distance between two points
///
/// Distance is rounded up to the next integer.
///
/// # Example
///
/// ```
/// use hex_math::{distance, Point};
///
/// let point: Point = Point::new_2d(1, 2);
/// let other: Point = Point::new_2d(3, 4);
///
/// assert_eq!(4, distance(&point, &other));
/// ```
///
/// ```
/// use hex_math::{distance, Point};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
///
/// assert_eq!(9, distance(&point, &other));
/// ```
pub fn distance(point: &Point, other: &Point) -> i32 {
  let point = point.clone();
  let other = other.clone();
  let diff: Point = point - other;
  let base = distance_2d(&point, &other);
  let height = diff.t.abs();

  base + height
}

/// Calculate the manhattan distance between two points ignoring height
///
/// Distance is rounded up to the next integer.
///
/// # Example
///
/// ```
/// use hex_math::{distance_2d, Point};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
///
/// assert_eq!(4, distance_2d(&point, &other));
/// ```
pub fn distance_2d(point: &Point, other: &Point) -> i32 {
  let point = point.clone();
  let other = other.clone();
  let diff: Point = point - other;
  let (q, r, s) = diff.values_cube_2d();
  let distance = (q.abs() + r.abs() + s.abs()) / 2;

  distance
}

