use structs::Point;
use traits::{HasValues, HasWalls};

/// A prism is a point with walls
#[derive(Debug)]
pub struct Prism(pub Point, pub i32, pub i32, pub i32, pub i32);

/// Access the prism's coordinate values
impl HasValues for Prism {
  fn values(&self) -> (i32, i32, i32) {
    let &Prism(ref point, _, _, _, _) = self;

    point.values()
  }
}

/// Access the prism's wall strength values
impl HasWalls for Prism {
  fn walls(&self) -> (i32, i32, i32, i32) {
    let &Prism(_, e, se, sw, d) = self;

    (e, se, sw, d)
  }
}

/// Convert from a point to a prism with zero walls
impl<'a> From<&'a Point> for Prism {
  fn from(point: &'a Point) -> Prism {
    Prism(point.values().into(), 0, 0, 0, 0)
  }
}

/// Convert from tuples of values and wall strengths
impl From<((i32, i32, i32), (i32, i32, i32, i32))> for Prism {
  fn from(
    (values, (e, se, sw, d)): ((i32, i32, i32), (i32, i32, i32, i32))
  ) -> Prism {
    Prism(values.into(), e, se, sw, d)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let point: Point = Point(1, 2, 5);
    let Prism(Point(q, r, t), e, se, sw, d) = Prism(point, 1, 1, 1, 1);

    assert!(1 == q);
    assert!(2 == r);
    assert!(5 == t);
    assert!(1 == e);
    assert!(1 == se);
    assert!(1 == sw);
    assert!(1 == d);
  }

  #[test]
  fn values() {
    let point: Point = Point(1, 2, 5);
    let (q, r, t) = Prism(point, 1, 1, 1, 1).values();

    assert!(1 == q);
    assert!(2 == r);
    assert!(5 == t);
  }

  #[test]
  fn walls() {
    let point: Point = Point(1, 2, 5);
    let (s, se, sw, d) = Prism(point, 1, 1, 1, 1).walls();

    assert!(1 == s);
    assert!(1 == se);
    assert!(1 == sw);
    assert!(1 == d);
  }

  #[test]
  fn from_point() {
    let point: Point = Point(1, 2, 5);
    let Prism(Point(q, r, t), e, se, sw, d) = Prism::from(&point);

    assert!(1 == q);
    assert!(2 == r);
    assert!(5 == t);
    assert!(0 == e);
    assert!(0 == se);
    assert!(0 == sw);
    assert!(0 == d);
  }

  #[test]
  fn from_i32_tuples() {
    let prism: Prism = Prism::from(((1, 2, 5), (1, 1, 1, 1)));
    let Prism(Point(q, r, t), e, se, sw, d) = prism;

    assert!(1 == q);
    assert!(2 == r);
    assert!(5 == t);
    assert!(1 == e);
    assert!(1 == se);
    assert!(1 == sw);
    assert!(1 == d);
  }
}
