use std::collections::HashMap;

use crate::{config::UserConfig, logging::Lines};
use super::{UserVariableValue, UserVariableKey, TemplateVariables};
use std::{format as s};

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


impl Lines for ValidConfig {
    fn lines(&self) -> Vec<String> {
      self
        .user_variables
        .iter()
        .map(|(k, v)|{
          s!("{} -> {}", k.value, v.value)
        })
        .collect()
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
