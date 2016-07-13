//! Useful stuff for finding points in a ring

use std::collections::HashSet;

use point::Point;
use travel::{Direction, travel};

/// Find points at the same height in a ring of a provided radius
///
/// # Example
///
/// ```
/// use hex_math::{Point, ring};
/// use std::collections::HashSet;
///
/// let point: Point = Point::new(1, 2, 5);
/// let set: HashSet<Point> = ring(&point, 2);
///
/// assert!(set.contains(&Point::new(1, 0, 5)));
/// assert!(set.contains(&Point::new(2, 0, 5)));
/// assert!(set.contains(&Point::new(3, 0, 5)));
/// assert!(set.contains(&Point::new(3, 1, 5)));
/// assert!(set.contains(&Point::new(3, 2, 5)));
/// assert!(set.contains(&Point::new(2, 3, 5)));
/// assert!(set.contains(&Point::new(1, 4, 5)));
/// assert!(set.contains(&Point::new(0, 4, 5)));
/// assert!(set.contains(&Point::new(-1, 4, 5)));
/// assert!(set.contains(&Point::new(-1, 3, 5)));
/// assert!(set.contains(&Point::new(-1, 2, 5)));
/// assert!(set.contains(&Point::new(0, 1, 5)));
///
/// assert!(set.contains(&Point::new(1, 1, 6)));
/// assert!(set.contains(&Point::new(2, 1, 6)));
/// assert!(set.contains(&Point::new(2, 2, 6)));
/// assert!(set.contains(&Point::new(1, 3, 6)));
/// assert!(set.contains(&Point::new(0, 3, 6)));
/// assert!(set.contains(&Point::new(0, 2, 6)));
///
/// assert!(set.contains(&Point::new(1, 1, 4)));
/// assert!(set.contains(&Point::new(2, 1, 4)));
/// assert!(set.contains(&Point::new(2, 2, 4)));
/// assert!(set.contains(&Point::new(1, 3, 4)));
/// assert!(set.contains(&Point::new(0, 3, 4)));
/// assert!(set.contains(&Point::new(0, 2, 4)));
///
/// assert!(set.contains(&Point::new(1, 2, 7)));
/// assert!(set.contains(&Point::new(1, 2, 3)));
///
/// assert_eq!(set.len(), 26);
/// ```
pub fn ring(point: &Point, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = ring_2d(&point, range);

  for index in 1..range + 1 {
    let diff = range - index;
    let up: Point = travel(&point, Direction::Up, index);
    let down: Point = travel(&point, Direction::Down, index);
    let up_ring: HashSet<Point> = ring_2d(&up, diff);
    let down_ring: HashSet<Point> = ring_2d(&down, diff);

    set.extend(up_ring);
    set.extend(down_ring);
  }

  set.insert(travel(&point, Direction::Up, range));
  set.insert(travel(&point, Direction::Down, range));

  set
}

/// Find points at the same height in a ring of a provided radius
///
/// # Example
///
/// ```
/// use hex_math::{Point, ring_2d};
/// use std::collections::HashSet;
///
/// let point: Point = Point::new(1, 2, 5);
/// let set: HashSet<Point> = ring_2d(&point, 1);
///
/// assert_eq!(set.len(), 6);
/// assert!(set.contains(&Point::new(1, 1, 5)));
/// assert!(set.contains(&Point::new(2, 1, 5)));
/// assert!(set.contains(&Point::new(2, 2, 5)));
/// assert!(set.contains(&Point::new(1, 3, 5)));
/// assert!(set.contains(&Point::new(0, 3, 5)));
/// assert!(set.contains(&Point::new(0, 2, 5)));
/// ```
pub fn ring_2d(point: &Point, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = HashSet::new();
  let mut position: Point = travel(&point, Direction::Northwest, range);

  for direction in 0..6 {
    for _ in 0..range {

      let direction: Direction = match direction {
        0 => Direction::East,
        1 => Direction::Southeast,
        2 => Direction::Southwest,
        3 => Direction::West,
        4 => Direction::Northwest,
        5 => Direction::Northeast,
        _ => break,
      };

      set.insert(position);
      position = travel(&position, direction, 1);
    }
  }

  set
}

