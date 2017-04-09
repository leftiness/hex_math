use std::borrow::Borrow;
use std::collections::HashSet;

use enums::Direction;
use ring;
use structs::Point;
use travel::travel;

/// Find points in a spherical ring of a provided radius
pub fn of<T: Borrow<Point>>(point: &T, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = ring::base(point, range);

  for index in 1..range + 1 {
    let diff = range - index;
    let up: Point = travel(point, &Direction::Up, index);
    let down: Point = travel(point, &Direction::Down, index);
    let up_ring: HashSet<Point> = ring::base(&up, diff);
    let down_ring: HashSet<Point> = ring::base(&down, diff);

    set.extend(up_ring);
    set.extend(down_ring);
  }

  set.insert(travel(point, &Direction::Up, range));
  set.insert(travel(point, &Direction::Down, range));

  set
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn of() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = super::of(&point, 2);

    assert!(set.contains(&Point(1, 0, 5)));
    assert!(set.contains(&Point(2, 0, 5)));
    assert!(set.contains(&Point(3, 0, 5)));
    assert!(set.contains(&Point(3, 1, 5)));
    assert!(set.contains(&Point(3, 2, 5)));
    assert!(set.contains(&Point(2, 3, 5)));
    assert!(set.contains(&Point(1, 4, 5)));
    assert!(set.contains(&Point(0, 4, 5)));
    assert!(set.contains(&Point(-1, 4, 5)));
    assert!(set.contains(&Point(-1, 3, 5)));
    assert!(set.contains(&Point(-1, 2, 5)));
    assert!(set.contains(&Point(0, 1, 5)));

    assert!(set.contains(&Point(1, 1, 6)));
    assert!(set.contains(&Point(2, 1, 6)));
    assert!(set.contains(&Point(2, 2, 6)));
    assert!(set.contains(&Point(1, 3, 6)));
    assert!(set.contains(&Point(0, 3, 6)));
    assert!(set.contains(&Point(0, 2, 6)));

    assert!(set.contains(&Point(1, 1, 4)));
    assert!(set.contains(&Point(2, 1, 4)));
    assert!(set.contains(&Point(2, 2, 4)));
    assert!(set.contains(&Point(1, 3, 4)));
    assert!(set.contains(&Point(0, 3, 4)));
    assert!(set.contains(&Point(0, 2, 4)));

    assert!(set.contains(&Point(1, 2, 7)));
    assert!(set.contains(&Point(1, 2, 3)));
    assert!(set.len() == 26);
  }
}
