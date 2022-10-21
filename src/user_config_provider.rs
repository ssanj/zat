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
  pub fn does_exist(&self) -> bool {
    Path::new(&self.path).exists()
  }
}

impl From<TargetDir> for VariableFile {
  fn from(target_dir: TargetDir) -> Self {
      let variables_file = target_dir.join(".variables.prompt");
      VariableFile {
        path: variables_file.display().to_string()
      }
  }
}

#[derive(Debug, Clone)]
pub struct Config {
  // pub user_tokens: HashMap<String, String>,
  pub template_dir: TemplateDir,
  pub target_dir: TargetDir,
  pub ignores: Ignores
}

// Get user configuration
// Load token file (if any)
pub trait UserConfigProvider {
  fn get_config(&self) -> ZatResultX<Config>;
}
