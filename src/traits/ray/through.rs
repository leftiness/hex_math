use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use fns::line::denumerate;
use structs::{Point, Prism};
use structs::line::Iterator;
use structs::line::predicate::{Range, Walls};

/// Trait wrapping ray through implementation
pub trait Through: Borrow<Point> {
  /// Find unblocked points within range in a line through two points
  fn ray_through<U: Borrow<Point>, V: Borrow<Prism>>(
    &self,
    other: &U,
    range: i32,
    walls: &HashMap<Point, V>,
  ) -> HashSet<Point>;
}

impl<T> Through for T where T: Borrow<Point> {
  fn ray_through<U: Borrow<Point>, V: Borrow<Prism>>(
    &self,
    other: &U,
    range: i32,
    walls: &HashMap<Point, V>,
  ) -> HashSet<Point> {
    Iterator::new(self, other)
      .scan(Walls(walls, *self.borrow()), Walls::apply)
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
  fn ray_through() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(2, 2, 6);
    let mut map: HashMap<Point, Prism> = HashMap::new();

    let wall: Point = Point(2, 2, 7);
    let prism: Prism = Prism(wall, 0, 0, 0, 1);

    map.insert(wall, prism);

    let set: HashSet<Point> = point.ray_through(&other, 3, &map);

    assert!(set.contains(&Point(1, 2, 5)));
    assert!(set.contains(&Point(1, 2, 6)));
    assert!(set.contains(&Point(2, 2, 6)));
    assert!(set.len() == 3);
  }
}
