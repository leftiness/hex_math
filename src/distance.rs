use std::borrow::Borrow;

use structs::Point;

/// Calculate the manhattan distance between two points
///
/// Distance is rounded up to the next integer.
pub fn distance<T: Borrow<Point>>(point: &T, other: &T) -> i32 {
  let base = distance_2d(point, other);
  let &Point(_, _, t0) = point.borrow();
  let &Point(_, _, t1) = other.borrow();
  let height = (t0 - t1).abs();
  let distance = base + height;

  distance
}

/// Calculate the manhattan distance between two points ignoring height
///
/// Distance is rounded up to the next integer.
pub fn distance_2d<T: Borrow<Point>>(point: &T, other: &T) -> i32 {
  let diff: Point = point.borrow() - other.borrow();
  let Point(q, r, _) = diff;
  let s = diff.s();
  let distance = (q.abs() + r.abs() + s.abs()) / 2;

  distance
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn distance() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(3, 4, 10);

    assert!(9 == super::distance(&point, &other));
  }

  #[test]
  fn distance_2d() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(3, 4, 10);

    assert!(4 == super::distance_2d(&point, &other));
  }
}
