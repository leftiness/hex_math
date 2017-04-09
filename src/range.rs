use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::cmp::{max, min};

use enums::Direction;
use structs::{Point, Prism};
use travel::travel;

/// Find the points within the provided manhattan distance
pub fn range<T: Borrow<Point>>(point: &T, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = range_2d(point, range);

  for index in 1..range + 1 {
    let diff = range - index;
    let up: Point = travel(point, &Direction::Up, index);
    let down: Point = travel(point, &Direction::Down, index);
    let up_range: HashSet<Point> = range_2d(&up, diff);
    let down_range: HashSet<Point> = range_2d(&down, diff);

    set.extend(up_range);
    set.extend(down_range);
  }

  set
}

/// Find the points at the same height within the provided manhattan distance
pub fn range_2d<T: Borrow<Point>>(point: &T, range: i32) -> HashSet<Point> {
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

/// Find reachable points within a specified range
///
/// A point may be within range while being unreachable if the path to that
/// point is blocked by an invalid point.
pub fn flood<T: Borrow<Point>, U: Borrow<Prism>>(
  point: &T,
  range: i32,
  map: &HashMap<Point, U>,
) -> HashSet<Point> {
  util::flood(point, range, self::range, map)
}

/// Find reachable points of the same height within a specified range
///
/// A point may be within range while being unreachable if the path to that
/// point is blocked by an invalid point.
pub fn flood_2d<T: Borrow<Point>, U: Borrow<Prism>>(
  point: &T,
  range: i32,
  map: &HashMap<Point, U>,
) -> HashSet<Point> {
  util::flood(point, range, self::range_2d, map)
}

mod util {
  use super::*;

  use traits::IsPointMap;

  /// Find reachable points within a specified range with a provided function
  pub fn flood<T: Borrow<Point>, U: Borrow<Prism>>(
    start: &T,
    range: i32,
    range_fn: fn(&Point, i32) -> HashSet<Point>,
    map: &HashMap<Point, U>,
  ) -> HashSet<Point> {

    let mut visited: HashSet<Point> = HashSet::new();
    let mut fringes: Vec<Point> = Vec::new();
    let mut found: Vec<Point> = Vec::new();

    fringes.push(*start.borrow());

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
  use structs::Prism;
  use traits::IsPointMap;

  #[test]
  fn range() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = super::range(&point, 1);

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

  #[test]
  fn range_2d() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = super::range_2d(&point, 1);

    assert!(set.contains(&Point(1, 2, 5)));
    assert!(set.contains(&Point(2, 2, 5)));
    assert!(set.contains(&Point(1, 3, 5)));
    assert!(set.contains(&Point(0, 3, 5)));
    assert!(set.contains(&Point(0, 2, 5)));
    assert!(set.contains(&Point(1, 1, 5)));
    assert!(set.contains(&Point(2, 1, 5)));
    assert!(set.len() == 7);
  }

  #[test]
  fn flood() {
    let mut map: HashMap<Point, Prism> = HashMap::new();

    let start:     Point = Point(1, 2, 2);
    let west:      Point = travel(&start, &Direction::West,      1);
    let northwest: Point = travel(&start, &Direction::Northwest, 1);
    let northeast: Point = travel(&start, &Direction::Northeast, 1);
    let up:        Point = travel(&start, &Direction::Up,        1);

    map.insert_walled_point(Prism(west,      0, 1, 0, 0));
    map.insert_walled_point(Prism(northwest, 0, 1, 1, 0));
    map.insert_walled_point(Prism(northeast, 0, 0, 1, 0));
    map.insert_walled_point(Prism(up,        0, 0, 0, 1));
    map.insert_walled_point(Prism(start,     1, 1, 1, 1));

    let result: HashSet<Point> = super::flood(&start, 2, &map);

    assert!(result.contains(&start));
    assert!(result.contains(&Point( 0, 2, 2)));
    assert!(result.contains(&Point( 0, 1, 2)));
    assert!(result.contains(&Point(-1, 2, 2)));
    assert!(result.contains(&Point(-1, 3, 2)));
    assert!(result.contains(&Point( 0, 2, 1)));
    assert!(result.contains(&Point( 0, 2, 3)));
    assert!(result.len() == 7);
  }

  #[test]
  fn flood_2d() {
    let mut map: HashMap<Point, Prism> = HashMap::new();

    let start:     Point = Point(1, 2, 0);
    let west:      Point = travel(&start, &Direction::West,      1);
    let northwest: Point = travel(&start, &Direction::Northwest, 1);
    let northeast: Point = travel(&start, &Direction::Northeast, 1);

    map.insert_walled_point(Prism(west,      0, 1, 0, 0));
    map.insert_walled_point(Prism(northwest, 0, 1, 1, 0));
    map.insert_walled_point(Prism(northeast, 0, 0, 1, 0));
    map.insert_walled_point(Prism(start,     1, 1, 1, 0));

    let result: HashSet<Point> = super::flood_2d(&start, 2, &map);

    assert!(result.contains(&start));
    assert!(result.contains(&Point( 0, 2, 0)));
    assert!(result.contains(&Point( 0, 1, 0)));
    assert!(result.contains(&Point(-1, 2, 0)));
    assert!(result.contains(&Point(-1, 3, 0)));
    assert!(result.len() == 5);
  }

  #[test]
  fn flood_util() {

    let start: Point = Point(0, 0, 0);
    let wall: Point = Point(0, 0, 2);

    let mut map: HashMap<Point, Prism> = HashMap::new();

    map.insert(wall, Prism(wall, 0, 0, 0, 1));

    fn range_1d(point: &Point, range: i32) -> HashSet<Point> {

      let mut set: HashSet<Point> = HashSet::new();
      let up: Point = travel(point, &Direction::Up, range);
      let down: Point = travel(point, &Direction::Down, range);

      set.insert(up);
      set.insert(down);

      set

    };

    let result: HashSet<Point> = util::flood(&start, 2, range_1d, &map);

    assert!(result.contains(&start));
    assert!(result.contains(&Point(0, 0, 1)));
    assert!(result.contains(&Point(0, 0, -1)));
    assert!(result.contains(&Point(0, 0, -2)));
    assert!(result.len() == 4);
  }
}
