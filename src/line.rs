//! Useful stuff for finding points in a line

use std::collections::HashSet;

use point::Point;
use distance::distance_2d;

/// Find the points in a line between the current point and the one provided
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::Point;
/// use hex_math::line;
///
/// let spot: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let set: HashSet<Point> = line(&spot, &other);
///
/// assert_eq!(set.len(), 5);
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 6)));
/// assert!(set.contains(&Point::new(2, 3, 8)));
/// assert!(set.contains(&Point::new(3, 3, 9)));
/// assert!(set.contains(&Point::new(3, 4, 10)));
/// ```
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::Point;
/// use hex_math::line;
///
/// let spot: Point = Point::new_2d(1, 2);
/// let other: Point = Point::new_2d(3, 4);
/// let set: HashSet<Point> = line(&spot, &other);
///
/// assert_eq!(set.len(), 5);
/// assert!(set.contains(&Point::new_2d(1, 2)));
/// assert!(set.contains(&Point::new_2d(2, 2)));
/// assert!(set.contains(&Point::new_2d(2, 3)));
/// assert!(set.contains(&Point::new_2d(3, 3)));
/// assert!(set.contains(&Point::new_2d(3, 4)));
/// ```
pub fn line(point: &Point, other: &Point) -> HashSet<Point> {
  let distance: i32 = distance_2d(&point, &other);
  let mut set: HashSet<Point> = HashSet::new();

  for index in 0..distance + 1 {
    let t: f32 = index as f32 / distance as f32;
    let spot: Point = util::point_round(util::point_lerp(&point, &other, t));

    set.insert(spot);
  }

  set
}

mod util {
  use point::Point;

  /// Linear interpolation of floats with specified offset
  pub fn lerp(a: i32, b: i32, t: f32, o: f32) -> f32 {
    let a = a as f32 + o;
    let b = b as f32 + o;

    a + ((b - a) * t)
  }

  /// Linear interpolation of points with small offset
  ///
  /// The offset is used to prevent the interpolation from falling exactly
  /// on a border between two points. It is eliminated with rounding later.
  pub fn point_lerp(a: &Point, b: &Point, t: f32) -> (f32, f32, f32, f32) { (
    lerp(a.q, b.q, t, 1e-6),
    lerp(a.r, b.r, t, 1e-6),
    lerp(a.s, b.s, t, -2e-6),
    lerp(a.t, b.t, t, 1e-6),
  ) }

  /// Round a float point back to a standard point
  pub fn point_round((q, r, s, t): (f32, f32, f32, f32)) -> Point {
    let mut rq = q.round();
    let mut rr = r.round();

    let rs = s.round();
    let rt = t.round();

    let dq = (rq - q).abs();
    let dr = (rr - r).abs();
    let ds = (rs - s).abs();

    if (dq > ds) && (dq > dr) {
      rq = -rs - rr;
    } else if ds < dr {
      rr = -rq - rs;
    }

    let spot: Point = Point::new(rq as i32, rr as i32, rt as i32);

    spot
  }

}

#[cfg(test)]
mod tests {
  use point::Point;

  use super::util;

  #[test]
  fn lerp() {
    let result = util::lerp(1, 2, 0.5, 1e-6);

    assert_eq!(result, 1.5 + 1e-6);
  }

  #[test]
  fn point_lerp() {
    let spot: Point = Point::new(1, 2, 5);
    let other: Point = Point::new(3, 4, 10);
    let result = util::point_lerp(&spot, &other, 0.5);
    let expect = (2f32 + 1e-6, 3f32 + 1e-6, -5f32 - 2e-6, 7.5f32 + 1e-6);

    assert_eq!(result, expect);
  }

  #[test]
  fn point_round() {
    let coordinates = (1.6, 1.6, -3.2, 2.5);
    let spot: Point = util::point_round(coordinates);

    assert_eq!(spot, Point::new(2, 1, 3));
  }

}

