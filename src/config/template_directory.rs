use std::path::PathBuf;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateDir {
  path: String
}

impl TemplateDir {
  pub fn new(path: &str) -> Self {
    TemplateDir {
      path: path.to_owned()
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

  pub fn path(&self) -> &str {
    self.path.as_str()
  }
}


impl AsRef<Path> for TemplateDir {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.path)
  }
}

impl From<&Path> for TemplateDir {
  fn from(path: &Path) -> Self {
      Self::new(&path.to_string_lossy().to_string())
  }
}

