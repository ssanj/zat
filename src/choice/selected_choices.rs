use std::collections::HashMap;
use crate::templates::{TemplateVariable, TemplateVariables, UserChoiceKey, UserChoiceValue};

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedChoices {
  pub choices: HashMap<UserChoiceKey, UserChoiceValue>,
  pub other_variables: TemplateVariables
}

impl SelectedChoices {
  pub fn new(choices: HashMap<UserChoiceKey, UserChoiceValue>, other_variables: Vec<TemplateVariable>) -> Self {
    Self {
      choices,
      other_variables: TemplateVariables::new(other_variables)
    }
  }

  pub fn has_choices(&self) -> bool {
    !self.choices.is_empty()
  }
}
