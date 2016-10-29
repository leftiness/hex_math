use std::convert::From;

use structs::Point;
use traits::HasValues;

/// Enum describing positions in relation to a point
#[derive(Debug, PartialEq)]
pub enum Direction {
  East,
  Southeast,
  Southwest,
  West,
  Northwest,
  Northeast,
  Up,
  Down,
}

impl Direction {

  /// Get a vector of directions
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Direction};
  ///
  /// let vec: Vec<Direction> = Direction::to_vec();
  ///
  /// assert_eq!(Direction::East, vec[0]);
  /// ```
  pub fn to_vec() -> Vec<Direction> {
    vec![
      Direction::East,
      Direction::Southeast,
      Direction::Southwest,
      Direction::West,
      Direction::Northwest,
      Direction::Northeast,
      Direction::Up,
      Direction::Down,
    ]
  }

  /// Return the opposite direction
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::Direction;
  ///
  /// let east: Direction = Direction::East;
  ///
  /// assert_eq!(Direction::West, east.opposite());
  pub fn opposite(&self) -> Direction {

    match self {
      &Direction::East      => Direction::West,
      &Direction::Southeast => Direction::Northwest,
      &Direction::Southwest => Direction::Northeast,
      &Direction::West      => Direction::East,
      &Direction::Northwest => Direction::Southeast,
      &Direction::Northeast => Direction::Southwest,
      &Direction::Up        => Direction::Down,
      &Direction::Down      => Direction::Up,
    }

  }

}

impl <'a, 'b, T> From<(&'a T, &'b T)> for Direction where T: HasValues {

  /// Get the direction from one point to another
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Direction, Point};
  ///
  /// let point:     Point = Point::new(1, 2, 5);
  /// let east:      Point = Point::new(2, 2, 5);
  /// let southeast: Point = Point::new(1, 3, 5);
  /// let southwest: Point = Point::new(0, 3, 5);
  /// let west:      Point = Point::new(0, 2, 5);
  /// let northwest: Point = Point::new(1, 1, 5);
  /// let northeast: Point = Point::new(2, 1, 5);
  /// let up:        Point = Point::new(1, 2, 6);
  /// let down:      Point = Point::new(1, 2, 4);
  ///
  /// let from = Direction::from;
  ///
  /// assert_eq!(Direction::East,      from((&point, &east)));
  /// assert_eq!(Direction::Southeast, from((&point, &southeast)));
  /// assert_eq!(Direction::Southwest, from((&point, &southwest)));
  /// assert_eq!(Direction::West,      from((&point, &west)));
  /// assert_eq!(Direction::Northwest, from((&point, &northwest)));
  /// assert_eq!(Direction::Northeast, from((&point, &northeast)));
  /// assert_eq!(Direction::Up,        from((&point, &up)));
  /// assert_eq!(Direction::Down,      from((&point, &down)));
  /// ```
  fn from((p0, p1): (&'a T, &'b T)) -> Direction {

    let p0: Point = p0.values().into();
    let p1: Point = p1.values().into();
    let diff: Point = &p1 - &p0;

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

}
