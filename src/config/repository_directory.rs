use std::path::PathBuf;
use std::path::Path;

use super::SHELL_HOOK_FILE;

#[derive(Debug, Clone, PartialEq)]
pub struct RepositoryDir {
  path: String
}

impl RepositoryDir {
  pub fn new(path: &str) -> Self {
    RepositoryDir {
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

  pub fn shell_hook_file(&self) -> PathBuf {
    self.join(SHELL_HOOK_FILE)
  }
}


impl AsRef<Path> for RepositoryDir {
  fn as_ref(&self) -> &Path {
    Path::new(&self.path)
  }
}

impl From<&Path> for RepositoryDir {
  fn from(path: &Path) -> Self {
    Self::new(path.to_string_lossy().as_ref())
  }
}

impl Default for RepositoryDir {
  fn default() -> Self {
    Self {
      path: "".to_owned()
    }
  }
}

