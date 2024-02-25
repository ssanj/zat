use std::path::Path;
use super::RepositoryDir;

#[derive(Debug, Clone)]
pub struct VariableFile {
  path: String
}

pub const DOT_VARIABLES_PROMPT: &str  = ".variables.zat-prompt";

impl VariableFile {

  pub fn does_exist(&self) -> bool {
    Path::new(&self.path).exists()
  }

  pub fn get_path(&self) -> &str {
    self.path.as_str()
  }
}

impl From<RepositoryDir> for VariableFile {
  fn from(template_dir: RepositoryDir) -> Self {
      let variables_file = template_dir.join(DOT_VARIABLES_PROMPT);
      VariableFile {
        path: variables_file.display().to_string()
      }
  }
}

impl AsRef<Path> for VariableFile {
  fn as_ref(&self) -> &Path {
    Path::new(&self.path)
  }
}

