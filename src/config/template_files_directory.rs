use std::path::Path;
use super::RepositoryDir;
use std::format as s;
use crate::spath;

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

  pub fn relative_path(&self, file: &str) -> String {
    let template_files_dir_path = Path::new(self.path());
    let file_path = Path::new(file);
    // TODO: Should we raise this as an ZatError?
    let relative = file_path.strip_prefix(template_files_dir_path).expect(&s!("Could not strip path prefix '{}' from path '{}'", self.path(), file));
    spath!(relative).clone()
  }
}

impl From<&RepositoryDir> for TemplateFilesDir {
    fn from(template_dir: &RepositoryDir) -> Self {
      TemplateFilesDir::new(&template_dir.join(TEMPLATE_FILES_DIR).to_string_lossy().to_string())
    }
}

impl AsRef<Path> for TemplateFilesDir {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.path)
  }
}

impl Default for TemplateFilesDir {
  fn default() -> Self {
    Self {
      path: "".to_owned()
    }
  }
}
