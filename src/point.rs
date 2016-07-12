//! Basic point on a coordinate plane

use std::ops::{Add, Sub};
use std::collections::HashSet;
use std::cmp::{max, min};

use travel::{Direction, travel};

/// Basic point on a coordinate plane
///
/// The point contains three coordinates (QRS) to describe its position in
/// two dimensions and a fourth (T) to describe its third dimension.
/// The first three are "cube" coordinates as they describe a 2D hexagon as
/// if it were a cube in 3D space, making several algorithms easier to use.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {

  /// This can also be considered axis X on a cube.
  pub q: i32,

  /// This can also be considered axis Z on a cube.
  pub r: i32,

  /// This can also be considered axis Y on a cube.
  pub s: i32,

  /// This is the height of the point in 3D space.
  pub t: i32,
}

impl Point {

  /// Factory function for making new points
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// ```
  pub fn new(q: i32, r: i32, t: i32) -> Point {
    Point {q: q, r: r, s: -q - r, t: t}
  }

  /// Convenience function for making two-dimensional points
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new_2d(1, 2);
  /// ```
  pub fn new_2d(q: i32, r: i32) -> Point {
    Point::new(q, r, 0)
  }

  /// Convenient getter for the point's axial coordinate values
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  ///
  /// assert_eq!((1, 2, 5), spot.values());
  /// ```
  ///
  /// Those using two-dimensional points may simply ignore a value.
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new_2d(1, 2);
  /// let (q, r, _) = spot.values();
  ///
  /// assert_eq!((1, 2), (q, r));
  /// ```
  pub fn values(&self) -> (i32, i32, i32) {
    (self.q, self.r, self.t)
  }

  /// Convenient getter for the point's cube coordinate values
  ///
  /// # Exampes
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  ///
  /// assert_eq!((1, 2, -3, 5), spot.values_cube());
  /// ```
  pub fn values_cube(&self) -> (i32, i32, i32, i32) {
    (self.q, self.r, self.s, self.t)
  }

  /// Calculate the manhattan distance between two points
  ///
  /// Distance is rounded up to the next integer.
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new_2d(1, 2);
  /// let other: Point = Point::new_2d(3, 4);
  ///
  /// assert_eq!(4, spot.distance(other));
  /// ```
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = Point::new(3, 4, 10);
  ///
  /// assert_eq!(9, spot.distance(other));
  /// ```
  pub fn distance(self, other: Point) -> i32 {
    let diff: Point = self - other;
    let base = self.distance_2d(other);
    let height = diff.t.abs();

    base + height
  }

  /// Calculate the manhattan distance between two points ignoring height
  ///
  /// Distance is rounded up to the next integer.
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = Point::new(3, 4, 10);
  ///
  /// assert_eq!(4, spot.distance_2d(other));
  /// ```
  pub fn distance_2d(self, other: Point) -> i32 {
    let diff: Point = self - other;
    let distance = (diff.q.abs() + diff.r.abs() + diff.s.abs()) / 2;

    distance
  }

  /// Find the points in a line between the current point and the one provided
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// # use std::collections::HashSet;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = Point::new(3, 4, 10);
  /// let set: HashSet<Point> = spot.line(other);
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
  /// # use hex_math::point::Point;
  /// # use std::collections::HashSet;
  /// #
  /// let spot: Point = Point::new_2d(1, 2);
  /// let other: Point = Point::new_2d(3, 4);
  /// let set: HashSet<Point> = spot.line(other);
  ///
  /// assert_eq!(set.len(), 5);
  /// assert!(set.contains(&Point::new_2d(1, 2)));
  /// assert!(set.contains(&Point::new_2d(2, 2)));
  /// assert!(set.contains(&Point::new_2d(2, 3)));
  /// assert!(set.contains(&Point::new_2d(3, 3)));
  /// assert!(set.contains(&Point::new_2d(3, 4)));
  /// ```
  pub fn line(self, other: Point) -> HashSet<Point> {
    let distance: i32 = self.distance_2d(other);
    let mut set: HashSet<Point> = HashSet::new();

    for index in 0..distance + 1 {
      let t: f32 = index as f32 / distance as f32;
      let spot: Point = util::point_round(util::point_lerp(self, other, t));

      set.insert(spot);
    }

    set
  }

  /// Find the points within the provided manhattan distance
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// # use std::collections::HashSet;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let set: HashSet<Point> = spot.range(1);
  ///
  /// assert_eq!(set.len(), 9);
  /// assert!(set.contains(&Point::new(1, 2, 5)));
  /// assert!(set.contains(&Point::new(2, 2, 5)));
  /// assert!(set.contains(&Point::new(1, 3, 5)));
  /// assert!(set.contains(&Point::new(0, 3, 5)));
  /// assert!(set.contains(&Point::new(0, 2, 5)));
  /// assert!(set.contains(&Point::new(1, 1, 5)));
  /// assert!(set.contains(&Point::new(2, 1, 5)));
  /// assert!(set.contains(&Point::new(1, 2, 4)));
  /// assert!(set.contains(&Point::new(1, 2, 6)));
  /// ```
  pub fn range(&self, range: i32) -> HashSet<Point> {
    let mut set: HashSet<Point> = self.range_2d(range);

    for index in 1..range + 1 {
      let diff = range - index;
      let up: Point = travel(&self, Direction::Up, index);
      let down: Point = travel(&self, Direction::Down, index);
      let up_range: HashSet<Point> = up.range_2d(diff);
      let down_range: HashSet<Point> = down.range_2d(diff);

      set.extend(up_range);
      set.extend(down_range);
    }

    set
  }

