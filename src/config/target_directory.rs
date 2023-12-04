use std::{path::Path, ffi::OsStr};

#[derive(Debug, Clone, PartialEq)]
pub struct TargetDir {
  pub path: String
}

impl TargetDir {
  pub fn new(path: &str) -> Self {
    TargetDir {
      path: path.to_owned()
    }
  }

  pub fn does_exist(&self) -> bool {
    Path::new(&self.path).exists()
  }
}

impl From<&Path> for TargetDir {
  fn from(path: &Path) -> Self {
      TargetDir::new(&path.to_string_lossy().to_string())
  }
}

impl From<&str> for TargetDir {
  fn from(path: &str) -> Self {
      TargetDir::new(path)
  }
}


impl AsRef<Path> for TargetDir {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.path)
  }
}

impl AsRef<OsStr> for TargetDir {
  fn as_ref(&self) -> &OsStr {
    self.path.as_ref()
  }
}

impl Default for TargetDir {
  fn default() -> Self {
    Self {
      path: "".to_owned()
    }
  }
}
