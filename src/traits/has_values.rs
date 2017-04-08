use std::ops::{Add, Sub, Neg};

/// Provide access to the point's coordinate values
pub trait HasValues<T = i32>
  where T: Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy {

  /// Return a tuple of (QRT)
  fn values(&self) -> (T, T, T);

  /// Return a tuple of (QR)
  fn values_2d(&self) -> (T, T) {
    let (q, r, _) = self.values();

    (q, r)
  }

  /// Return a tuple of (QRST)
  fn values_cube(&self) -> (T, T, T, T) {
    let (q, r, t) = self.values();
    let s = -q - r;

    (q, r, s, t)
  }

  /// Return a tuple of (QRS)
  fn values_cube_2d(&self) -> (T, T, T) {
    let (q, r, s, _) = self.values_cube();

    (q, r, s)
  }

}

#[cfg(test)]
mod tests {
  use super::*;
  use structs::Point;

  #[test]
  fn values_2d() {
    let (q, r) = Point::new(1, 2, 5).values_2d();

    assert!(1 == q);
    assert!(2 == r);
  }

  #[test]
  fn values_cube() {
    let (q, r, s, t) = Point::new(1, 2, 5).values_cube();

    assert!( 1 == q);
    assert!( 2 == r);
    assert!(-3 == s);
    assert!( 5 == t);
  }

  #[test]
  fn values_cube_2d() {
    let (q, r, s) = Point::new(1, 2, 5).values_cube_2d();

    assert!( 1 == q);
    assert!( 2 == r);
    assert!(-3 == s);
  }
}
