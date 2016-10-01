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

}

