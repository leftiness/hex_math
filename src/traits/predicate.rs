/// Report if a condition is true
pub trait Predicate<T> {

  /// Apply the predicate to determine if it passed or failed
  fn apply(&self, args: &T) -> bool;
}

impl <T, U: Predicate<T>> Predicate<T> for Option<U> {
  /// Unwrap an optional predicate and apply him or else return false
  fn apply(&self, args: &T) -> bool {
    match self {
      &Some(ref predicate) => predicate.apply(args),
      &None => false
    }
  }
}

impl <T, U, V> Predicate<T> for (U, V)
  where U: Predicate<T>, V: Predicate<T> {

  /// Apply both predicates and return true if either passes
  fn apply(&self, args: &T) -> bool {
    let &(ref first, ref second) = self;

    first.apply(args) || second.apply(args)
  }
}

impl <T> Predicate<T> for bool {

  /// Always return the constant value.
  fn apply(&self, _: &T) -> bool {
    *self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn apply_some() {
    assert!(Some(true).apply(&()));
  }

  #[test]
  fn apply_none() {
    assert!(!None::<bool>.apply(&()));
  }

  #[test]
  fn apply_tuple_yes_no() {
    assert!((true, false).apply(&()));
  }

  #[test]
  fn apply_tuple_yes_yes() {
    assert!((true, true).apply(&()));
  }

  #[test]
  fn apply_tuple_no_yes() {
    assert!((false, true).apply(&()));
  }

  #[test]
  fn apply_tuple_no_no() {
    assert!(!(false, false).apply(&()));
  }

  #[test]
  fn apply_true() {
    assert!(true.apply(&()));
  }

  #[test]
  fn apply_false() {
    assert!(!false.apply(&()));
  }
}
