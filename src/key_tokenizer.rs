use std::collections::HashMap;

use crate::template_variable_expander::{ExpandedKey, ExpandedValue, ExpandedVariables};

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct TokenizedExpandedKey {
  pub value: String
}

impl TokenizedExpandedKey {
  pub fn new(input: &str) -> Self {
    Self {
      value: input.to_owned()
    }
  }
}

impl AsRef<str> for TokenizedExpandedKey {
  fn as_ref(&self) -> &str {
      &self.value
  }
}


pub trait KeyTokenizer {
  fn tokenize_keys(&self, expanded_variables: ExpandedVariables) -> HashMap<TokenizedExpandedKey, ExpandedValue>;
}
