use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use structs::{Point, Prism};
use traits::IsPointMap;

/// Trait wrapping generic flood implementation
pub trait GenericFlood: Borrow<Point> {
  /// Find reachable points within a specified range with a provided function
  fn generic_flood<U: Borrow<Prism>>(
    &self,
    range: i32,
    range_fn: fn(&Point, i32) -> HashSet<Point>,
    map: &HashMap<Point, U>,
  ) -> HashSet,Point>;
}

impl<T> GenericFlood for T where T: Borrow<Point> {
  fn generic_flood<U: Borrow<Prism>>(
    &self,
    range: i32,
    range_fn: fn(&Point, i32) -> HashSet<Point>,
    map: &HashMap<Point, U>,
  ) -> HashSet<Point> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut fringes: Vec<Point> = Vec::new();
    let mut found: Vec<Point> = Vec::new();

    fringes.push(*self.borrow());

    for _ in 0 .. range {
      for point in &fringes {
        for neighbor in &range_fn(point, 1) {
          if visited.contains(neighbor) {
            continue;
          } else if !map.has_wall_between(point, neighbor) {
            found.push(*neighbor);
          }
        }
      }

      visited.extend(fringes);
      fringes = found;
      found = Vec::new();
    }

    visited.extend(fringes);

    visited
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use enums::Direction::*;
  use structs::Prism;
  use traits::travel::Travel;

  #[test]
  fn generic_flood() {

    let start: Point = Point(0, 0, 0);
    let wall: Point = Point(0, 0, 2);

    let mut map: HashMap<Point, Prism> = HashMap::new();

    map.insert(wall, Prism(wall, 0, 0, 0, 1));

    fn range_1d(point: &Point, range: i32) -> HashSet<Point> {

      let mut set: HashSet<Point> = HashSet::new();
      let up: Point = point.travel(&Up, range);
      let down: Point = point.travel(&Down, range);

      set.insert(up);
      set.insert(down);

      set

    };

    let result: HashSet<Point> = start.generic_flood(2, range_1d, &map);

    assert!(result.contains(&start));
    assert!(result.contains(&Point(0, 0, 1)));
    assert!(result.contains(&Point(0, 0, -1)));
    assert!(result.contains(&Point(0, 0, -2)));
    assert!(result.len() == 4);
  }
}
