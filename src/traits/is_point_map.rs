use std::borrow::Borrow;
use std::collections::HashMap;

use enums::Direction;
use structs::{Point, Prism};

pub trait IsPointMap<T, U> where T: Borrow<Point>, U: Borrow<Prism> {

  /// Check for a wall on the map
  fn has_wall(&self, &T, &Direction) -> bool;

  /// Check for a wall between two points on the map
  fn has_wall_between(&self, &T, &T) -> bool;

  /// Insert a new walled point
  fn insert_walled_point(&mut self, U) -> Option<U>;

}

impl<U: Borrow<Prism>> IsPointMap<Point, U> for HashMap<Point, U> {

  /// Check for a wall on the map
  fn has_wall(&self, p0: &Point, dir: &Direction) -> bool {
    match self.get(p0.borrow()) {
      Some(p) => p.borrow().has_wall(dir),
      None => false,
    }
  }

  /// Check for a wall between two points on the map
  fn has_wall_between(
    &self,
    p0: &Point,
    p1: &Point,
  ) -> bool {
    if p0 == p1 {
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
    let &Prism(point, _, _, _, _) = prism.borrow();
    let old_value: Option<U> = self.insert(point, prism);

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

    let p0: Point = Point(1, 2, 5);
    let pr0: Prism = Prism(p0, 1, 0, 0, 0);

    map.insert(p0, pr0);

    assert!(map.has_wall(&p0, &Direction::East));
  }

  #[test]
  fn has_wall_between() {
    let mut map: HashMap<Point, Prism> = HashMap::new();

    let p0: Point = Point(0, 2, 5);
    let p1: Point = Point(1, 2, 5);
    let p2: Point = Point(2, 2, 5);

    let pr0: Prism = Prism(p0, 1, 0, 0, 0);
    let pr1: Prism = Prism(p1, 1, 0, 0, 0);

    map.insert(p0, pr0);
    map.insert(p1, pr1);

    assert!(map.has_wall_between(&p1, &p2));
    assert!(map.has_wall_between(&p1, &p0));
  }

  #[test]
  fn insert_walled_point() {
    let mut map: HashMap<Point, Prism> = HashMap::new();

    let p0: Point = Point(1, 2, 5);
    let pr0: Prism = Prism(p0, 1, 1, 1, 1);

    map.insert_walled_point(pr0);

    assert!(map.contains_key(&p0));

    let &Prism(Point(q, r, t), e, se, sw, d) = map.get(&p0).unwrap();

    assert!(1 == q);
    assert!(2 == r);
    assert!(5 == t);
    assert!(1 == e);
    assert!(1 == se);
    assert!(1 == sw);
    assert!(1 == d);
  }
}
