use std::collections::HashMap;

use crate::config::user_config::UserConfig;
use super::variables::{UserVariableValue, UserVariableKey, TemplateVariables};

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
