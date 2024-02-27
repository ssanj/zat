    // "choice": [
    //   {
    //     "display": "Short",
    //     "description": "A shorter README"
    //     "value": "short"
    //   },
    //   {
    //     "display": "Long",
    //     "description": "A longer README",
    //     "value": "long"
    //   }
    // ]

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Choice {
  pub display: String,
  pub description: String,
  pub value: String
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
