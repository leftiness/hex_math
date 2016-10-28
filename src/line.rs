use std::collections::{HashMap, HashSet};

use structs::{Point, Prism};
use traits::{HasValues, HasWalls};

/// Find the points in a line between the current point and the one provided
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{line, Point};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let set: HashSet<Point> = line(&point, &other);
///
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 6)));
/// assert!(set.contains(&Point::new(2, 3, 8)));
/// assert!(set.contains(&Point::new(3, 3, 9)));
/// assert!(set.contains(&Point::new(3, 4, 10)));
/// assert_eq!(set.len(), 5);
/// ```
pub fn line<T: HasValues>(
  point: &T,
  other: &T
) -> HashSet<Point> {
  util::line(point, other, None, None::<&HashMap<Point, Prism>>)
}

/// Find the points within range in a line through two points
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{line_through, Point};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(2, 2, 6);
/// let set: HashSet<Point> = line_through(&point, &other, 3);
///
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 6)));
/// assert!(set.contains(&Point::new(3, 2, 7)));
/// assert!(set.contains(&Point::new(4, 2, 8)));
/// assert_eq!(set.len(), 4);
/// ```
pub fn line_through<T: HasValues>(
  point: &T,
  other: &T,
  range: i32,
) -> HashSet<Point> {
  util::line(point, other, Some(range), None::<&HashMap<Point, Prism>>)
}

/// Find unblocked points in a line between two points
///
/// # Example
///
/// ```
/// use std::collections::{HashSet, HashMap};
///
/// use hex_math::{HasValues, ray, Point, Prism};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let mut map: HashMap<Point, Prism> = HashMap::new();
///
/// let wall: Point = Point::new(3, 4, 10);
/// let prism: Prism = Prism::new(Point::from(wall.values()), 0, 0, 0, 1);
///
/// map.insert(wall, prism);
///
/// let set: HashSet<Point> = ray(&point, &other, &map);
///
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 6)));
/// assert!(set.contains(&Point::new(2, 3, 8)));
/// assert!(set.contains(&Point::new(3, 3, 9)));
/// assert_eq!(set.len(), 4);
/// ```
pub fn ray<T: HasValues, U: HasWalls>(
  point: &T,
  other: &T,
  map: &HashMap<Point, U>,
) -> HashSet<Point> {
  util::line(point, other, None, Some(map))
}

/// Find unblocked points within range in a line through two points
///
/// # Example
///
/// ```
/// use std::collections::{HashSet, HashMap};
///
/// use hex_math::{HasValues, ray_through, Point, Prism};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(2, 2, 6);
/// let mut map: HashMap<Point, Prism> = HashMap::new();
///
/// let wall: Point = Point::new(4, 2, 8);
/// let prism: Prism = Prism::new(Point::from(wall.values()), 0, 0, 0, 1);
///
/// map.insert(wall, prism);
///
/// let set: HashSet<Point> = ray_through(&point, &other, 3, &map);
///
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 6)));
/// assert!(set.contains(&Point::new(3, 2, 7)));
/// assert_eq!(set.len(), 3);
/// ```
pub fn ray_through<T: HasValues, U: HasWalls>(
  point: &T,
  other: &T,
  range: i32,
  map: &HashMap<Point, U>,
) -> HashSet<Point> {
  util::line(point, other, Some(range), Some(map))
}

mod util {
  use std::collections::{HashSet, HashMap};

  use distance::{distance, distance_2d};
  use enums::Direction;
  use structs::{FloatPoint, Point};
  use traits::{HasValues, HasWalls};

  /// Return the floats one step along a line between two points
  ///
  /// The lerp is offset a small amount to prevent points from landing
  /// directly on the line between two hexes.
  pub fn step_size<T: HasValues>(a: &T, b: &T, step: f32) -> FloatPoint {

    let lerp = |x: i32, y: i32| 1e-6 + (y - x) as f32 * step;
    let (qa, ra, ta) = a.values();
    let (qb, rb, tb) = b.values();

    let result = FloatPoint::new(
      lerp(qa, qb),
      lerp(ra, rb),
      lerp(ta, tb),
    );

    result

  }

  /// Return the direction of one point which is one step after the other
  pub fn get_step_direction(point: &Point, other: &Point) -> Direction {

    let diff: Point = other - point;
    let (dq, dr, dt) = diff.values();

    match dt.signum() {
       1 => return Direction::Up,
      -1 => return Direction::Down,
       _ => (),
    }

    return match (dq.signum(), dr.signum()) {
      ( 1,  0) => Direction::East,
      ( 0,  1) => Direction::Southeast,
      (-1,  1) => Direction::Southwest,
      (-1,  0) => Direction::West,
      ( 0, -1) => Direction::Northwest,
      ( 1, -1) => Direction::Northeast,
      _ => panic!(),
    }

  }

