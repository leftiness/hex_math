use std::borrow::Borrow;
use std::collections::HashSet;
use std::cmp::{max, min};

use structs::Point;

/// Find the points at the same height within the provided manhattan distance
pub fn base<T: Borrow<Point>>(point: &T, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = HashSet::new();

  for dq in -range..range + 1 {
    let lower: i32 = max(-range, -dq - range);
    let upper: i32 = min(range, -dq + range);

    for ds in lower..upper + 1 {
      let dr: i32 = -dq - ds;
      let found = point.borrow() + &Point(dq, dr, 0);

      set.insert(found);
    }
  }

  set
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn base() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = super::base(&point, 1);

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
