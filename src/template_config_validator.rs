use std::collections::HashMap;

use crate::models::{TargetDir, TemplateDir};
use crate::user_config_provider::{Ignores, UserConfig};
use crate::variables::{UserVariableValue, UserVariableKey, TemplateVariables}; // Should this be moved into a shared models?

#[derive(Debug, Clone, PartialEq)]
pub struct ValidConfig {
  pub user_variables: HashMap<UserVariableKey, UserVariableValue>,
  pub user_config: UserConfig
}

impl ValidConfig {
  pub fn new(user_variables: HashMap<UserVariableKey, UserVariableValue>, user_config: UserConfig) -> Self {
    Self {
      user_variables,
      user_config
    }
  }
}


#[derive(Debug, Clone, PartialEq)]
pub enum TemplateVariableReview {
  Rejected,
  Accepted(ValidConfig)
}

pub trait TemplateConfigValidator {
  fn validate(&self, user_config: UserConfig, template_variables: TemplateVariables) -> TemplateVariableReview;
}
