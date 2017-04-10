use structs::Point;
use traits::Predicate;

/// Stop the line if the index is out of range
#[derive(Debug)]
pub struct Range(pub i32);

impl Predicate<(i32, Point, Point)> for Range {
  fn apply(&self, args: &(i32, Point, Point)) -> bool {
    let &Range(range) = self;
    let &(index, _, _) = args;

    index >= range
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const POINT: &'static Point = &Point(1, 2, 5);
  const OTHER: &'static Point = &Point(3, 4, 10);

  #[test]
  fn apply_greater() {
    assert!(Range(5).apply(&(6, *POINT, *OTHER)));
  }

  #[test]
  fn apply_lesser() {
    assert!(!Range(5).apply(&(4, *POINT, *OTHER)));
  }
}
