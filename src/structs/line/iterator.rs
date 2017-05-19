use std::borrow::Borrow;
use std::iter;

use structs::Point;
use traits::distance::{Base, Height};

/// A line iterator returns points along a line through two points
pub struct Iterator {
  start: Point,
  current: Point,
  round_target: Point,
  target: Point<f32>,
  step_size: Point<f32>,
  going_nowhere: bool,
  returned_start: bool,
}

impl Iterator {
  /// Create a new line iterator
  pub fn new<T: Borrow<Point>, U: Borrow<Point>>(
    start: &T,
    end: &U,
  ) -> Iterator {
    let start = *start.borrow();
    let end = *end.borrow();

    Iterator {
      start: start,
      current: start,
      round_target: start,
      target: start.into(),
      step_size: Self::step_size(&start, &end),
      going_nowhere: start == end,
      returned_start: false,
    }
  }

  /// Return the floats one step along a line between two points
  ///
  /// The lerp is offset a small amount to prevent points from landing
  /// directly on the line between two hexes.
  fn step_size(start: &Point, end: &Point) -> Point<f32> {
    let &Point(q0, r0, t0) = start;
    let &Point(q1, r1, t1) = end;

    let distance = if (q0, r0) == (q1, r1) {
      start.height(end)
    } else {
      start.base_distance(end)
    };

    let step = (distance as f32).recip();
    let lerp = |x: i32, y: i32| 1e-6 + (y - x) as f32 * step;

    Point(lerp(q0, q1), lerp(r0, r1), lerp(t0, t1))
  }

}

impl iter::Iterator for Iterator {
  type Item = Point;

  /// Find the next point in the line
  fn next(&mut self) -> Option<Point> {
    if !self.returned_start {
      self.returned_start = true;

      return Some(self.start);
    }

    if self.going_nowhere {
      return None;
    }

    if self.round_target == self.current {
      self.target = &self.target + &self.step_size;
      self.round_target = self.target.round();
    }

    self.current = match self.round_target.height(&self.current) {
      0 => self.round_target,
      height => &self.current + &Point(0, 0, height.signum())
    };

    Some(self.current)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const START: &'static Point = &Point(1, 2, 5);
  const END: &'static Point = &Point(1, 12, 5);

  #[test]
  fn next_return_start() {
    assert!(*START == Iterator::new(START, END).nth(0).unwrap());
  }

  #[test]
  fn next_going_nowhere() {
    assert!(Iterator::new(START, START).nth(1).is_none());
  }

  #[test]
  fn next_returns_line() {
    let mut iter = Iterator::new(START, &Point(3, 4, 10));

    assert!(*START == iter.next().unwrap());
    assert!(Point(1, 2, 6) == iter.next().unwrap());
    assert!(Point(2, 2, 6) == iter.next().unwrap());
    assert!(Point(2, 2, 7) == iter.next().unwrap());
    assert!(Point(2, 2, 8) == iter.next().unwrap());
    assert!(Point(2, 3, 8) == iter.next().unwrap());
    assert!(Point(2, 3, 9) == iter.next().unwrap());
    assert!(Point(3, 3, 9) == iter.next().unwrap());
    assert!(Point(3, 3, 10) == iter.next().unwrap());
    assert!(Point(3, 4, 10) == iter.next().unwrap());
  }

  #[test]
  fn step_size() {
    let Point(q, r, t) = Iterator::step_size(START, END);

    assert!(1e-6 == q);
    assert!(1f32 + 1e-6 == r);
    assert!(1e-6 == t);
  }

  #[test]
  fn step_size_vertical() {
    let Point(q, r, t) = Iterator::step_size(START, &Point(1, 2, 10));

    assert!(1e-6 == q);
    assert!(1e-6 == r);
    assert!(1f32 + 1e-6 == t);
  }
}
