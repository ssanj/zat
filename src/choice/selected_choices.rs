use std::collections::HashMap;
use crate::templates::{TemplateVariable, TemplateVariables, UserChoiceKey, UserChoiceValue};

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedChoices {
  pub choices: HashMap<UserChoiceKey, UserChoiceValue>,
  pub variables: TemplateVariables
}

impl SelectedChoices {
  pub fn new(choices: HashMap<UserChoiceKey, UserChoiceValue>, variables: Vec<TemplateVariable>) -> Self {
    Self {
      choices,
      variables: TemplateVariables::new(variables)
    }
  }
}
