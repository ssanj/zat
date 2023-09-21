use std::collections::HashMap;

use crate::variables::{UserVariableKey, UserVariableValue, TemplateVariables};
use super::key_tokenizer::TokenizedKeysExpandedVariables;

pub trait ExpandFilters {
  fn expand_filers(&self, template_variables: TemplateVariables, user_variables: HashMap<UserVariableKey, UserVariableValue>) -> TokenizedKeysExpandedVariables;
}
