use traits::{HasValues, HasWalls};
use point::Point;

/// A prism is a point with walls
///
#[derive(Debug)]
pub struct Prism {

  /// Strength of the east wall
  pub east: i32,

  /// Strength of the southeast wall
  pub southeast: i32,

  /// Strength of the southwest wall
  pub southwest: i32,

  /// Strength of the floor
  pub down: i32,

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
  /// let prism: Prism = Prism::new(point, 1, 1, 1, 1);
  /// ```
  pub fn new(
    point: Point,
    east: i32,
    southeast: i32,
    southwest: i32,
    down: i32
  ) -> Prism {
    Prism {
      point: point,
      east: east,
      southeast: southeast,
      southwest: southwest,
      down: down,
    }
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
/// let prism: Prism = Prism::new(point, 1, 1, 1, 1);
///
/// assert_eq!((1, 2, 5), prism.values());
/// ```
impl HasValues for Prism {

  fn values(&self) -> (i32, i32, i32) {
    self.point.values()
  }

}

/// Access the prism's wall strength values
///
/// # Example
///
/// ```
/// use hex_math::{Point, Prism, HasWalls};
///
/// let point: Point = Point::new(1, 2, 5);
/// let prism: Prism = Prism::new(point, 1, 1, 1, 1);
///
/// assert_eq!((1, 1, 1, 1), prism.walls());
/// ```
impl HasWalls for Prism {

  fn walls(&self) -> (i32, i32, i32, i32) {
    (self.east, self.southeast, self.southwest, self.down)
  }

}

/// Convert from tuples of values and wall strengths
///
/// # Example
///
/// ```
/// use hex_math::{Point, Prism, HasValues, HasWalls};
///
/// let point: Point = Point::new(1, 2, 5);
/// let values: (i32, i32, i32) = point.values();
/// let prism: Prism = Prism::new(point, 1, 1, 1, 1);
/// let other: Prism = Prism::from((values, prism.walls()));
///
/// assert_eq!((1, 2, 5), other.values());
/// assert_eq!((1, 1, 1, 1), other.walls());
/// ```
impl From<((i32, i32, i32), (i32, i32, i32, i32))> for Prism {

  fn from(
    (values, (e, se, sw, d)): ((i32, i32, i32), (i32, i32, i32, i32))
  ) -> Prism {
    Prism::new(Point::from(values), e, se, sw, d)
  }

}
