//! Useful stuff for finding points in a line

use std::collections::HashSet;

use traits::has_values::HasValues;
use point::Point;

/// Find the points in a line between the current point and the one provided
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{line, Point};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let set: HashSet<Point> = line(&point, &other);
///
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 6)));
/// assert!(set.contains(&Point::new(2, 3, 8)));
/// assert!(set.contains(&Point::new(3, 3, 9)));
/// assert!(set.contains(&Point::new(3, 4, 10)));
/// assert_eq!(set.len(), 5);
/// ```
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{line, Point};
///
/// let point: Point = Point::new_2d(1, 2);
/// let other: Point = Point::new_2d(3, 4);
/// let set: HashSet<Point> = line(&point, &other);
///
/// assert!(set.contains(&Point::new_2d(1, 2)));
/// assert!(set.contains(&Point::new_2d(2, 2)));
/// assert!(set.contains(&Point::new_2d(2, 3)));
/// assert!(set.contains(&Point::new_2d(3, 3)));
/// assert!(set.contains(&Point::new_2d(3, 4)));
/// assert_eq!(set.len(), 5);
/// ```
pub fn line<T: HasValues>(point: &T, other: &T) -> HashSet<Point> {
  util::line(point, other, None, None)
}

/// Find the points within range in a line through two points
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{line_through, Point};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(2, 2, 6);
/// let set: HashSet<Point> = line_through(&point, &other, 3);
///
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 6)));
/// assert!(set.contains(&Point::new(3, 2, 7)));
/// assert!(set.contains(&Point::new(4, 2, 8)));
/// assert_eq!(set.len(), 4);
/// ```
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{line_through, Point};
///
/// let point: Point = Point::new_2d(1, 2);
/// let other: Point = Point::new_2d(2, 2);
/// let set: HashSet<Point> = line_through(&point, &other, 3);
///
/// assert!(set.contains(&Point::new_2d(1, 2)));
/// assert!(set.contains(&Point::new_2d(2, 2)));
/// assert!(set.contains(&Point::new_2d(3, 2)));
/// assert!(set.contains(&Point::new_2d(4, 2)));
/// assert_eq!(set.len(), 4);
/// ```
pub fn line_through<T: HasValues>(
  point: &T,
  other: &T,
  range: i32,
) -> HashSet<Point> {
  util::line(point, other, Some(range), None)
}

/// Find unblocked points in a line between two points
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{ray, Point};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let mut opaque: HashSet<Point> = HashSet::new();
///
/// opaque.insert(Point::new(3, 3, 9));
///
/// let set: HashSet<Point> = ray(&point, &other, &opaque);
///
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 6)));
/// assert!(set.contains(&Point::new(2, 3, 8)));
/// assert!(set.contains(&Point::new(3, 3, 9)));
/// assert_eq!(set.len(), 4);
/// ```
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{ray, Point};
///
/// let point: Point = Point::new_2d(1, 2);
/// let other: Point = Point::new_2d(3, 4);
/// let mut opaque: HashSet<Point> = HashSet::new();
///
/// opaque.insert(Point::new_2d(3, 3));
///
/// let set: HashSet<Point> = ray(&point, &other, &opaque);
///
/// assert!(set.contains(&Point::new_2d(1, 2)));
/// assert!(set.contains(&Point::new_2d(2, 2)));
/// assert!(set.contains(&Point::new_2d(2, 3)));
/// assert!(set.contains(&Point::new_2d(3, 3)));
/// assert_eq!(set.len(), 4);
/// ```
pub fn ray<T: HasValues>(
  point: &T,
  other: &T,
  opaque: &HashSet<Point>,
) -> HashSet<Point> {
  util::line(point, other, None, Some(opaque))
}

