#[macro_export]
macro_rules! assert_error_with {
  ($result:expr, $matcher:pat => $match_result:expr, $func:expr) => {
    {
      match $result {
        $matcher => $func($match_result),
        other @ Err(..) => panic!("Unexpected Err(..) variant, expected: {:?}, got: {:?}", stringify!($matcher), other),
        other @ Ok(..) => panic!("Expected Err(..) but got Ok(..): {:?}", other)
      }
    }
  }
}
