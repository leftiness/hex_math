//! A prism is a point with walls

use std::collections::HashMap;

use traits::has_values::HasValues;
use enums::Direction;
use point::Point;

/// A prism is a point with walls
///
/// A wall's strength determines what might pass through it or destroy it. A
/// wall with zero strength is the same as no wall.
///
/// Note that it's possible to designate all walls with only four directions
/// by consistently using the same directions because one prism's west is
/// another prism's east.
#[derive(Debug)]
pub struct Prism {

  /// The walls surrounding the point and their strength
  pub walls: HashMap<Direction, i32>,

  /// The point surrounded by walls
  pub point: Point,

}

impl Prism {

  /// Factory function for making new prisms
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Point, Prism};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  /// let prism: Prism = Prism::new(point);
  /// ```
  pub fn new(point: Point) -> Prism {
    Prism { point: point, walls: HashMap::new() }
  }

  /// Add walls to the prism
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Direction, Point, Prism};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  /// let prism: Prism = Prism::new(point).add_wall(Direction::East, 1);
  ///
  /// assert!(&prism.walls.contains_key(&Direction::East));
  /// assert_eq!(&1, prism.walls.get(&Direction::East).unwrap());
  /// ```
  pub fn add_wall(mut self, direction: Direction, strength: i32) -> Prism {
    self.walls.insert(direction, strength);

    self
  }

  /// Test for a wall with at least 1 strength
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Direction, Point, Prism};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  /// let prism: Prism = Prism::new(point).add_wall(Direction::East, 1);
  ///
  /// assert!(prism.has_wall(Direction::East));
  /// ```
  pub fn has_wall(&self, direction: Direction) -> bool {
    self.has_wall_strength(direction, 1)
  }

  /// Test for a wall with at least the provided strength
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Direction, Point, Prism};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  /// let prism: Prism = Prism::new(point).add_wall(Direction::East, 1);
  ///
  /// assert!(prism.has_wall_strength(Direction::East, 1));
  /// ```
  pub fn has_wall_strength(
    &self,
    direction: Direction,
    strength: i32,
  ) -> bool {
    self.walls.get(&direction).unwrap_or(&0) >= &strength
  }

}

/// Access the prism's coordinate values
///
/// # Example
///
/// ```
/// use hex_math::{Point, Prism, HasValues};
///
/// let point: Point = Point::new(1, 2, 5);
/// let prism: Prism = Prism::new(point);
///
/// assert_eq!((1, 2, 5), prism.values());
/// assert_eq!((1, 2, -3, 5), prism.values_cube());
/// assert_eq!((1, 2), prism.values_2d());
/// assert_eq!((1, 2, -3), prism.values_cube_2d());
/// ```
impl HasValues for Prism {

  fn values(&self) -> (i32, i32, i32) {
    self.point.values()
  }

}
