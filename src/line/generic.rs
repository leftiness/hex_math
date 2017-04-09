use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

use distance;
use structs::{FloatPoint, Point, Prism};
use traits::IsPointMap;

/// Return the floats one step along a line between two points
///
/// The lerp is offset a small amount to prevent points from landing
/// directly on the line between two hexes.
fn step_size<T: Borrow<Point>>(point: &T, other: &T) -> FloatPoint {
  let &Point(q0, r0, t0) = point.borrow();
  let &Point(q1, r1, t1) = other.borrow();

  let distance = if (q0, r0) == (q1, r1) {
    distance::height(point, other)
  } else {
    distance::base(point, other)
  };

  let step = (distance as f32).recip();
  let lerp = |x: i32, y: i32| 1e-6 + (y - x) as f32 * step;
  let result = FloatPoint(lerp(q0, q1), lerp(r0, r1), lerp(t0, t1));

  result

}

/// Find the points in a line between two points
///
/// Optionally provide a range. The line will end at that range.
///
/// Optionally provide a map with walls which are impassable.
pub fn generic<T: Borrow<Point>, U: Borrow<Prism>>(
  point: &T,
  other: &T,
  range: Option<i32>,
  map: Option<&HashMap<Point, U>>,
) -> HashSet<Point> {
  let mut set: HashSet<Point> = HashSet::new();

  let point = *point.borrow();
  let other = *other.borrow();

  set.insert(point);

  if point == other {
    return set;
  }

  let has_wall_between = |p0: &Point, p1: &Point| match map {
    Some(map) => map.has_wall_between(p0, p1),
    None => false,
  };

  let mut round: Point = point;
  let mut step: FloatPoint = point.into();
  let mut last: Point = point;

  let distance: i32 = distance::with_height(&point, &other);
  let range: i32 = range.unwrap_or(distance);
  let size: FloatPoint = step_size(&point, &other);

  for _ in 0 .. range {
    if &round == &last {
      step = &step + &size;
      round = step.round();
    }

    let Point(_, _, tdiff) = &round - &last;
    let need_vertical_step = tdiff != 0;

    let current: Point = if need_vertical_step {
      &last + &Point(0, 0, tdiff.signum())
    } else {
      round
    };

    if has_wall_between(&last, &current) {
      break;
    }

    last = current;
    set.insert(current);
  }

  set
}

#[cfg(test)]
mod tests {
  use super::*;
  use structs::FloatPoint;

  #[test]
  fn step_size() {
    let point: Point = Point(1, 2, 5);
    let other: Point = Point(1, 12, 5);
    let FloatPoint(q, r, t) = super::step_size(&point, &other);

    assert!(1e-6 == q);
    assert!(1f32 + 1e-6 == r);
    assert!(1e-6 == t);
  }
}
