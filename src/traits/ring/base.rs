use std::borrow::Borrow;
use std::collections::HashSet;

use enums::Direction;
use structs::Point;
use traits::travel::Travel;

/// Trait wrapping base ring implementation
pub trait Base: Borrow<Point> {
  /// Find points at the same height in a ring of a provided radius
  fn base_ring(&self, range: i32) -> HashSet<Point>;
}

impl<T> Base for T where T: Borrow<Point> {
  fn base_ring(&self, range: i32) -> HashSet<Point> {
    let mut set: HashSet<Point> = HashSet::new();
    let mut point: Point = self.travel(&Direction::Northwest, range);

    for direction in Direction::to_vec().iter().take(6) {
      for _ in 0..range {
        set.insert(point);
        point = point.travel(direction, 1);
      }
    }

    set
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn base_ring() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = point.base_ring(1);

    assert!(set.contains(&Point(1, 1, 5)));
    assert!(set.contains(&Point(2, 1, 5)));
    assert!(set.contains(&Point(2, 2, 5)));
    assert!(set.contains(&Point(1, 3, 5)));
    assert!(set.contains(&Point(0, 3, 5)));
    assert!(set.contains(&Point(0, 2, 5)));
    assert!(set.len() == 6);
  }
}
