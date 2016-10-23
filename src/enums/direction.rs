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
