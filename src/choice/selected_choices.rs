use serde::de::value;

use crate::templates::Choice;

pub struct SelectedChoices {
  pub choices: Vec<Choice>
}

impl SelectedChoices {
  pub fn new(choices: Vec<Choice>) -> Self {
    Self {
      choices
    }
  }
}
