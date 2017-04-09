use std::borrow::Borrow;

use distance::{base, height};
use structs::Point;

/// Calculate the manhattan distance between two points including height
///
/// Distance is rounded up to the next integer.
pub fn with_height<T: Borrow<Point>>(point: &T, other: &T) -> i32 {
  base(point, other) + height(point, other)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn distance() {
    assert!(9 == super::with_height(&Point(1, 2, 5), &Point(3, 4, 10)));
  }
}
