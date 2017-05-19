use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use structs::{Point, Prism};
use traits::range::Range;

/// Trait wrapping flood implementation
pub trait Flood: Borrow<Point> {
  /// Find reachable points within a specified range
  ///
  /// A point may be within range while being unreachable if the path to that
  /// point is blocked by an invalid point.
  fn flood<U: Borrow<Prism>>(
    &self,
    range: i32,
    map: &HashMap<Point, U>,
  ) -> HashSet<Point>;
}

impl<T> Flood for T where T: Borrow<Point> {
  fn flood<U: Borrow<Prism>>(
    &self,
    range: i32,
    map: &Hashmap<Point, U>
  ) -> HashSet<Point> {
    point.flood_generic(range, point::range, map)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use enums::Direction::*;
  use structs::Prism;
  use traits::IsPointMap;
  use traits::travel::Travel;

  #[test]
  fn flood() {
    let mut map: HashMap<Point, Prism> = HashMap::new();

    let start:     Point = Point(1, 2, 2);
    let west:      Point = start.travel(&West,      1);
    let northwest: Point = start.travel(&Northwest, 1);
    let northeast: Point = start.travel(&Northeast, 1);
    let up:        Point = start.travel(&Up,        1);

    map.insert_walled_point(Prism(west,      0, 1, 0, 0));
    map.insert_walled_point(Prism(northwest, 0, 1, 1, 0));
    map.insert_walled_point(Prism(northeast, 0, 0, 1, 0));
    map.insert_walled_point(Prism(up,        0, 0, 0, 1));
    map.insert_walled_point(Prism(start,     1, 1, 1, 1));

    let result: HashSet<Point> = start.flood(2, &map);

    assert!(result.contains(&start));
    assert!(result.contains(&Point( 0, 2, 2)));
    assert!(result.contains(&Point( 0, 1, 2)));
    assert!(result.contains(&Point(-1, 2, 2)));
    assert!(result.contains(&Point(-1, 3, 2)));
    assert!(result.contains(&Point( 0, 2, 1)));
    assert!(result.contains(&Point( 0, 2, 3)));
    assert!(result.len() == 7);
  }
}
