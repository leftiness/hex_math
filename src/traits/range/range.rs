use std::borrow::Borrow;
use std::collections::HashSet;

use enums::Direction;
use structs::Point;
use traits::travel::Travel;
use traits::range::Base;

/// Trait wrapping range implementation
pub trait Range: Borrow<Point> {
  /// Find the points within the provided manhattan distance
  fn range(&self, range: i32) -> HashSet<Point>;
}

impl<T> Range for T where T: Borrow<Point> {
  pub fn range(&self, range: i32) -> HashSet<Point> {
    let mut set: HashSet<Point> = point.base_range(range);

    for index in 1..range + 1 {
      let diff = range - index;

      set.extend(point.travel(&Direction::Up, index).base_range(diff));
      set.extend(point.travel(&Direction::Down, index).base_range(diff));
    }

    set
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn range() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = point.range(1);

    assert!(set.contains(&Point(1, 2, 5)));
    assert!(set.contains(&Point(2, 2, 5)));
    assert!(set.contains(&Point(1, 3, 5)));
    assert!(set.contains(&Point(0, 3, 5)));
    assert!(set.contains(&Point(0, 2, 5)));
    assert!(set.contains(&Point(1, 1, 5)));
    assert!(set.contains(&Point(2, 1, 5)));
    assert!(set.contains(&Point(1, 2, 4)));
    assert!(set.contains(&Point(1, 2, 6)));
    assert!(set.len() == 9);
  }
}
