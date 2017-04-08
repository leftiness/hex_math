use std::convert::From;
use std::cmp::Eq;
use std::hash::Hash;
use std::collections::HashMap;

use enums::Direction;
use traits::{HasValues, HasWalls};

pub trait IsPointMap<T, U>
  where T: HasValues + Eq + Hash + From<(i32, i32, i32)>
  , U: HasWalls {

  /// Check for a wall on the map
  fn has_wall(&self, &T, &Direction) -> bool;

  /// Check for a wall between two points on the map
  fn has_wall_between(&self, &T, &T) -> bool;

  /// Insert a new walled point
  fn insert_walled_point(&mut self, U) -> Option<U>;

}

impl<T, U> IsPointMap<T, U> for HashMap<T, U>
  where T: HasValues + Eq + Hash + From<(i32, i32, i32)>, U: HasWalls {

  /// Check for a wall on the map
  fn has_wall(&self, p0: &T, dir: &Direction) -> bool {
    match self.get(p0) {
      Some(p) => p.has_wall(dir),
      None => false,
    }
  }

  /// Check for a wall between two points on the map
  fn has_wall_between(
    &self,
    p0: &T,
    p1: &T,
  ) -> bool {

    if &p0 == &p1 {
      return false;
    }

    let dir: Direction = (p0, p1).into();
    let result = self.has_wall(p0, &dir) || self.has_wall(p1, &dir.opposite());

    result

  }

  /// Insert a new walled point
  fn insert_walled_point(
    &mut self,
    prism: U,
  ) -> Option<U> {

    let key: T = prism.values().into();
    let old_value: Option<U> = self.insert(key, prism);

    old_value

  }

}

#[cfg(test)]
mod tests {
  use super::*;
  use structs::{Point, Prism};

  #[test]
  fn has_wall() {
    let mut map: HashMap<Point, Prism> = HashMap::new();

    let p0: Point = Point::new(1, 2, 5);
    let pr0: Prism = Prism::new(p0.values().into(), 1, 0, 0, 0);

    map.insert(p0.values().into(), pr0);

    assert!(map.has_wall(&p0, &Direction::East));
  }

  #[test]
  fn has_wall_between() {
    let mut map: HashMap<Point, Prism> = HashMap::new();

    let p0: Point = Point::new(0, 2, 5);
    let p1: Point = Point::new(1, 2, 5);
    let p2: Point = Point::new(2, 2, 5);

    let pr0: Prism = Prism::new(p0.values().into(), 1, 0, 0, 0);
    let pr1: Prism = Prism::new(p1.values().into(), 1, 0, 0, 0);

    map.insert(p0.values().into(), pr0);
    map.insert(p1.values().into(), pr1);

    assert!(map.has_wall_between(&p1, &p2));
    assert!(map.has_wall_between(&p1, &p0));
  }

  #[test]
  fn insert_walled_point() {
    let mut map: HashMap<Point, Prism> = HashMap::new();

    let p0: Point = Point::new(1, 2, 5);
    let pr0: Prism = Prism::new(p0.values().into(), 1, 1, 1, 1);

    map.insert_walled_point(pr0);

    assert!(map.contains_key(&p0));

    let pr1: &Prism = map.get(&p0).unwrap();

    assert!((1, 2, 5) == pr1.values());
    assert!((1, 1, 1, 1) == pr1.walls());
  }
}
