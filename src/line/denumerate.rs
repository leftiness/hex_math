/// Reverse the effects of enumerating an iterator.
pub fn denumerate<T>((_, element): (usize, T)) -> T {
  element
}

#[cfg(test)]
mod tests {
  #[test]
  fn denumerate() {
    assert!(1 == super::denumerate((0 as usize, 1)));
  }
}
