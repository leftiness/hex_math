use std::borrow::Borrow;
use std::collections::HashSet;

use distance::with_height;
use line::{denumerate, Iterator};
use line::predicate::Range;
use structs::Point;

/// Find the points in a line between the current point and the one provided
pub fn of<T: Borrow<Point>>(point: &T, other: &T) -> HashSet<Point> {
  Iterator::new(point, other)
    .enumerate()
    .scan(Range(with_height(point, other) as usize), Range::apply)
    .map(denumerate)
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn of() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(3, 4, 10);
    let set: HashSet<Point> = super::of(&point, &other);

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
  fn of_vertical() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(1, 2, 7);
    let line: HashSet<Point> = super::of(&point, &other);

    assert!(line.contains(&Point(1, 2, 5)));
    assert!(line.contains(&Point(1, 2, 6)));
    assert!(line.contains(&Point(1, 2, 7)));
    assert!(line.len() == 3);
  }
}
