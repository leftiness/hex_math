use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use line;
use line::predicate::{Range, Walls};
use structs::{Point, Prism};

/// Find unblocked points within range in a line through two points
pub fn ray_through<T: Borrow<Point>, U: Borrow<Prism>>(
  point: &T,
  other: &T,
  range: i32,
  walls: &HashMap<Point, U>,
) -> HashSet<Point> {
  line::generic(point, other, (Walls(walls), Range(range)))
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

    let set: HashSet<Point> = super::ray_through(&point, &other, 3, &map);

    assert!(set.contains(&Point(1, 2, 5)));
    assert!(set.contains(&Point(1, 2, 6)));
    assert!(set.contains(&Point(2, 2, 6)));
    assert!(set.len() == 3);
  }
}
