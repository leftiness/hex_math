use std::borrow::Borrow;
use std::collections::HashSet;

use fns::line::denumerate;
use structs::Point;
use structs::line::Iterator;
use structs::line::predicate::Range;
use traits::distance::Distance;

/// Trait wrapping line implementation
pub trait Line: Borrow<Point> {
  /// Find the points in a line between the current point and the one provided
  fn line<T: Borrow<Point>>(&self, other: &T) -> HashSet<Point>;
}

impl<T> Line for T where T: Borrow<Point> {
  fn line<U: Borrow<Point>>(&self, other: &U) -> HashSet<Point> {
    Iterator::new(self, other)
      .enumerate()
      .scan(Range(self.distance(other) as usize), Range::apply)
      .map(denumerate)
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn line() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(3, 4, 10);
    let set: HashSet<Point> = point.line(&other);

    assert!(set.contains(&Point(1, 2, 5)));
    assert!(set.contains(&Point(1, 2, 6)));
    assert!(set.contains(&Point(2, 2, 6)));
    assert!(set.contains(&Point(2, 2, 7)));
    assert!(set.contains(&Point(2, 2, 8)));
    assert!(set.contains(&Point(2, 3, 8)));
    assert!(set.contains(&Point(2, 3, 9)));
    assert!(set.contains(&Point(3, 3, 9)));
    assert!(set.contains(&Point(3, 3, 10)));
    assert!(set.contains(&Point(3, 4, 10)));
    assert!(set.len() == 10);
  }

  #[test]
  fn line_vertical() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(1, 2, 7);
    let line: HashSet<Point> = point.line(&other);

    assert!(line.contains(&Point(1, 2, 5)));
    assert!(line.contains(&Point(1, 2, 6)));
    assert!(line.contains(&Point(1, 2, 7)));
    assert!(line.len() == 3);
  }
}
