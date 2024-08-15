use std::fmt;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Choice {
  pub display: String,
  pub description: String,
  pub value: String
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.display, self.description)
    }
}

impl Choice {

  pub fn new(display: &str, description: &str, value: &str) -> Self {
    Self {
      display: display.to_owned(),
      description: description.to_owned(),
      value: value.to_owned()
    }
  }
}
