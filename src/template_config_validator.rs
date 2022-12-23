use std::collections::HashMap;

use crate::models::{TargetDir, TemplateDir};
use crate::user_config_provider::{Ignores, UserConfigX};
use crate::variables::{UserVariableValue, UserVariableKey, TemplateVariables}; // Should this be moved into a shared models?

#[derive(Debug, Clone, PartialEq)]
pub struct ValidConfig {
  pub user_variables: HashMap<UserVariableKey, UserVariableValue>,
  pub user_config: UserConfigX
}

impl ValidConfig {
  pub fn new(user_variables: HashMap<UserVariableKey, UserVariableValue>, user_config: UserConfigX) -> Self {
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
  fn validate(&self, user_config: UserConfigX, template_variables: TemplateVariables) -> TemplateVariableReview;
}
