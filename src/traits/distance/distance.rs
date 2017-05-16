use std::borrow::Borrow;

use structs::Point;
use traits::distance::{Base, Height};

/// Trait wrapping distance implementation
pub trait Distance: Borrow<Point> {
  /// Calculate the manhattan distance between two points including height
  ///
  /// Distance is rounded up to the next integer.
  fn distance<T: Borrow<Point>>(&self, other: &T) -> i32;
}

impl<T> Distance for T where T: Borrow<Point> {
  fn distance<U: Borrow<Point>>(&self, other: &U) -> i32 {
    self.base_distance(other) + self.height(other)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn distance() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(3, 4, 10);

    assert!(9 == point.distance(&other));
  }
}
