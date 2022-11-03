use std::collections::HashMap;
use std::path::Path;

use crate::models::{SourceFile, TargetDir, TargetFile, TemplateDir};
use crate::shared_models::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Ignores {
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
  pub ignores: Ignores
}

/// Behaviour to return configuration provided by the "user"
pub trait UserConfigProvider {
  /// Returns the UserConfig
  fn get_config(&self) -> ZatResultX<UserConfig>;
}