  /// Find the points at the same height within the provided manhattan distance
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// # use std::collections::HashSet;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let set: HashSet<Point> = spot.range_2d(1);
  ///
  /// assert_eq!(set.len(), 7);
  /// assert!(set.contains(&Point::new(1, 2, 5)));
  /// assert!(set.contains(&Point::new(2, 2, 5)));
  /// assert!(set.contains(&Point::new(1, 3, 5)));
  /// assert!(set.contains(&Point::new(0, 3, 5)));
  /// assert!(set.contains(&Point::new(0, 2, 5)));
  /// assert!(set.contains(&Point::new(1, 1, 5)));
  /// assert!(set.contains(&Point::new(2, 1, 5)));
  /// ```
  pub fn range_2d(&self, range: i32) -> HashSet<Point> {
    let mut set: HashSet<Point> = HashSet::new();

    for dq in -range..range + 1 {
      let lower: i32 = max(-range, -dq - range);
      let upper: i32 = min(range, -dq + range);

      for ds in lower..upper + 1 {
        let dr: i32 = -dq - ds;
        let point = self.clone() + Point::new(dq, dr, 0);

        set.insert(point);
      }

    }

    set
  }

  /// Find reachable points within a specified range
  ///
  /// A point may be within range while being unreachable if the path to that
  /// point is blocked by an invalid point.
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// # use std::collections::HashSet;
  /// #
  /// let spot: Point = Point::new(1, 2, 2);
  /// let mut invalid: HashSet<Point> = HashSet::new();
  ///
  /// invalid.insert(Point::new(1, 1, 2));
  /// invalid.insert(Point::new(2, 1, 2));
  /// invalid.insert(Point::new(2, 2, 2));
  /// invalid.insert(Point::new(1, 3, 2));
  /// invalid.insert(Point::new(0, 3, 2));
  /// invalid.insert(Point::new(1, 2, 1));
  /// invalid.insert(Point::new(1, 2, 3));
  ///
  /// let result: HashSet<Point> = spot.flood(2, &invalid);
  ///
  /// assert_eq!(result.len(), 7);
  /// assert!(result.contains(&spot));
  /// assert!(result.contains(&Point::new(0, 2, 2)));
  /// assert!(result.contains(&Point::new(0, 1, 2)));
  /// assert!(result.contains(&Point::new(-1, 2, 2)));
  /// assert!(result.contains(&Point::new(-1, 3, 2)));
  /// assert!(result.contains(&Point::new(0, 2, 1)));
  /// assert!(result.contains(&Point::new(0, 2, 3)));
  /// ```
  pub fn flood(
    self,
    range: i32,
    invalid: &HashSet<Point>,
  ) -> HashSet<Point> {
    util::flood(self, range, Point::range, invalid)
  }

  /// Find reachable points of the same height within a specified range
  ///
  /// A point may be within range while being unreachable if the path to that
  /// point is blocked by an invalid point.
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// # use std::collections::HashSet;
  /// #
  /// let spot: Point = Point::new_2d(1, 2);
  /// let mut invalid: HashSet<Point> = HashSet::new();
  ///
  /// invalid.insert(Point::new_2d(1, 1));
  /// invalid.insert(Point::new_2d(2, 1));
  /// invalid.insert(Point::new_2d(2, 2));
  /// invalid.insert(Point::new_2d(1, 3));
  /// invalid.insert(Point::new_2d(0, 3));
  ///
  /// let result: HashSet<Point> = spot.flood_2d(2, &invalid);
  ///
  /// assert_eq!(result.len(), 5);
  /// assert!(result.contains(&spot));
  /// assert!(result.contains(&Point::new_2d(0, 2)));
  /// assert!(result.contains(&Point::new_2d(0, 1)));
  /// assert!(result.contains(&Point::new_2d(-1, 2)));
  /// assert!(result.contains(&Point::new_2d(-1, 3)));
  /// ```
  pub fn flood_2d(
    self,
    range: i32,
    invalid: &HashSet<Point>,
  ) -> HashSet<Point> {
    util::flood(self, range, Point::range_2d, invalid)
  }

