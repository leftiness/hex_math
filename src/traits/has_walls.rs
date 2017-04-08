use enums::Direction;
use traits::HasValues;

/// Provide access to a prism's walls
pub trait HasWalls: HasValues {

  /// Return a tuple of (East, Southeast, Southwest, Down) wall strength
  ///
  /// A wall's strength determines what might pass through it or destroy it. A
  /// wall with zero strength is the same as no wall.
  ///
  /// Note that it's possible to designate all walls with only four directions
  /// by consistently using the same directions because one prism's west is
  /// another prism's east.
  fn walls(&self) -> (i32, i32, i32, i32);

  /// Return whether there is a wall in the provided direction
  ///
  /// If the direction is not one of the four directions, false will always
  /// be returned.
  fn has_wall(&self, direction: &Direction) -> bool {

    let (e, se, sw, d) = self.walls();

    let result: bool = match direction {
      &Direction::East => e > 0,
      &Direction::Southeast => se > 0,
      &Direction::Southwest => sw > 0,
      &Direction::Down => d > 0,
      _ => false,
    };

    result

  }

}

#[cfg(test)]
mod tests {
  use super::*;
  use structs::{Point, Prism};

  #[test]
  fn has_wall() {
    let point: Point = Point(1, 2, 5);
    let prism: Prism = Prism(point, 1, 0, 0, 0);

    assert!(prism.has_wall(&Direction::East));
    assert!(!prism.has_wall(&Direction::Southeast));
  }
}