  /// Find the points in a line between two points
  ///
  /// Optionally provide a range. The line will end at that range.
  ///
  /// Optionally provide a map with walls which are impassable.
  pub fn line<T: HasValues, U: HasWalls>(
    point: &T,
    other: &T,
    range: Option<i32>,
    map: Option<&HashMap<Point, U>>,
  ) -> HashSet<Point> {

    let mut set: HashSet<Point> = HashSet::new();

    set.insert(Point::from(point.values()));

    if point.values() == other.values() {
      return set;
    }

    let distance: i32 = if point.values_2d() == other.values_2d() {
      distance(point, other)
    } else {
      distance_2d(point, other)
    };

    let empty: HashMap<Point, U> = HashMap::new();
    let map: &HashMap<Point, U> = map.unwrap_or(&empty);

    let step: f32 = 1f32 / distance as f32;
    let size: FloatPoint = step_size(point, other, step);

    let mut found: FloatPoint = FloatPoint::from(point.values());
    let mut last: Point = Point::from(point.values());

    let has_wall = |point: &Point, dir: &Direction| match map.get(point) {
      Some(prism) => prism.has_wall(dir),
      None => false,
    };

    let has_wall = |point: &Point, other: &Point| {

      let dir: Direction = get_step_direction(point, other);
      let result = has_wall(point, &dir) || has_wall(other, &dir.opposite());

      result

    };

    let range: i32 = range.unwrap_or(distance);
    let mut steps: i32 = 0;

    // TODO This wip logic is broken. Tests fail. See github issue.
    'outer: for _ in 0 .. range {

      found = &found + &size;

      let round: Point = found.round();

      for t in last.t .. round.t {

        let vertical: Point = Point::new(round.q, round.r, t);

        if &vertical == &last {
          continue;
        }

        if has_wall(&last, &round) {
          break 'outer;
        }

        last = Point::from(vertical.values());
        set.insert(vertical);
        steps += 1;

        if steps == range {
          break 'outer;
        }

      }

      if has_wall(&last, &round) {
        break;
      }

      last = Point::from(round.values());
      set.insert(round);
      steps += 1;

      if steps == range {
        break;
      }

    }

    set

  }

}

#[cfg(test)]
mod tests {
  use std::collections::{HashSet, HashMap};

  use enums::Direction;
  use structs::{Point, Prism};
  use structs::FloatPoint;
  use traits::HasValues;

  use super::util;

  #[test]
  fn step_size() {
    let point: Point = Point::new(1, 2, 5);
    let other: Point = Point::new(1, 12, 5);
    let size: FloatPoint = util::step_size(&point, &other, 1f32);

    assert_eq!((1e-6, 10f32 + 1e-6, 1e-6), size.values());
  }

  #[test]
  fn line_vertical() {
    let point: Point = Point::new(1, 2, 5);
    let other: Point = Point::new(1, 2, 7);

    let line: HashSet<Point> = util::line(
      &point,
      &other,
      None,
      None::<&HashMap<Point, Prism>>
    );

    assert!(line.contains(&Point::new(1, 2, 5)));
    assert!(line.contains(&Point::new(1, 2, 6)));
    assert!(line.contains(&Point::new(1, 2, 7)));
    assert_eq!(line.len(), 3);
  }

  #[test]
  fn get_step_direction() {

    let point:     Point = Point::new(1, 2, 5);
    let east:      Point = Point::new(2, 2, 5);
    let southeast: Point = Point::new(1, 3, 5);
    let southwest: Point = Point::new(0, 3, 5);
    let west:      Point = Point::new(0, 2, 5);
    let northwest: Point = Point::new(1, 1, 5);
    let northeast: Point = Point::new(2, 1, 5);
    let up:        Point = Point::new(1, 2, 6);
    let down:      Point = Point::new(1, 2, 4);

    let get = util::get_step_direction;

    assert_eq!(Direction::East,      get(&point, &east));
    assert_eq!(Direction::Southeast, get(&point, &southeast));
    assert_eq!(Direction::Southwest, get(&point, &southwest));
    assert_eq!(Direction::West,      get(&point, &west));
    assert_eq!(Direction::Northwest, get(&point, &northwest));
    assert_eq!(Direction::Northeast, get(&point, &northeast));
    assert_eq!(Direction::Up,        get(&point, &up));
    assert_eq!(Direction::Down,      get(&point, &down));

  }

}