  /// Rotate the point a specified number of times
  ///
  /// Positive rotations are clockwise.
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  ///
  /// assert_eq!(spot.rotate(1), Point::new(-2, 3, 5));
  /// assert_eq!(spot.rotate(2), Point::new(-3, 1, 5));
  /// assert_eq!(spot.rotate(3), Point::new(-1, -2, 5));
  /// assert_eq!(spot.rotate(4), Point::new(2, -3, 5));
  /// assert_eq!(spot.rotate(5), Point::new(3, -1, 5));
  /// assert_eq!(spot.rotate(6), spot);
  /// assert_eq!(spot.rotate(-1), Point::new(3, -1, 5));
  /// assert_eq!(spot.rotate(-2), Point::new(2, -3, 5));
  /// assert_eq!(spot.rotate(-3), Point::new(-1, -2, 5));
  /// assert_eq!(spot.rotate(-4), Point::new(-3, 1, 5));
  /// assert_eq!(spot.rotate(-5), Point::new(-2, 3, 5));
  /// assert_eq!(spot.rotate(-6), spot);
  /// ```
  pub fn rotate(self, mut times: i32) -> Point {

    let (q, r, s, t) = self.values_cube();

    if times < 0 {
      times += 6;
    }

    return match times % 6 {
      1 => Point::new(-r, -s, t),
      2 => Point::new(s, q, t),
      3 => Point::new(-q, -r, t),
      4 => Point::new(r, s, t),
      5 => Point::new(-s, -q, t),
      _ => self,
    }

  }

}

/// Add one point to another
///
/// # Example
///
/// ```
/// # use hex_math::point::Point;
/// #
/// let spot: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let result: Point = spot + other;
///
/// assert_eq!((4, 6, 15), result.values());
/// ```
impl Add for Point {

  type Output = Point;

  fn add(self, rhs: Point) -> Point {
    Point::new(self.q + rhs.q, self.r + rhs.r, self.t + rhs.t)
  }

}

/// Subtract one point from another
///
/// # Example
///
/// ```
/// # use hex_math::point::Point;
/// #
/// let spot: Point = Point::new(1, 2, 5);
/// let other: Point = Point::new(3, 4, 10);
/// let result: Point = spot - other;
///
/// assert_eq!((-2, -2, -5), result.values());
/// ```
impl Sub for Point {

  type Output = Point;

  fn sub(self, rhs: Point) -> Point {
    Point::new(self.q - rhs.q, self.r - rhs.r, self.t - rhs.t)
  }

}

mod util {
  use super::Point;
  use std::collections::HashSet;

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
  pub fn point_lerp(a: Point, b: Point, t: f32) -> (f32, f32, f32, f32) { (
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

  /// Find reachable points within a specified range with a provided function
  pub fn flood(
    start: Point,
    range: i32,
    range_fn: fn(&Point, i32) -> HashSet<Point>,
    invalid: &HashSet<Point>,
  ) -> HashSet<Point> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut fringes: Vec<Vec<Point>> = Vec::new();

    if invalid.contains(&start) {
      return visited;
    }

    fringes.push(vec![start]);
    visited.insert(start);

    for step in 1..range + 1 {
      let mut found = vec![];

      for point in &fringes[step as usize - 1] {
        for neighbor in range_fn(point, 1) {
          if !invalid.contains(&neighbor) && !visited.contains(&neighbor) {
            found.push(neighbor);
          }
        }
      }

      visited.extend(&found);
      fringes.push(found);
    }

    visited
  }

}

#[cfg(test)]
mod tests {
  #[allow(unused)]
  use travel::{Direction, travel};

  use super::Point;
  use super::util;

  use std::collections::HashSet;

  #[test]
  fn lerp() {
    let result = util::lerp(1, 2, 0.5, 1e-6);

    assert_eq!(result, 1.5 + 1e-6);
  }

  #[test]
  fn point_lerp() {
    let spot: Point = Point::new(1, 2, 5);
    let other: Point = Point::new(3, 4, 10);
    let result = util::point_lerp(spot, other, 0.5);
    let expect = (2f32 + 1e-6, 3f32 + 1e-6, -5f32 - 2e-6, 7.5f32 + 1e-6);

    assert_eq!(result, expect);
  }

  #[test]
  fn point_round() {
    let coordinates = (1.6, 1.6, -3.2, 2.5);
    let spot: Point = util::point_round(coordinates);

    assert_eq!(spot, Point::new(2, 1, 3));
  }

  #[test]
  fn flood() {
    let spot: Point = Point::new(0, 0, 0);
    let mut invalid: HashSet<Point> = HashSet::new();

    invalid.insert(Point::new(0, 0, 2));

    fn range_1d(point: &Point, range: i32) -> HashSet<Point> {
      let mut set: HashSet<Point> = HashSet::new();
      let up: Point = travel(&point, Direction::Up, range);
      let down: Point = travel(&point, Direction::Down, range);

      set.insert(up);
      set.insert(down);

      set
    };

    let result: HashSet<Point> = util::flood(spot, 2, range_1d, &invalid);

    assert_eq!(result.len(), 4);
    assert!(result.contains(&spot));
    assert!(result.contains(&Point::new(0, 0, 1)));
    assert!(result.contains(&Point::new(0, 0, -1)));
    assert!(result.contains(&Point::new(0, 0, -2)));
  }

}
