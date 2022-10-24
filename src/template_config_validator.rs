use std::collections::HashMap;

use crate::models::{TargetDir, TemplateDir};
use crate::user_config_provider::{Ignores, UserConfig};
use crate::variables::{UserVariableValue, UserVariableKey, TemplateVariables}; // Should this be moved into a shared models?

#[derive(Debug, Clone)]
pub struct ValidConfig {
  user_variables: HashMap<UserVariableKey, UserVariableValue>,
  template_dir: TemplateDir,
  target_dir: TargetDir,
  ignores: Ignores
}

#[derive(Debug, Clone)]
pub enum TemplateVariableReview {
  Rejected,
  Accepted(ValidConfig)
}

pub trait TemplateConfigValidator {
  fn validate(template_variables: TemplateVariables) -> TemplateVariableReview;
}

