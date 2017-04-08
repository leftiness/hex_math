use structs::Point;
use traits::HasValues;

/// Calculate the manhattan distance between two points
///
/// Distance is rounded up to the next integer.
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
pub fn distance_2d<T: HasValues>(point: &T, other: &T) -> i32 {

  let point: Point = Point::from(point.values());
  let other: Point = Point::from(other.values());
  let diff: Point = &point - &other;
  let (q, r, s) = diff.values_cube_2d();
  let distance = (q.abs() + r.abs() + s.abs()) / 2;

  distance

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn distance() {
    let point: Point = Point::new(1, 2, 5);
    let other: Point = Point::new(3, 4, 10);

    assert!(9 == super::distance(&point, &other));
  }

  #[test]
  fn distance_2d() {
    let point: Point = Point::new(1, 2, 5);
    let other: Point = Point::new(3, 4, 10);

    assert!(4 == super::distance_2d(&point, &other));
  }
}
