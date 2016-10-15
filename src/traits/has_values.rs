use std::ops::{Add, Sub, Neg};

/// Provide access to the point's coordinate values
pub trait HasValues<T = i32>
  where T: Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy {

  /// Return a tuple of (QRT)
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Point, HasValues};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  ///
  /// assert_eq!((1, 2, 5), point.values());
  /// ```
  fn values(&self) -> (T, T, T);

  /// Return a tuple of (QR)
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Point, HasValues};
  ///
  /// let point: Point = Point::new_2d(1, 2);
  ///
  /// assert_eq!((1, 2), point.values_2d());
  /// ```
  fn values_2d(&self) -> (T, T) {
    let (q, r, _) = self.values();

    (q, r)
  }

  /// Return a tuple of (QRST)
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Point, HasValues};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  ///
  /// assert_eq!((1, 2, -3, 5), point.values_cube());
  /// ```
  fn values_cube(&self) -> (T, T, T, T) {
    let (q, r, t) = self.values();
    let s = -q - r;

    (q, r, s, t)
  }

  /// Return a tuple of (QRS)
  ///
  /// ```
  /// use hex_math::{Point, HasValues};
  ///
  /// let point: Point = Point::new_2d(1, 2);
  ///
  /// assert_eq!((1, 2, -3), point.values_cube_2d());
  /// ```
  fn values_cube_2d(&self) -> (T, T, T) {
    let (q, r, s, _) = self.values_cube();

    (q, r, s)
  }

}

