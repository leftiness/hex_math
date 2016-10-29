use std::cmp::Eq;
use std::hash::Hash;
use std::collections::HashMap;

use enums::Direction;
use traits::{HasValues, HasWalls};

pub trait IsPointMap<T, U> where T: HasValues + Eq + Hash, U: HasWalls {

  /// Check for a wall on the map
  fn has_wall(&self, &T, &Direction) -> bool;

  /// Check for a wall between two points on the map
  fn has_wall_between(&self, &T, &T) -> bool;

}

impl<T, U> IsPointMap<T, U> for HashMap<T, U>
  where T: HasValues + Eq + Hash, U: HasWalls {

  /// Check for a wall on the map
  ///
  /// # Example
  ///
  /// ```
  /// use std::collections::HashMap;
  /// use hex_math::{Direction, IsPointMap, HasValues, Point, Prism};
  ///
  /// let mut map: HashMap<Point, Prism> = HashMap::new();
  ///
  /// let p0: Point = Point::new(1, 2, 5);
  /// let pr0: Prism = Prism::new(p0.values().into(), 1, 0, 0, 0);
  ///
  /// map.insert(p0.values().into(), pr0);
  ///
  /// assert!(map.has_wall(&p0, &Direction::East));
  /// ```
  fn has_wall(&self, p0: &T, dir: &Direction) -> bool {
    match self.get(p0) {
      Some(p) => p.has_wall(dir),
      None => false,
    }
  }

  /// Check for a wall between two points on the map
  ///
  /// # Example
  ///
  /// ```
  /// use std::collections::HashMap;
  /// use hex_math::{IsPointMap, HasValues, Point, Prism};
  ///
  /// let mut map: HashMap<Point, Prism> = HashMap::new();
  ///
  /// let p0: Point = Point::new(0, 2, 5);
  /// let p1: Point = Point::new(1, 2, 5);
  /// let p2: Point = Point::new(2, 2, 5);
  ///
  /// let pr0: Prism = Prism::new(p0.values().into(), 1, 0, 0, 0);
  /// let pr1: Prism = Prism::new(p1.values().into(), 1, 0, 0, 0);
  ///
  /// map.insert(p0.values().into(), pr0);
  /// map.insert(p1.values().into(), pr1);
  ///
  /// assert!(map.has_wall_between(&p1, &p2));
  /// assert!(map.has_wall_between(&p1, &p0));
  /// ```
  fn has_wall_between(
    &self,
    p0: &T,
    p1: &T,
  ) -> bool {

    let dir: Direction = (p0, p1).into();
    let result = self.has_wall(p0, &dir) || self.has_wall(p1, &dir.opposite());

    result

  }

}

