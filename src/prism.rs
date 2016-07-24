//! A prism is a point with walls

use std::collections::HashMap;
use std::convert::From;

use point::Point;
use travel::Direction;

/// A prism is a point with walls
///
/// A wall's strength determines what might pass through it or destroy it. A
/// wall with zero strength is the same as no wall.
///
/// Note that it's possible to designate all walls with only four directions
/// by consistently using the same directions because one prism's west is
/// another prism's east.
#[derive(Clone, Debug)]
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

/// Convert a point to a prism
///
/// # Example
///
/// ```
/// use std::convert::From;
///
/// use hex_math::{Point, Prism};
///
/// let point: Point = Point::new(1, 2, 5);
/// let prism: Prism = Prism::new(point);
/// let other: Point = From::from(prism);
///
/// assert_eq!(point, other);
/// ```
impl From<Point> for Prism {

  fn from(point: Point) -> Prism {
    Prism::new(point)
  }

}

