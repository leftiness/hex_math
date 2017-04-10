use std::borrow::Borrow;
use std::collections::HashMap;

use structs::{Point, Prism};
use traits::{IsPointMap, Predicate};

/// Stops the line if it hits a wall
#[derive(Debug)]
pub struct Walls<'a, T: Borrow<Prism>>(pub &'a HashMap<Point, T>) where T: 'a;

impl <'a, T> Predicate<(i32, Point, Point)> for Walls<'a, T>
  where T: Borrow<Prism> {

  fn apply(&self, args: &(i32, Point, Point)) -> bool {
    let &Walls(walls) = self;
    let &(_, current, next) = args;

    walls.has_wall_between(&current, &next)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use enums::Direction;
  use travel::travel;

  const POINT: &'static Point = &Point(1, 2, 5);

  #[test]
  fn apply_with_wall() {
    let east = travel(&POINT, &Direction::East, 1);
    let prism = Prism(*POINT, 1, 0, 0, 0);

    let mut walls = HashMap::new();

    walls.insert_walled_point(prism);

    assert!(Walls(&walls).apply(&(0, *POINT, east)));
  }

  #[test]
  fn apply_without_wall() {
    let walls: HashMap<Point, Prism> = HashMap::new();

    assert!(!Walls(&walls).apply(&(0, Point(1, 2, 5), *POINT)));
  }
}
