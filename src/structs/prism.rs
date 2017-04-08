use structs::Point;
use traits::{HasValues, HasWalls};

/// A prism is a point with walls
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
impl HasValues for Prism {

  fn values(&self) -> (i32, i32, i32) {
    self.point.values()
  }

}

/// Access the prism's wall strength values
impl HasWalls for Prism {

  fn walls(&self) -> (i32, i32, i32, i32) {
    (self.east, self.southeast, self.southwest, self.down)
  }

}

/// Convert from a point to a prism with zero walls
impl<'a> From<&'a Point> for Prism {

  fn from(point: &'a Point) -> Prism {
    Prism::new(Point::from(point.values()), 0, 0, 0, 0)
  }

}

/// Convert from tuples of values and wall strengths
impl From<((i32, i32, i32), (i32, i32, i32, i32))> for Prism {

  fn from(
    (values, (e, se, sw, d)): ((i32, i32, i32), (i32, i32, i32, i32))
  ) -> Prism {
    Prism::new(Point::from(values), e, se, sw, d)
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let point: Point = Point::new(1, 2, 5);
    let prism: Prism = Prism::new(point, 1, 1, 1, 1);

    assert!(1 == prism.point.q);
    assert!(2 == prism.point.r);
    assert!(5 == prism.point.t);
    assert!(1 == prism.east);
    assert!(1 == prism.southeast);
    assert!(1 == prism.southwest);
    assert!(1 == prism.down);
  }

  #[test]
  fn values() {
    let point: Point = Point::new(1, 2, 5);
    let (q, r, t) = Prism::new(point, 1, 1, 1, 1).values();

    assert!(1 == q);
    assert!(2 == r);
    assert!(5 == t);
  }

  #[test]
  fn walls() {
    let point: Point = Point::new(1, 2, 5);
    let (s, se, sw, d) = Prism::new(point, 1, 1, 1, 1).walls();

    assert!(1 == s);
    assert!(1 == se);
    assert!(1 == sw);
    assert!(1 == d);
  }

  #[test]
  fn from_point() {
    let point: Point = Point::new(1, 2, 5);
    let prism: Prism = Prism::from(&point);

    assert!(1 == prism.point.q);
    assert!(2 == prism.point.r);
    assert!(5 == prism.point.t);
    assert!(0 == prism.east);
    assert!(0 == prism.southeast);
    assert!(0 == prism.southwest);
    assert!(0 == prism.down);
  }

  #[test]
  fn from_i32_tuples() {
    let prism: Prism = Prism::from(((1, 2, 5), (1, 1, 1, 1)));

    assert!(1 == prism.point.q);
    assert!(2 == prism.point.r);
    assert!(5 == prism.point.t);
    assert!(1 == prism.east);
    assert!(1 == prism.southeast);
    assert!(1 == prism.southwest);
    assert!(1 == prism.down);
  }
}
