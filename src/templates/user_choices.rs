use std::collections::HashMap;

use super::{UserChoiceKey, UserChoiceValue};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserChoices {
  pub value: HashMap<UserChoiceKey, UserChoiceValue>
}

impl UserChoices {
  pub fn new(value: HashMap<UserChoiceKey, UserChoiceValue>) -> Self {
    Self {
      value
    }
  }

  pub fn is_empty(&self) -> bool {
    self.value.is_empty()
  }

  pub fn has_choices(&self) -> bool {
    !self.is_empty()
  }
}
