use std::collections::HashMap;
use std::path::Path;

use crate::models::{SourceFile, TargetDir, TargetFile, TemplateDir};
use crate::shared_models::*;

#[derive(Debug, Clone)]
pub struct FileFilter {
  value: String
}

impl FileFilter {
  pub fn new(value: &str) -> Self {
    Self {
      value: value.to_owned()
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ignores { //TODO: Use regex filtering for these
  pub files: Vec<String>,
  pub directories: Vec<String>,
}

impl Default for Ignores {
  fn default() -> Self {
    Ignores {
      files: vec![],
      directories: vec![]
    }
  }
}

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
pub struct UserConfig {
  // pub user_tokens: HashMap<String, String>,
  pub template_dir: TemplateDir,
  pub target_dir: TargetDir,
  pub ignores: Ignores //TODO: We will need to always include the ".variables" file into the ignores
}

/// Behaviour to return configuration provided by the "user"
pub trait UserConfigProvider {
  /// Returns the UserConfig
  fn get_config(&self) -> ZatResultX<UserConfig>;
}
