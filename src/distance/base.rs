use std::borrow::Borrow;

use structs::{CubePoint, Point};

/// Calculate the manhattan distance between two points ignoring height
///
/// Distance is rounded to the nearest integer.
pub fn base<T: Borrow<Point>>(point: &T, other: &T) -> i32 {
  let diff: Point = point.borrow() - other.borrow();
  let CubePoint(q, r, s, _) = diff.into();
  let distance = (q.abs() + r.abs() + s.abs()) / 2;

  distance
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn base() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(3, 4, 10);

    assert!(4 == super::base(&point, &other));
  }
}
