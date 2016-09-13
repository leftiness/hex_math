//! Provide access to a prism's walls

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

}
