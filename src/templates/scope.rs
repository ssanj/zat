use serde::Deserialize;

// IncludeChoiceValueScope
//   "scope": [
//   {
//     "choice": "test_framework", // only this value for this choice
//     "value": "scalatest"
//   }
// ]

// ExcludeChoiceValueScope
// "scope": [
//   {
//     "choice": "test_framework",
//     "not_value": "scalatest" // any value in the test_framework choice that is not scalatest
//   }
// ]

// IncludeChoiceScope
// "scope": [
//   {
//     "choice": "test_framework" // any choice in test_framework
//   }
// ]

// IncludeChoiceScope
// "scope": [
//   {
//     "not_choice": "test_framework" // any choice but this
//   }
// ]

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Scope {
  IncludeChoiceValueScope(IncludeChoiceValue), // The order of these definitions matter. Add the most specific types first; eg. IncludeChoiceValueScope before IncludeChoiceScope
  ExcludeChoiceValueScope(ExcludeChoiceValue),
  IncludeChoiceScope(IncludeChoice),
  ExcludeChoiceScope(ExcludeChoice),
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
