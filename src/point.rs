//! Basic point on a coordinate plane

use std::ops::{Add, Sub};
use std::collections::HashSet;
use std::cmp::{max, min};

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

  /// Create a point which is relatively northwest a specified number of units
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.northwest(2);
  ///
  /// assert_eq!((1, 0, 5), other.values());
  /// ```
  pub fn northwest(&self, units: i32) -> Point {
    Point::new(self.q, self.r - units, self.t)
  }

  /// Create a point which is relatively west a specified number of units
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.west(2);
  ///
  /// assert_eq!((-1, 2, 5), other.values());
  /// ```
  pub fn west(&self, units: i32) -> Point {
    Point::new(self.q - units, self.r, self.t)
  }

  /// Create a point which is relatively southwest a specified number of units
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.southwest(2);
  ///
  /// assert_eq!((-1, 4, 5), other.values());
  /// ```
  pub fn southwest(&self, units: i32) -> Point {
    Point::new(self.q - units, self.r + units, self.t)
  }

  /// Create a point which is relatively southeast a specified number of units
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.southeast(2);
  ///
  /// assert_eq!((1, 4, 5), other.values());
  /// ```
  pub fn southeast(&self, units: i32) -> Point {
    Point::new(self.q, self.r + units, self.t)
  }

  /// Create a point which is relatively east a specified number of units
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.east(2);
  ///
  /// assert_eq!((3, 2, 5), other.values());
  /// ```
  pub fn east(&self, units: i32) -> Point {
    Point::new(self.q + units, self.r, self.t)
  }

  /// Create a point which is relatively northeast a specified number of units
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.northeast(2);
  ///
  /// assert_eq!((3, 0, 5), other.values());
  /// ```
  pub fn northeast(&self, units: i32) -> Point {
    Point::new(self.q + units, self.r - units, self.t)
  }

  /// Create a point which is relatively up a specified number of units
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.up(2);
  ///
  /// assert_eq!((1, 2, 7), other.values());
  /// ```
  pub fn up(&self, units: i32) -> Point {
    Point::new(self.q, self.r, self.t + units)
  }

  /// Create a point which is relatively down a specified number of units
  ///
  /// # Example
  ///
  /// ```
  /// # use hex_math::point::Point;
  /// #
  /// let spot: Point = Point::new(1, 2, 5);
  /// let other: Point = spot.down(2);
  ///
  /// assert_eq!((1, 2, 3), other.values());
  /// ```
  pub fn down(&self, units: i32) -> Point {
    Point::new(self.q, self.r, self.t - units)
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
      let spot: Point = point_round(point_lerp(self, other, t));

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
  pub fn range(self, range: i32) -> HashSet<Point> {
    let mut set: HashSet<Point> = self.range_2d(range);

    for index in 1..range + 1 {
      let diff = range - index;
      let down: HashSet<Point> = self.up(index).range_2d(diff);
      let up: HashSet<Point> = self.down(index).range_2d(diff);

      set.extend(down);
      set.extend(up);
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
  pub fn range_2d(self, range: i32) -> HashSet<Point> {
    let mut set: HashSet<Point> = HashSet::new();

    for dq in -range..range + 1 {
      let lower: i32 = max(-range, -dq - range);
      let upper: i32 = min(range, -dq + range);

      for ds in lower..upper + 1 {
        let dr: i32 = -dq - ds;
        let point = self + Point::new(dq, dr, 0);

        set.insert(point);
      }

    }

    set
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

/// Linear interpolation of floats with specified offset
fn lerp(a: i32, b: i32, t: f32, o: f32) -> f32 {
  let a = a as f32 + o;
  let b = b as f32 + o;

  a + ((b - a) * t)
}

/// Linear interpolation of points with small offset
///
/// The offset is used to prevent the interpolation from falling exactly
/// on a border between two points. It is eliminated with rounding later.
fn point_lerp(a: Point, b: Point, t: f32) -> (f32, f32, f32, f32) { (
  lerp(a.q, b.q, t, 1e-6),
  lerp(a.r, b.r, t, 1e-6),
  lerp(a.s, b.s, t, -2e-6),
  lerp(a.t, b.t, t, 1e-6),
) }

/// Round a float point back to a standard point
fn point_round((q, r, s, t): (f32, f32, f32, f32)) -> Point {
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

