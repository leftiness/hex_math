use std::convert::From;

use structs::Point;
use traits::HasValues;

use Direction::*;

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
  pub fn to_vec() -> Vec<Direction> {
    vec![ East, Southeast, Southwest, West, Northwest, Northeast, Up, Down ]
  }

  /// Return the opposite direction
    pub fn opposite(&self) -> Direction {

    match self {
      &East      => West,
      &Southeast => Northwest,
      &Southwest => Northeast,
      &West      => East,
      &Northwest => Southeast,
      &Northeast => Southwest,
      &Up        => Down,
      &Down      => Up,
    }

  }

}

impl <'a, 'b, T> From<(&'a T, &'b T)> for Direction where T: HasValues {

  /// Get the direction from one point to another
  fn from((p0, p1): (&'a T, &'b T)) -> Direction {

    let p0: Point = p0.values().into();
    let p1: Point = p1.values().into();
    let diff: Point = &p1 - &p0;

    let (dq, dr, dt) = diff.values();

    match dt.signum() {
       1 => return Up,
      -1 => return Down,
       _ => (),
    }

    return match (dq.signum(), dr.signum()) {
      ( 1,  0) => East,
      ( 0,  1) => Southeast,
      (-1,  1) => Southwest,
      (-1,  0) => West,
      ( 0, -1) => Northwest,
      ( 1, -1) => Northeast,
      _ => panic!(),
    }

  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn to_vec() {
    let vec: Vec<Direction> = Direction::to_vec();

    assert!(vec[0] == East);
    assert!(vec[1] == Southeast);
    assert!(vec[2] == Southwest);
    assert!(vec[3] == West);
    assert!(vec[4] == Northwest);
    assert!(vec[5] == Northeast);
    assert!(vec[6] == Up);
    assert!(vec[7] == Down);
    assert!(vec.len() == 8)
  }

  #[test]
  fn opposite() {
    assert!(East.opposite()      == West);
    assert!(Southeast.opposite() == Northwest);
    assert!(Southwest.opposite() == Northeast);
    assert!(West.opposite()      == East);
    assert!(Northwest.opposite() == Southeast);
    assert!(Northeast.opposite() == Southwest);
    assert!(Up.opposite()        == Down);
    assert!(Down.opposite()      == Up);
  }

  #[test]
  fn from() {
    let point:     Point = Point(1, 2, 5);
    let east:      Point = Point(2, 2, 5);
    let southeast: Point = Point(1, 3, 5);
    let southwest: Point = Point(0, 3, 5);
    let west:      Point = Point(0, 2, 5);
    let northwest: Point = Point(1, 1, 5);
    let northeast: Point = Point(2, 1, 5);
    let up:        Point = Point(1, 2, 6);
    let down:      Point = Point(1, 2, 4);

    let from = Direction::from;

    assert!(East      == from((&point, &east)));
    assert!(Southeast == from((&point, &southeast)));
    assert!(Southwest == from((&point, &southwest)));
    assert!(West      == from((&point, &west)));
    assert!(Northwest == from((&point, &northwest)));
    assert!(Northeast == from((&point, &northeast)));
    assert!(Up        == from((&point, &up)));
    assert!(Down      == from((&point, &down)));
  }
}
