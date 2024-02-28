use std::collections::HashMap;

use super::{UserChoiceKey, UserChoiceValue};

#[derive(Debug, Clone, PartialEq)]
pub struct UserChoices {
  pub value: HashMap<UserChoiceKey, UserChoiceValue>
}

impl UserChoices {
  pub fn new(value: HashMap<UserChoiceKey, UserChoiceValue>) -> Self {
    Self {
      value
    }
  }
}
