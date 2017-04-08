use enums::Direction;
use enums::Direction::*;
use structs::Point;
use traits::HasValues;

/// Create a point in the specified direction a specified number of units
pub fn travel<T: HasValues>(
  point: &T,
  direction: &Direction,
  units: i32
) -> Point {
  let (q, r, t) = point.values();

  return match direction {
    &East      => Point::new(q + units, r        , t        ),
    &Southeast => Point::new(q        , r + units, t        ),
    &Southwest => Point::new(q - units, r + units, t        ),
    &West      => Point::new(q - units, r        , t        ),
    &Northwest => Point::new(q        , r - units, t        ),
    &Northeast => Point::new(q + units, r - units, t        ),
    &Up        => Point::new(q        , r        , t + units),
    &Down      => Point::new(q        , r        , t - units),
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn travel() {
    let point: Point = Point::new(1, 2, 5);

    assert!(Point::new( 3, 2, 5) == super::travel(&point, &East     , 2));
    assert!(Point::new( 1, 4, 5) == super::travel(&point, &Southeast, 2));
    assert!(Point::new(-1, 4, 5) == super::travel(&point, &Southwest, 2));
    assert!(Point::new(-1, 2, 5) == super::travel(&point, &West     , 2));
    assert!(Point::new( 1, 0, 5) == super::travel(&point, &Northwest, 2));
    assert!(Point::new( 3, 0, 5) == super::travel(&point, &Northeast, 2));
    assert!(Point::new( 1, 2, 7) == super::travel(&point, &Up       , 2));
    assert!(Point::new( 1, 2, 3) == super::travel(&point, &Down     , 2));
  }

}
