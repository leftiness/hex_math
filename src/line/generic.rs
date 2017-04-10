use std::borrow::Borrow;
use std::collections::HashSet;

use distance;
use structs::{FloatPoint, Point};
use traits::Predicate;

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
/// Be careful with the infinite loop. If you provide a predicate which
/// always returns false, he'll keep following the line until he panics about
/// running out of 32-bit integers.
pub fn generic<T: Borrow<Point>, U: Predicate<(i32, Point, Point)>>(
  point: &T,
  other: &T,
  stop_condition: U,
) -> HashSet<Point> {
  let mut set: HashSet<Point> = HashSet::new();

  let point = *point.borrow();
  let other = *other.borrow();

  set.insert(point);

  if point == other {
    return set;
  }

  let mut round: Point = point;
  let mut step: FloatPoint = point.into();
  let mut current: Point = point;
  let mut index = 0;

  let size: FloatPoint = step_size(&point, &other);

  loop {
    if &round == &current {
      step = &step + &size;
      round = step.round();
    }

    let next: Point = match distance::height(&round, &current) {
      0 => round,
      height => &current + &Point(0, 0, height.signum())
    };

    if stop_condition.apply(&(index, current, next)) {
      break;
    }

    current = next;
    set.insert(next);
    index += 1;
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
