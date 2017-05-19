use std::borrow::Borrow;
use std::collections::HashSet;

use enums::Direction;
use range;
use structs::Point;
use traits::travel::Travel;

/// Find the points within the provided manhattan distance
pub fn of<T: Borrow<Point>>(point: &T, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = range::base(point, range);

  for index in 1..range + 1 {
    let diff = range - index;
    let up: Point = point.travel(&Direction::Up, index);
    let down: Point = point.travel(&Direction::Down, index);
    let up_range: HashSet<Point> = range::base(&up, diff);
    let down_range: HashSet<Point> = range::base(&down, diff);

    set.extend(up_range);
    set.extend(down_range);
  }

  set
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn of() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = super::of(&point, 1);

    assert!(set.contains(&Point(1, 2, 5)));
    assert!(set.contains(&Point(2, 2, 5)));
    assert!(set.contains(&Point(1, 3, 5)));
    assert!(set.contains(&Point(0, 3, 5)));
    assert!(set.contains(&Point(0, 2, 5)));
    assert!(set.contains(&Point(1, 1, 5)));
    assert!(set.contains(&Point(2, 1, 5)));
    assert!(set.contains(&Point(1, 2, 4)));
    assert!(set.contains(&Point(1, 2, 6)));
    assert!(set.len() == 9);
  }
}
