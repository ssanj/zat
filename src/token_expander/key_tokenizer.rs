use std::collections::HashMap;
use crate::logging::Lines;
use super::{ExpandedValue, ExpandedVariables};
use std::{format as s};


#[derive(Debug, Clone)]
pub struct TokenizedKeysExpandedVariables {
  pub value: HashMap<TokenizedExpandedKey, ExpandedValue>
}

impl Lines for TokenizedKeysExpandedVariables {
    fn lines(&self) -> Vec<String> {
      self
        .value
        .iter()
        .map(|(k, v)|{
          s!("{} -> {}", k.value, v.value)
        })
        .collect()
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
