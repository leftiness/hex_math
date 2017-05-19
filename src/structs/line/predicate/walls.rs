use std::borrow::Borrow;
use std::collections::HashMap;

use structs::{Point, Prism};
use traits::IsPointMap;

/// Stops the line if it hits a wall
#[derive(Debug)]
pub struct Walls<'a, T: Borrow<Prism> + 'a>(
  pub &'a HashMap<Point, T>,
  pub Point,
);

impl <'a, T> Walls<'a, T> where T: Borrow<Prism> {
  /// Determine if the point hit a wall
  pub fn apply(&mut self, next: Point) -> Option<Point> {
    let &mut Walls(walls, ref mut last) = self;

    if walls.has_wall_between(&last, &next) {
      None
    } else {
      *last = next;

      Some(next)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const POINT: &'static Point = &Point(1, 2, 5);
  const EAST: &'static Point = &Point(2, 2, 5);

  #[test]
  fn apply_with_wall() {
    let prism = Prism(*POINT, 1, 0, 0, 0);

    let mut walls = HashMap::new();

    walls.insert_walled_point(prism);

    assert!(Walls(&walls, *POINT).apply(*EAST).is_none());
  }

  #[test]
  fn apply_without_wall() {
    let walls: HashMap<Point, Prism> = HashMap::new();

    assert!(Walls(&walls, *POINT).apply(*EAST).is_some());
  }
}
