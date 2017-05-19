use std::borrow::Borrow;

use enums::Direction;
use enums::Direction::*;
use structs::Point;

/// Trait wrapping travel implementation
pub trait Travel: Borrow<Point> {
  /// Create a point in the specified direction a specified number of units
  fn travel(&self, direction: &Direction, units: i32) -> Point;
}

impl<T> Travel for T where T: Borrow<Point> {
  fn travel(&self, direction: &Direction, units: i32) -> Point {
    let &Point(q, r, t) = self.borrow();

    return match direction {
      &East      => Point(q + units, r        , t        ),
      &Southeast => Point(q        , r + units, t        ),
      &Southwest => Point(q - units, r + units, t        ),
      &West      => Point(q - units, r        , t        ),
      &Northwest => Point(q        , r - units, t        ),
      &Northeast => Point(q + units, r - units, t        ),
      &Up        => Point(q        , r        , t + units),
      &Down      => Point(q        , r        , t - units),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn travel() {
    let point: Point = Point(1, 2, 5);

    assert!(Point( 3, 2, 5) == point.travel(&East     , 2));
    assert!(Point( 1, 4, 5) == point.travel(&Southeast, 2));
    assert!(Point(-1, 4, 5) == point.travel(&Southwest, 2));
    assert!(Point(-1, 2, 5) == point.travel(&West     , 2));
    assert!(Point( 1, 0, 5) == point.travel(&Northwest, 2));
    assert!(Point( 3, 0, 5) == point.travel(&Northeast, 2));
    assert!(Point( 1, 2, 7) == point.travel(&Up       , 2));
    assert!(Point( 1, 2, 3) == point.travel(&Down     , 2));
  }
}
