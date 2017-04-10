use std::borrow::Borrow;
use std::collections::HashSet;

use line;
use line::predicate::Range;
use structs::Point;

/// Find the points within range in a line through two points
pub fn through<T: Borrow<Point>>(
  point: &T,
  other: &T,
  range: i32,
) -> HashSet<Point> {
  line::generic(point, other, Range(range))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn line_through() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(2, 2, 6);
    let set: HashSet<Point> = super::through(&point, &other, 3);

    assert!(set.contains(&Point(1, 2, 5)));
    assert!(set.contains(&Point(1, 2, 6)));
    assert!(set.contains(&Point(2, 2, 6)));
    assert!(set.contains(&Point(2, 2, 7)));
    assert!(set.len() == 4);
  }
}
