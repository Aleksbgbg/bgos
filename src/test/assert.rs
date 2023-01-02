#[macro_export]
macro_rules! assert_starts_with {
  ($prefix:expr, $string:expr) => {
    for index in 0..$prefix.len() {
      assert_eq!(
        $prefix.chars().nth(index).unwrap(),
        $string.chars().nth(index).unwrap(),
        r#"line "{}" does not start with "{}" (mismatch at index {})"#,
        $string,
        $prefix,
        index
      );
    }
  };
}
