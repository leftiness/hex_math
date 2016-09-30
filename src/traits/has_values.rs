use point::Point;

/// Provide access to the point's coordinate values
pub trait HasValues {

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
  fn values(&self) -> (i32, i32, i32);

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
  fn values_2d(&self) -> (i32, i32) {
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
  fn values_cube(&self) -> (i32, i32, i32, i32) {
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
  fn values_cube_2d(&self) -> (i32, i32, i32) {
    let (q, r, s, _) = self.values_cube();

    (q, r, s)
  }

  /// Return a point from QRT
  ///
  /// ```
  /// use hex_math::{Point, HasValues};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  /// let other: Point = point.to_point();
  ///
  /// assert_eq!((1, 2, 5), other.values());
  /// ```
  ///
  /// ```
  /// use hex_math::{Point, Prism, HasValues};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  /// let prism: Prism = Prism::new(point, 1, 1, 1, 1);
  /// let other: Point = prism.to_point();
  ///
  /// assert_eq!((1, 2, 5), prism.values());
  /// assert_eq!((1, 2, 5), other.values());
  /// ```
  fn to_point(&self) -> Point {
    Point::from(self.values())
  }

  /// Return a vec of values
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Point, HasValues};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  /// let vec: Vec<i32> = point.into_vec();
  ///
  /// assert_eq!(1, vec[0]);
  /// assert_eq!(2, vec[1]);
  /// assert_eq!(5, vec[2]);
  /// ```
  fn into_vec(&self) -> Vec<i32> {
    let (q, r, t) = self.values();

    vec![q, r, t]
  }

  /// Return a vec of values_2d
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Point, HasValues};
  ///
  /// let point: Point = Point::new_2d(1, 2);
  /// let vec: Vec<i32> = point.into_vec_2d();
  ///
  /// assert_eq!(1, vec[0]);
  /// assert_eq!(2, vec[1]);
  /// ```
  fn into_vec_2d(&self) -> Vec<i32> {
    let (q, r) = self.values_2d();

    vec![q, r]
  }

  /// Return a vec of values_cube
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Point, HasValues};
  ///
  /// let point: Point = Point::new(1, 2, 5);
  /// let vec: Vec<i32> = point.into_vec_cube();
  ///
  /// assert_eq!(1, vec[0]);
  /// assert_eq!(2, vec[1]);
  /// assert_eq!(-3, vec[2]);
  /// assert_eq!(5, vec[3]);
  /// ```
  fn into_vec_cube(&self) -> Vec<i32> {
    let (q, r, s, t) = self.values_cube();

    vec![q, r, s, t]
  }

  /// Return a vec of values_cube_2d
  ///
  /// # Example
  ///
  /// ```
  /// use hex_math::{Point, HasValues};
  ///
  /// let point: Point = Point::new_2d(1, 2);
  /// let vec: Vec<i32> = point.into_vec_cube_2d();
  ///
  /// assert_eq!(1, vec[0]);
  /// assert_eq!(2, vec[1]);
  /// assert_eq!(-3, vec[2]);
  /// ```
  fn into_vec_cube_2d(&self) -> Vec<i32> {
    let (q, r, s) = self.values_cube_2d();

    vec![q, r, s]
  }

}

