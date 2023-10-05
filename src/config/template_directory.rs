use std::path::PathBuf;
use std::{path::Path, ffi::OsStr};


#[derive(Debug, Clone, PartialEq)]
pub struct TemplateDir {
  path: String
}

static TEMPLATE_FILES_DIR: &str = "template";

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

  pub fn template_files_path(&self) -> TemplateFilesDir {
    TemplateFilesDir::from(self)
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


impl AsRef<OsStr> for TemplateDir {
  fn as_ref(&self) -> &OsStr {
    self.path.as_ref()
  }
}

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
