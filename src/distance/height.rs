use std::borrow::Borrow;

use structs::Point;

/// Calculate the height distance between two points.
pub fn height<T: Borrow<Point>>(point: &T, other: &T) -> i32 {
  let &Point(_, _, t0) = point.borrow();
  let &Point(_, _, t1) = other.borrow();

  (t0 - t1).abs()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn height() {
    assert!(1 == super::height(&Point(1, 2, 5), &Point(1, 2, 6)));
  }
}
