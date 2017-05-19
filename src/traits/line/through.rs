use std::borrow::Borrow;
use std::collections::HashSet;

use fns::line::denumerate;
use structs::Point;
use structs::line::Iterator;
use structs::line::predicate::Range;

pub trait Through: Borrow<Point> {
  /// Find the points within range in a line through two points
  fn line_through<U: Borrow<Point>>(
    &self,
    other: &U,
    range: i32,
  ) -> HashSet<Point>;
}

impl<T> Through for T where T: Borrow<Point> {
  fn line_through<U: Borrow<Point>>(
    &self,
    other: &U,
    range: i32,
  ) -> HashSet<Point> {
    Iterator::new(self, other)
      .enumerate()
      .scan(Range(range as usize), Range::apply)
      .map(denumerate)
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn line_through() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(2, 2, 6);
    let set: HashSet<Point> = point.line_through(&other, 3);

    assert!(set.contains(&Point(1, 2, 5)));
    assert!(set.contains(&Point(1, 2, 6)));
    assert!(set.contains(&Point(2, 2, 6)));
    assert!(set.contains(&Point(2, 2, 7)));
    assert!(set.len() == 4);
  }
}
