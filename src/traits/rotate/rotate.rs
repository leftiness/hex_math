use std::borrow::Borrow;

use structs::Point;

/// Trait wrapping rotate implementation
pub trait Rotate: Borrow<Point> {
  /// Rotate the point around a provided center, keeping the same height
  ///
  /// Positive rotations are clockwise. Six rotations bring a point back to the
  /// starting position.
  fn rotate<T: Borrow<Point>>(&self, center: &T, times: i32) -> Point;
}

impl<T> Rotate for T where T: Borrow<Point> {
  fn rotate<U: Borrow<Point>>(&self, center: &U, times: i32) -> Point {
    let point = self.borrow();
    let center = center.borrow();

    if point == center {
      return *point;
    }

    let relative_point = point - center;
    let Point(q, r, t) = relative_point;
    let s = relative_point.s();

    let mut times = times % 6;

    if times < 0 {
      times += 6;
    }

    let rotated_point: Point = match times {
      0 => Point( q,  r, t),
      1 => Point(-r, -s, t),
      2 => Point( s,  q, t),
      3 => Point(-q, -r, t),
      4 => Point( r,  s, t),
      5 => Point(-s, -q, t),
      _ => panic!(),
    };

    let result: Point = &rotated_point + &center;

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rotate() {
    let point: Point = Point(1, 2, 5);
    let center: Point = Point(1, 1, 5);

    assert!(Point(0, 1, 5) == point.rotate(&center, 2));
  }
}
