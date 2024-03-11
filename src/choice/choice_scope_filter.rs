use std::collections::HashMap;

use crate::templates::{TemplateVariables, UserChoiceKey, UserChoiceValue};

use super::SelectedChoices;

pub trait ChoiceScopeFilter {
  fn filter_scopes(choices: &HashMap<UserChoiceKey, UserChoiceValue>, variables: &mut TemplateVariables);
}
