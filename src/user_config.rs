use std::collections::HashMap;

use crate::models::{ZatResult, ZatError, ZatAction, SourceFile, TargetDir, TargetFile, TemplateDir};

#[derive(Debug, Clone)]
pub struct Ignores {
  files: Vec<String>,
  directories: Vec<String>,
}

pub struct Config {
  user_tokens: HashMap<String, String>,
  template_dir: TemplateDir,
  target_dir: TargetDir,
  ignores: Ignores
}

// Get user configuration
// Load token file (if any)
pub trait UserConfig {
  fn get_config() -> ZatResult<Config>;
}
