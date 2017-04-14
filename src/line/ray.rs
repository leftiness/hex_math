use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use distance::with_height;
use line::{denumerate, Iterator};
use line::predicate::{Range, Walls};
use structs::{Point, Prism};

/// Find unblocked points in a line between two points
pub fn ray<T: Borrow<Point>, U: Borrow<Prism>>(
  point: &T,
  other: &T,
  walls: &HashMap<Point, U>,
) -> HashSet<Point> {
  Iterator::new(point, other)
    .scan(Walls(walls, *point.borrow()), Walls::apply)
    .enumerate()
    .scan(Range(with_height(point, other) as usize), Range::apply)
    .map(denumerate)
    .collect()
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

    let set: HashSet<Point> = super::ray(&point, &other, &map);

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
