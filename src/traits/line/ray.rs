use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use fns::line::denumerate;
use structs::{Point, Prism};
use structs::line::Iterator;
use traits::distance::Distance;
use structs::line::predicate::{Range, Walls};

/// Trait wrapping ray implementation
pub trait Ray: Borrow<Point> {
  /// Find unblocked points in a line between two points
  fn ray<U: Borrow<Point>, V: Borrow<Prism>>(
    &self,
    other: &U,
    walls: &HashMap<Point, V>,
  ) -> HashSet<Point>;
}

impl<T> Ray for T where T: Borrow<Point> {
  fn ray<U: Borrow<Point>, V: Borrow<Prism>>(
    &self,
    other: &U,
    walls: &HashMap<Point, V>,
  ) -> HashSet<Point> {
    Iterator::new(self, other)
      .scan(Walls(walls, *self.borrow()), Walls::apply)
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
  fn ray() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(3, 4, 10);
    let mut map: HashMap<Point, Prism> = HashMap::new();

    let wall: Point = Point(3, 3, 10);
    let prism: Prism = Prism(wall, 0, 0, 0, 1);

    map.insert(wall, prism);

    let set: HashSet<Point> = point.ray(&other, &map);

    assert!(set.contains(&Point(1, 2, 5)));
    assert!(set.contains(&Point(1, 2, 6)));
    assert!(set.contains(&Point(2, 2, 6)));
    assert!(set.contains(&Point(2, 2, 7)));
    assert!(set.contains(&Point(2, 2, 8)));
    assert!(set.contains(&Point(2, 3, 8)));
    assert!(set.contains(&Point(2, 3, 9)));
    assert!(set.contains(&Point(3, 3, 9)));
    assert!(set.len() == 8);
  }
}
