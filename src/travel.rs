//! Useful stuff for moving in a specified direction

use point::Point;

/// Enum describing positions in relation to a point
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
/// use hex_math::Point;
/// use hex_math::{Direction, travel};
///
/// let spot: Point = Point::new(1, 2, 5);
///
/// assert_eq!(travel(&spot, Direction::East     , 2), Point::new( 3, 2, 5));
/// assert_eq!(travel(&spot, Direction::Southeast, 2), Point::new( 1, 4, 5));
/// assert_eq!(travel(&spot, Direction::Southwest, 2), Point::new(-1, 4, 5));
/// assert_eq!(travel(&spot, Direction::West     , 2), Point::new(-1, 2, 5));
/// assert_eq!(travel(&spot, Direction::Northwest, 2), Point::new( 1, 0, 5));
/// assert_eq!(travel(&spot, Direction::Northeast, 2), Point::new( 3, 0, 5));
/// assert_eq!(travel(&spot, Direction::Up       , 2), Point::new( 1, 2, 7));
/// assert_eq!(travel(&spot, Direction::Down     , 2), Point::new( 1, 2, 3));
/// ```
pub fn travel(point: &Point, direction: Direction, units: i32) -> Point {
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
