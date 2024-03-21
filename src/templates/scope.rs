use serde::Deserialize;
use std::format as s;
use std::fmt::Display;

// IncludeChoiceValueScope
//   "scopes": [
//   {
//     "choice": "test_framework", // only this value for this choice
//     "value": "scalatest"
//   }
// ]

// ExcludeChoiceValueScope
// "scopes": [
//   {
//     "choice": "test_framework",
//     "not_value": "scalatest" // any value in the test_framework choice that is not scalatest
//   }
// ]

// IncludeChoiceScope
// "scopes": [
//   {
//     "choice": "test_framework" // any choice in test_framework
//   }
// ]

// IncludeChoiceScope
// "scopes": [
//   {
//     "not_choice": "test_framework" // any choice but this
//   }
// ]

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
#[allow(clippy::enum_variant_names)]
pub enum Scope {
  IncludeChoiceValueScope(IncludeChoiceValue), // The order of these definitions matter. Add the most specific types first; eg. IncludeChoiceValueScope before IncludeChoiceScope
  ExcludeChoiceValueScope(ExcludeChoiceValue),
  IncludeChoiceScope(IncludeChoice),
  ExcludeChoiceScope(ExcludeChoice),
}

impl Display for Scope {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let value =
        match self {
          Scope::IncludeChoiceValueScope(IncludeChoiceValue { choice, value }) => s!("include if choice: {} has value: {}", choice, value),
          Scope::ExcludeChoiceValueScope(ExcludeChoiceValue { choice, not_value }) => s!("exclude if choice: {} has value: {}", choice, not_value),
          Scope::IncludeChoiceScope(IncludeChoice { choice }) => s!("include if choice: {} is chosen with any value", choice),
          Scope::ExcludeChoiceScope(ExcludeChoice { not_choice }) => s!("exclude if choice: {} is chosen with any value", not_choice),
      };

      f.write_str(value.as_str())
  }
}

impl Scope {
  #[cfg(test)]
  pub fn new_include_choice(value: &str) -> Self {
    Scope::IncludeChoiceScope(IncludeChoice::new(value))
  }

  #[cfg(test)]
  pub fn new_include_choice_value(key: &str, value: &str) -> Self {
    Scope::IncludeChoiceValueScope(IncludeChoiceValue::new(key, value))
  }

  #[cfg(test)]
  pub fn new_exclude_choice(value: &str) -> Self {
    Scope::ExcludeChoiceScope(ExcludeChoice::new(value))
  }

  #[cfg(test)]
  pub fn new_exclude_choice_value(key: &str, value: &str) -> Self {
    Scope::ExcludeChoiceValueScope(ExcludeChoiceValue::new(key, value))
  }

}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IncludeChoice {
  pub choice: String
}

impl IncludeChoice {
  #[cfg(test)]
  pub fn new(value: &str) -> Self {
    Self {
      choice: value.to_owned()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ExcludeChoice {
  pub not_choice: String
}

impl ExcludeChoice {
  #[cfg(test)]
  pub fn new(value: &str) -> Self {
    Self {
      not_choice: value.to_owned()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IncludeChoiceValue {
  pub choice: String,
  pub value: String
}

impl IncludeChoiceValue {
  #[cfg(test)]
  pub fn new(choice: &str, value: &str) -> Self {
    Self {
      choice: choice.to_owned(),
      value: value.to_owned()
    }
  }
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ExcludeChoiceValue {
  pub choice: String,
  pub not_value: String
}

impl ExcludeChoiceValue {
  #[cfg(test)]
  pub fn new(choice: &str, value: &str) -> Self {
    Self {
      choice: choice.to_owned(),
      not_value: value.to_owned()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn decodes_included_choice() {
    let input = r#"
      {
        "choice": "test_framework"
      }
    "#;

    let scope = serde_json::from_str::<Scope>(input).unwrap();

    let expected_scope =
      Scope::IncludeChoiceScope(IncludeChoice::new("test_framework"));

    assert_eq!(scope, expected_scope);
  }

  #[test]
  fn decodes_exluded_choice() {
    let input = r#"
      {
        "not_choice": "test_framework"
      }
    "#;

    let scope = serde_json::from_str::<Scope>(input).unwrap();

    let expected_scope =
      Scope::ExcludeChoiceScope(ExcludeChoice::new("test_framework"));

    assert_eq!(scope, expected_scope);
  }

  #[test]
  fn decodes_included_choice_value() {
    let input = r#"
      {
        "choice": "test_framework",
        "value": "scalatest"
      }
    "#;

    let scope = serde_json::from_str::<Scope>(input).unwrap();

    let expected_scope =
      Scope::IncludeChoiceValueScope(IncludeChoiceValue::new("test_framework", "scalatest"));

    assert_eq!(scope, expected_scope);
  }

  #[test]
  fn decodes_excluded_choice_value() {
    let input = r#"
      {
        "choice": "test_framework",
        "not_value": "scalatest"
      }
    "#;

    let scope = serde_json::from_str::<Scope>(input).unwrap();

    let expected_scope =
      Scope::ExcludeChoiceValueScope(ExcludeChoiceValue::new("test_framework", "scalatest"));

    assert_eq!(scope, expected_scope);
  }
}
