use std::borrow::Borrow;
use std::collections::HashSet;
use std::cmp::{max, min};

use structs::Point;

/// Trait wrapping base range implementation
pub trait Base: Borrow<Point> {
  /// Find the points at the same height within the provided manhattan distance
  fn base_range(&self, range: i32) -> HashSet<Point>;
}

impl<T> Base for T where T: Borrow<Point> {
  pub fn base_range(&self, range: i32) -> HashSet<Point> {
    let mut set: HashSet<Point> = HashSet::new();

    for dq in -range..range + 1 {
      let lower: i32 = max(-range, -dq - range);
      let upper: i32 = min(range, -dq + range);

      for ds in lower..upper + 1 {
        let dr: i32 = -dq - ds;
        let found = self.borrow() + &Point(dq, dr, 0);

        set.insert(found);
      }
    }

    set
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn base_range() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = point.base_range(&point, 1);

    assert!(set.contains(&Point(1, 2, 5)));
    assert!(set.contains(&Point(2, 2, 5)));
    assert!(set.contains(&Point(1, 3, 5)));
    assert!(set.contains(&Point(0, 3, 5)));
    assert!(set.contains(&Point(0, 2, 5)));
    assert!(set.contains(&Point(1, 1, 5)));
    assert!(set.contains(&Point(2, 1, 5)));
    assert!(set.len() == 7);
  }
}
