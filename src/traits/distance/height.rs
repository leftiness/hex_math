use std::borrow::Borrow;

use structs::Point;

/// Trait wrapping height distance implementation
pub trait Height: Borrow<Point> {
  /// Calculate the height distance between two points.
  fn height<T: Borrow<Point>>(&self, other: &T) -> i32;
}

impl<T> Height for T where T: Borrow<Point> {
  fn height<U: Borrow<Point>>(&self, other: &U) -> i32 {
    let &Point(_, _, t0) = self.borrow();
    let &Point(_, _, t1) = other.borrow();

    (t0 - t1).abs()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn height() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(1, 2, 6);

    assert!(1 == point.height(&other));
  }
}
