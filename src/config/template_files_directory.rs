use std::path::{Path, PathBuf};
use super::TemplateDir;
use crate::config::SHELL_HOOK_FILE;

pub static TEMPLATE_FILES_DIR: &str = "template";

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateFilesDir {
  path: String
}

impl TemplateFilesDir {
  /// Make this private outside this class. Use the TemplateFilesDir::from to construct.
  fn new(path: &str) -> Self {
    Self {
      path: path.to_owned()
    }
  }

  pub fn path(&self) -> &str {
    self.path.as_str()
  }

  pub fn does_exist(&self) -> bool {
    Path::new(&self.path).exists()
  }


  pub fn join<P>(&self, other: P) -> PathBuf where
    P: AsRef<Path>
  {
    Path::new(&self.path).join(other)
  }
}

impl From<&TemplateDir> for TemplateFilesDir {
    fn from(template_dir: &TemplateDir) -> Self {
      TemplateFilesDir::new(&template_dir.join(TEMPLATE_FILES_DIR).to_string_lossy().to_string())
    }
}

impl AsRef<Path> for TemplateFilesDir {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.path)
  }
}
