#[macro_export]
macro_rules! s {
  ($lit:literal, $($args:tt)+) => {
    {
      format!($lit, $($args)+)
    }
  }
}
