use std::borrow::Borrow;
use std::collections::HashSet;

use enums::Direction;
use structs::Point;
use travel::travel;

/// Find points at the same height in a ring of a provided radius
pub fn base<T: Borrow<Point>>(point: &T, range: i32) -> HashSet<Point> {
  let mut set: HashSet<Point> = HashSet::new();
  let mut point: Point = travel(point, &Direction::Northwest, range);

  for direction in Direction::to_vec().iter().take(6) {
    for _ in 0..range {
      let next: Point = travel(&point, direction, 1);

      set.insert(point.into());
      point = next;
    }
  }

  set
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn base() {
    let point: Point = Point(1, 2, 5);
    let set: HashSet<Point> = super::base(&point, 1);

    assert!(set.contains(&Point(1, 1, 5)));
    assert!(set.contains(&Point(2, 1, 5)));
    assert!(set.contains(&Point(2, 2, 5)));
    assert!(set.contains(&Point(1, 3, 5)));
    assert!(set.contains(&Point(0, 3, 5)));
    assert!(set.contains(&Point(0, 2, 5)));
    assert!(set.len() == 6);
  }
}
