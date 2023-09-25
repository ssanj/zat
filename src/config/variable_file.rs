use std::path::Path;
use super::template_directory::TemplateDir;

#[derive(Debug, Clone)]
pub struct VariableFile {
  path: String
}

impl VariableFile {

  pub const PATH: &'static str  = ".variables.prompt";

  pub fn does_exist(&self) -> bool {
    Path::new(&self.path).exists()
  }
}

impl From<TemplateDir> for VariableFile {
  fn from(template_dir: TemplateDir) -> Self {
      let variables_file = template_dir.join(VariableFile::PATH);
      VariableFile {
        path: variables_file.display().to_string()
      }
  }
}

impl AsRef<Path> for VariableFile {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.path)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Filters {
  pub values: Vec<String>,
}


impl Default for Filters {
    fn default() -> Self {
        Self {
          values: vec![]
        }
    }
}

