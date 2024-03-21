use std::collections::HashMap;

use crate::templates::{TemplateVariables, UserChoiceKey, UserChoiceValue};

pub trait ChoiceScopeFilter {
  fn filter_scopes(choices: &HashMap<UserChoiceKey, UserChoiceValue>, variables: &mut TemplateVariables);
}
