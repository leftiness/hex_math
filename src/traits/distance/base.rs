use std::borrow::Borrow;

use structs::Point;

/// Trait wrapping base distance implementation
pub trait Base: Borrow<Point> {
  /// Calculate the manhattan distance between two points ignoring height
  ///
  /// Distance is rounded to the nearest integer.
  fn base_distance<T: Borrow<Point>>(&self, other: &T) -> i32;
}

impl<T> Base for T where T: Borrow<Point> {
  fn base_distance<U: Borrow<Point>>(&self, other: &U) -> i32 {
    let diff: Point = self.borrow() - other.borrow();
    let distance = (diff.q().abs() + diff.r().abs() + diff.s().abs()) / 2;

    distance
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn base_distance() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(3, 4, 10);

    assert!(4 == point.base_distance(&other));
  }
}
