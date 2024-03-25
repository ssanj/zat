use std::collections::HashMap;

use crate::{choice::SelectedChoices, config::UserConfig, error::ZatResult, logging::Lines};
use super::{UserVariableValue, UserVariableKey, UserChoiceKey, UserChoiceValue};
use std::{format as s};

#[derive(Debug, Clone, PartialEq)]
pub struct ValidConfig {
  pub user_variables: HashMap<UserVariableKey, UserVariableValue>,
  pub user_choices: HashMap<UserChoiceKey, UserChoiceValue>,
  pub user_config: UserConfig
}

impl ValidConfig {
  pub fn new(user_variables: HashMap<UserVariableKey, UserVariableValue>, user_choices: HashMap<UserChoiceKey, UserChoiceValue>, user_config: UserConfig) -> Self {
    Self {
      user_variables,
      user_choices,
      user_config
    }
  }
}


impl Lines for ValidConfig {
    fn lines(&self) -> Vec<String> {
      let mut variable_lines =
        self
          .user_variables
          .iter()
          .map(|(k, v)|{
            s!("{} -> {}", k.value, v.value)
          })
          .collect::<Vec<_>>();

      let mut choices_lines =
        self
          .user_choices
          .iter()
          .map(|(k, v)| {
            s!("{} -> item:{}, value:{}", k.value, v.value.display, v.value.value)
          })
          .collect::<Vec<_>>();

      choices_lines.append(&mut variable_lines);

      choices_lines
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum TemplateVariableReview {
  Rejected,
  Accepted(ValidConfig)
}

pub trait TemplateConfigValidator {
  fn validate(&self, user_config: &UserConfig, selected_choices: &SelectedChoices) -> ZatResult<TemplateVariableReview>;
}
