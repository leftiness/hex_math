use structs::Point;

/// Stop the line if the index is out of range
#[derive(Debug)]
pub struct Range(pub usize);

impl Range {
  /// Determine if the point is out of range
  pub fn apply(&mut self, args: (usize, Point)) -> Option<(usize, Point)> {
    let &mut Range(range) = self;
    let (index, next) = args;

    match index > range {
      true => None,
      false => Some((index, next)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const POINT: &'static Point = &Point(1, 2, 5);

  #[test]
  fn apply_greater() {
    assert!(Range(5).apply((6, *POINT)).is_none());
  }

  #[test]
  fn apply_lesser() {
    assert!(Range(5).apply((4, *POINT)).is_some());
  }
}
