use structs::Point;
use traits::HasValues;

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
pub fn distance<T: HasValues>(point: &T, other: &T) -> i32 {

  let base = distance_2d(point, other);
  let point: Point = Point::from(point.values());
  let other: Point = Point::from(other.values());
  let diff: Point = &point - &other;
  let height = diff.t.abs();
  let distance = base + height;

  distance

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
pub fn distance_2d<T: HasValues>(point: &T, other: &T) -> i32 {

  let point: Point = Point::from(point.values());
  let other: Point = Point::from(other.values());
  let diff: Point = &point - &other;
  let (q, r, s) = diff.values_cube_2d();
  let distance = (q.abs() + r.abs() + s.abs()) / 2;

  distance

}

