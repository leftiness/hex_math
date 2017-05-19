use std::borrow::Borrow;
use std::collections::HashSet;

use enums::Direction;
use structs::Point;
use traits::ring::Base;
use traits::travel::Travel;

/// Trait wrapping ring implementation
pub trait Ring: Borrow<Point> {
  /// Find points in a spherical ring of a provided radius
  fn ring(&self, range: i32) -> HashSet<Point>;
}

impl<T> Ring for T where T: Borrow<Point> {
  fn ring(&self, range: i32) -> HashSet<Point> {
    let mut set: HashSet<Point> = self.base_ring(range);

    for index in 1..range + 1 {
      let diff = range - index;

      set.extend(self.travel(&Direction::Up, index).base_ring(diff));
      set.extend(self.travel(&Direction::Down, index).base_ring(diff));
    }

    set.insert(self.travel(&Direction::Up, range));
    set.insert(self.travel(&Direction::Down, range));

    set
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ring() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = point.ring(2);

    assert!(set.contains(&Point(1, 0, 5)));
    assert!(set.contains(&Point(2, 0, 5)));
    assert!(set.contains(&Point(3, 0, 5)));
    assert!(set.contains(&Point(3, 1, 5)));
    assert!(set.contains(&Point(3, 2, 5)));
    assert!(set.contains(&Point(2, 3, 5)));
    assert!(set.contains(&Point(1, 4, 5)));
    assert!(set.contains(&Point(0, 4, 5)));
    assert!(set.contains(&Point(-1, 4, 5)));
    assert!(set.contains(&Point(-1, 3, 5)));
    assert!(set.contains(&Point(-1, 2, 5)));
    assert!(set.contains(&Point(0, 1, 5)));

    assert!(set.contains(&Point(1, 1, 6)));
    assert!(set.contains(&Point(2, 1, 6)));
    assert!(set.contains(&Point(2, 2, 6)));
    assert!(set.contains(&Point(1, 3, 6)));
    assert!(set.contains(&Point(0, 3, 6)));
    assert!(set.contains(&Point(0, 2, 6)));

    assert!(set.contains(&Point(1, 1, 4)));
    assert!(set.contains(&Point(2, 1, 4)));
    assert!(set.contains(&Point(2, 2, 4)));
    assert!(set.contains(&Point(1, 3, 4)));
    assert!(set.contains(&Point(0, 3, 4)));
    assert!(set.contains(&Point(0, 2, 4)));

    assert!(set.contains(&Point(1, 2, 7)));
    assert!(set.contains(&Point(1, 2, 3)));
    assert!(set.len() == 26);
  }
}
