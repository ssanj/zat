use std::collections::HashMap;

use super::{ExpandedValue, ExpandedVariables};


#[derive(Debug, Clone)]
pub struct TokenizedKeysExpandedVariables {
  pub value: HashMap<TokenizedExpandedKey, ExpandedValue>
}

impl TokenizedKeysExpandedVariables {
  pub fn new(value: HashMap<TokenizedExpandedKey, ExpandedValue>) -> Self {
    Self {
      value
    }
  }
}


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
  fn tokenize_keys(&self, expanded_variables: ExpandedVariables) -> TokenizedKeysExpandedVariables;
}
