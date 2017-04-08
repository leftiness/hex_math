use traits::HasValues;
use structs::Point;

/// Rotate the point around a provided center, keeping the same height
///
/// Positive rotations are clockwise. Six rotations bring a point back to the
/// starting position.
pub fn rotate_2d<T: HasValues>(point: &T, center: &T, mut times: i32) -> Point {

  let point: Point = Point::from(point.values());
  let center: Point = Point::from(center.values());

  if &point == &center {
    return point;
  }

  let relative_point: Point = &point - &center;
  let (q, r, s, t) = relative_point.values_cube();

  times %= 6;

  if times < 0 {
    times += 6;
  }

  let rotated_point: Point = match times {
    0 => relative_point,
    1 => Point::new(-r, -s, t),
    2 => Point::new(s, q, t),
    3 => Point::new(-q, -r, t),
    4 => Point::new(r, s, t),
    5 => Point::new(-s, -q, t),
    _ => panic!(),
  };

  let result: Point = &rotated_point + &center;

  result

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rotate_2d() {
    let point: Point = Point::new(1, 2, 5);
    let center: Point = Point::new(1, 1, 5);

    assert!(Point::new(0, 1, 5) == super::rotate_2d(&point, &center, 2));
  }
}
