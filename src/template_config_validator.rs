use std::collections::HashMap;

use crate::models::{TargetDir, TemplateDir};
use crate::user_config::{Ignores, Config}; // Should this be moved into a shared models?

#[derive(Debug, Clone)]
pub struct ValidConfig {
  user_tokens: HashMap<String, String>,
  template_dir: TemplateDir,
  target_dir: TargetDir,
  ignores: Ignores
}

#[derive(Debug, Clone)]
pub enum ConfigState {
  ConfigIsIncorrect,
  ConfigIsCorrect(ValidConfig)
}

pub trait TemplateConfigValidator {
  fn validate(config: Config) -> ConfigState;
}

