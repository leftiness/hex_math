//! Useful stuff for moving in a specified direction

use traits::has_values::HasValues;
use point::Point;

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

/// Create a point in the specified direction a specified number of units
///
/// # Example
///
/// ```
/// use hex_math::{Direction, Point, travel};
///
/// let point: Point = Point::new(1, 2, 5);
///
/// assert_eq!(travel(&point, Direction::East     , 2), Point::new( 3, 2, 5));
/// assert_eq!(travel(&point, Direction::Southeast, 2), Point::new( 1, 4, 5));
/// assert_eq!(travel(&point, Direction::Southwest, 2), Point::new(-1, 4, 5));
/// assert_eq!(travel(&point, Direction::West     , 2), Point::new(-1, 2, 5));
/// assert_eq!(travel(&point, Direction::Northwest, 2), Point::new( 1, 0, 5));
/// assert_eq!(travel(&point, Direction::Northeast, 2), Point::new( 3, 0, 5));
/// assert_eq!(travel(&point, Direction::Up       , 2), Point::new( 1, 2, 7));
/// assert_eq!(travel(&point, Direction::Down     , 2), Point::new( 1, 2, 3));
/// ```
pub fn travel<T: HasValues>(
  point: &T,
  direction: Direction,
  units: i32
) -> Point {
  let (q, r, t) = point.values();

  return match direction {
    Direction::East      => Point::new(q + units, r        , t        ),
    Direction::Southeast => Point::new(q        , r + units, t        ),
    Direction::Southwest => Point::new(q - units, r + units, t        ),
    Direction::West      => Point::new(q - units, r        , t        ),
    Direction::Northwest => Point::new(q        , r - units, t        ),
    Direction::Northeast => Point::new(q + units, r - units, t        ),
    Direction::Up        => Point::new(q        , r        , t + units),
    Direction::Down      => Point::new(q        , r        , t - units),
  }

}
