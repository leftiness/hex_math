//! Enum describing positions in relation to a point

use std::convert::From;

/// Enum describing positions in relation to a point
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

/// Convert from i32 to Direction
///
/// # Example
///
/// ```
/// use hex_math::Direction;
///
/// let direction: Direction = Direction::from(0);
/// assert_eq!(direction, Direction::East);
/// ```
///
/// ```
/// use hex_math::Direction;
///
/// for i in 0..6 {
///   let direction: Direction = Direction::from(i);
/// }
/// ```
impl From<i32> for Direction {

  fn from(number: i32) -> Direction {

    let result = match number % 6 {
      0 => Direction::East,
      1 => Direction::Southeast,
      2 => Direction::Southwest,
      3 => Direction::West,
      4 => Direction::Northwest,
      5 => Direction::Northeast,
      _ => panic!(),
    };

    result

  }

}
