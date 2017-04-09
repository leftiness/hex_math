use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use line;
use structs::{Point, Prism};

/// Find the points in a line between the current point and the one provided
pub fn of<T: Borrow<Point>>(point: &T, other: &T) -> HashSet<Point> {
  line::generic(point, other, None, None::<&HashMap<Point, Prism>>)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn line() {
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
  fn line_vertical() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(1, 2, 7);
    let line: HashSet<Point> = super::of(&point, &other);

    assert!(line.contains(&Point(1, 2, 5)));
    assert!(line.contains(&Point(1, 2, 6)));
    assert!(line.contains(&Point(1, 2, 7)));
    assert!(line.len() == 3);
  }
}