/// Find unblocked points within range in a line through two points
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{ray_through, Point};
///
/// let point: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(2, 2, 6);
/// let mut opaque: HashSet<Point> = HashSet::new();
///
/// opaque.insert(Point::new(3, 2, 7));
///
/// let set: HashSet<Point> = ray_through(&point, &other, 3, &opaque);
///
/// assert!(set.contains(&Point::new(1, 2, 5)));
/// assert!(set.contains(&Point::new(2, 2, 6)));
/// assert!(set.contains(&Point::new(3, 2, 7)));
/// assert_eq!(set.len(), 3);
/// ```
///
/// ```
/// use std::collections::HashSet;
///
/// use hex_math::{ray_through, Point};
///
/// let point: Point = Point::new_2d(1, 2);
/// let other: Point = Point::new_2d(2, 2);
/// let mut opaque: HashSet<Point> = HashSet::new();
///
/// opaque.insert(Point::new_2d(3, 2));
///
/// let set: HashSet<Point> = ray_through(&point, &other, 3, &opaque);
///
/// assert!(set.contains(&Point::new_2d(1, 2)));
/// assert!(set.contains(&Point::new_2d(2, 2)));
/// assert!(set.contains(&Point::new_2d(3, 2)));
/// assert_eq!(set.len(), 3);
/// ```
pub fn ray_through<T: HasValues>(
  point: &T,
  other: &T,
  range: i32,
  opaque: &HashSet<Point>,
) -> HashSet<Point> {
  util::line(point, other, Some(range), Some(opaque))
}

mod util {
  use std::collections::HashSet;

  use traits::has_values::HasValues;
  use point::Point;
  use distance::{distance, distance_2d};

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
  pub fn point_lerp<T: HasValues>(
    a: &T,
    b: &T,
    t: f32
  ) -> (f32, f32, f32, f32) {
    let (qa, ra, sa, ta) = a.values_cube();
    let (qb, rb, sb, tb) = b.values_cube();

    let result = (
      lerp(qa, qb, t, 1e-6),
      lerp(ra, rb, t, 1e-6),
      lerp(sa, sb, t, -2e-6),
      lerp(ta, tb, t, 1e-6),
    );

    result
  }

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

    let point: Point = Point::new(rq as i32, rr as i32, rt as i32);

    point
  }

  /// Find the points in a line between two points
  ///
  /// Optionally provide a range. The line will end at that range.
  ///
  /// Optionally provide a set of opaque point which are impassable.
  pub fn line<T: HasValues>(
    point: &T,
    other: &T,
    range: Option<i32>,
    opaque: Option<&HashSet<Point>>,
  ) -> HashSet<Point> {
    let mut set: HashSet<Point> = HashSet::new();

    if point.values() == other.values() {
      set.insert(point.to_point());

      return set;
    }

    let distance: i32 = if point.values_2d() == other.values_2d() {
      distance(point, other)
    } else {
      distance_2d(point, other)
    };

    let empty: HashSet<Point> = HashSet::new();
    let opaque: &HashSet<Point> = opaque.unwrap_or(&empty);
    let should_check_opaque: bool = !opaque.is_empty();

    for index in 0..range.unwrap_or(distance) + 1 {
      let t: f32 = index as f32 / distance as f32;
      let lerp: (f32, f32, f32, f32) = point_lerp(point, other, t);
      let found: Point = point_round(lerp);
      let should_break = should_check_opaque && opaque.contains(&found);

      set.insert(found);

      if should_break {
        break;
      }
    }

    set
  }

}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use point::Point;

  use super::util;

  #[test]
  fn lerp() {
    let result = util::lerp(1, 2, 0.5, 1e-6);

    assert_eq!(result, 1.5 + 1e-6);
  }

  #[test]
  fn point_lerp() {
    let point: Point = Point::new(1, 2, 5);
    let other: Point = Point::new(3, 4, 10);
    let result = util::point_lerp(&point, &other, 0.5);
    let expect = (2f32 + 1e-6, 3f32 + 1e-6, -5f32 - 2e-6, 7.5f32 + 1e-6);

    assert_eq!(result, expect);
  }

  #[test]
  fn point_round() {
    let coordinates = (1.6, 1.6, -3.2, 2.5);
    let point: Point = util::point_round(coordinates);

    assert_eq!(point, Point::new(2, 1, 3));
  }

  #[test]
  fn line_vertical() {
    let point: Point = Point::new(1, 2, 5);
    let other: Point = Point::new(1, 2, 7);
    let line: HashSet<Point> = util::line(&point, &other, None, None);

    assert!(line.contains(&Point::new(1, 2, 5)));
    assert!(line.contains(&Point::new(1, 2, 6)));
    assert!(line.contains(&Point::new(1, 2, 7)));
  }

}

