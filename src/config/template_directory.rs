use std::path::PathBuf;
use std::{path::Path, ffi::OsStr};


#[derive(Debug, Clone, PartialEq)]
pub struct TemplateDir {
  pub path: String
}

impl TemplateDir {
  pub fn new(path: &str) -> Self {
    TemplateDir {
      path: path.to_owned(),
    }
  }

  pub fn join<P>(&self, other: P) -> PathBuf where
    P: AsRef<Path>
  {
    Path::new(&self.path).join(other)
  }


  pub fn does_exist(&self) -> bool {
    Path::new(&self.path).exists()
  }
}

impl AsRef<Path> for TemplateDir {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.path)
  }
}

impl AsRef<OsStr> for TemplateDir {
  fn as_ref(&self) -> &OsStr {
    self.path.as_ref()
  }
}
