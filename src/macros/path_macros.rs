#[macro_export]
macro_rules! spath {
  ($expr:expr) => {
    {
      &$expr.to_string_lossy().to_string()
    }
  };

  ($expr:expr, $t:ty) => {
    {
      &<$t as AsRef<Path>>::as_ref($expr).to_string_lossy().to_string()
    }
  };
}
