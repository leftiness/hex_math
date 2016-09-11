//! Useful stuff for finding points in a range

use std::collections::HashSet;
use std::cmp::{max, min};

use traits::has_values::HasValues;
use enums::Direction;
use point::Point;
use travel::travel;

/// Find the points within the provided manhattan distance
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{Point,range};
///
/// let point: Point = Point::new(1, 2, 5);
/// let set: HashSet<Point> = range(&point, 1);
///
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 5)));
/// assert!(set.contains(&Point::new(1, 3, 5)));
/// assert!(set.contains(&Point::new(0, 3, 5)));
/// assert!(set.contains(&Point::new(0, 2, 5)));
/// assert!(set.contains(&Point::new(1, 1, 5)));
/// assert!(set.contains(&Point::new(2, 1, 5)));
/// assert!(set.contains(&Point::new(1, 2, 4)));
/// assert!(set.contains(&Point::new(1, 2, 6)));
/// assert_eq!(set.len(), 9);
/// ```
pub fn range<T: HasValues>(point: &T, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = range_2d(point, range);

  for index in 1..range + 1 {
    let diff = range - index;
    let up: Point = travel(point, Direction::Up, index);
    let down: Point = travel(point, Direction::Down, index);
    let up_range: HashSet<Point> = range_2d(&up, diff);
    let down_range: HashSet<Point> = range_2d(&down, diff);

    set.extend(up_range);
    set.extend(down_range);
  }

  set
}

/// Find the points at the same height within the provided manhattan distance
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{Point, range_2d};
///
/// let point: Point = Point::new(1, 2, 5);
/// let set: HashSet<Point> = range_2d(&point, 1);
///
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 5)));
/// assert!(set.contains(&Point::new(1, 3, 5)));
/// assert!(set.contains(&Point::new(0, 3, 5)));
/// assert!(set.contains(&Point::new(0, 2, 5)));
/// assert!(set.contains(&Point::new(1, 1, 5)));
/// assert!(set.contains(&Point::new(2, 1, 5)));
/// assert_eq!(set.len(), 7);
/// ```
pub fn range_2d<T: HasValues>(point: &T, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = HashSet::new();

  for dq in -range..range + 1 {
    let lower: i32 = max(-range, -dq - range);
    let upper: i32 = min(range, -dq + range);

    for ds in lower..upper + 1 {
      let dr: i32 = -dq - ds;
      let found = &point.to_point() + &Point::new(dq, dr, 0);

      set.insert(found);
    }

  }

  set
}

/// Find reachable points within a specified range
///
/// A point may be within range while being unreachable if the path to that
/// point is blocked by an invalid point.
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{flood, Point};
///
/// let point: Point = Point::new(1, 2, 2);
/// let mut invalid: HashSet<Point> = HashSet::new();
///
/// invalid.insert(Point::new(1, 1, 2));
/// invalid.insert(Point::new(2, 1, 2));
/// invalid.insert(Point::new(2, 2, 2));
/// invalid.insert(Point::new(1, 3, 2));
/// invalid.insert(Point::new(0, 3, 2));
/// invalid.insert(Point::new(1, 2, 1));
/// invalid.insert(Point::new(1, 2, 3));
///
/// let result: HashSet<Point> = flood(&point, 2, &invalid);
///
/// assert!(result.contains(&point));
/// assert!(result.contains(&Point::new(0, 2, 2)));
/// assert!(result.contains(&Point::new(0, 1, 2)));
/// assert!(result.contains(&Point::new(-1, 2, 2)));
/// assert!(result.contains(&Point::new(-1, 3, 2)));
/// assert!(result.contains(&Point::new(0, 2, 1)));
/// assert!(result.contains(&Point::new(0, 2, 3)));
/// assert_eq!(result.len(), 7);
/// ```
pub fn flood<T: HasValues>(
  point: &T,
  range: i32,
  invalid: &HashSet<Point>,
) -> HashSet<Point> {
  util::flood(point, range, self::range, invalid)
}

/// Find reachable points of the same height within a specified range
///
/// A point may be within range while being unreachable if the path to that
/// point is blocked by an invalid point.
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{flood_2d, Point};
///
/// let point: Point = Point::new_2d(1, 2);
/// let mut invalid: HashSet<Point> = HashSet::new();
///
/// invalid.insert(Point::new_2d(1, 1));
/// invalid.insert(Point::new_2d(2, 1));
/// invalid.insert(Point::new_2d(2, 2));
/// invalid.insert(Point::new_2d(1, 3));
/// invalid.insert(Point::new_2d(0, 3));
///
/// let result: HashSet<Point> = flood_2d(&point, 2, &invalid);
///
/// assert!(result.contains(&point));
/// assert!(result.contains(&Point::new_2d(0, 2)));
/// assert!(result.contains(&Point::new_2d(0, 1)));
/// assert!(result.contains(&Point::new_2d(-1, 2)));
/// assert!(result.contains(&Point::new_2d(-1, 3)));
/// assert_eq!(result.len(), 5);
/// ```
pub fn flood_2d<T: HasValues>(
  point: &T,
  range: i32,
  invalid: &HashSet<Point>,
) -> HashSet<Point> {
  util::flood(point, range, self::range_2d, invalid)
}

mod util {
  use std::collections::HashSet;

  use traits::has_values::HasValues;
  use point::Point;

  /// Find reachable points within a specified range with a provided function
  pub fn flood<T: HasValues>(
    start: &T,
    range: i32,
    range_fn: fn(&Point, i32) -> HashSet<Point>,
    invalid: &HashSet<Point>,
  ) -> HashSet<Point> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut fringes: Vec<Point> = Vec::new();
    let start: Point = start.to_point();

    if invalid.contains(&start) {
      return visited;
    }

    fringes.push(start);

    for _ in 1..range + 1 {
      let mut found = vec![];

      for point in &fringes {
        for neighbor in range_fn(point, 1) {
          if !invalid.contains(&neighbor) && !visited.contains(&neighbor) {
            found.push(neighbor);
          }
        }
      }

      visited.extend(fringes);
      fringes = found;
    }

    visited.extend(fringes);

    visited
  }

}

#[cfg(test)]
mod tests {
  use enums::Direction;
  use point::Point;
  use travel::travel;

  use super::util;

  use std::collections::HashSet;

  #[test]
  fn flood() {
    let point: Point = Point::new(0, 0, 0);
    let mut invalid: HashSet<Point> = HashSet::new();

    invalid.insert(Point::new(0, 0, 2));

    fn range_1d(point: &Point, range: i32) -> HashSet<Point> {
      let mut set: HashSet<Point> = HashSet::new();
      let up: Point = travel(point, Direction::Up, range);
      let down: Point = travel(point, Direction::Down, range);

      set.insert(up);
      set.insert(down);

      set
    };

    let result: HashSet<Point> = util::flood(&point, 2, range_1d, &invalid);

    assert!(result.contains(&point));
    assert!(result.contains(&Point::new(0, 0, 1)));
    assert!(result.contains(&Point::new(0, 0, -1)));
    assert!(result.contains(&Point::new(0, 0, -2)));
    assert_eq!(result.len(), 4);
  }

}
