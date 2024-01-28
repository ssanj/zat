use serde::Deserialize;

use crate::logging::Lines;
use std::{format as s};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TemplateVariables {
  pub tokens: Vec<TemplateVariable>
}


impl Default for TemplateVariables {
  fn default() -> Self {
    TemplateVariables {
      tokens: vec![]
    }
  }
}

impl Lines for TemplateVariables {

    fn lines(&self) -> Vec<String> {
        self
          .tokens
          .clone()
          .into_iter()
          .flat_map(|token| {
            vec!
              [
                s!("Variable name: {}", token.variable_name),
                s!("Description: {}", token.description),
                s!("Prompt: {}", token.prompt),
                s!("Default value: {}", token.default_value.unwrap_or_else(|| "-".to_owned())),
                s!(""),
              ]
          })
          .collect()
    }
}


#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TemplateVariable {
  pub variable_name: String,
  pub description: String,
  pub prompt: String,
  #[serde(default)] // use default value if not found in the input
  pub filters: Vec<VariableFilter>,

  #[serde(default)] // use default value if not found in the input
  pub default_value: Option<String>
}

impl TemplateVariable {

  #[cfg(test)]
  pub fn new(variable_name: &str, description: &str, prompt: &str, filters: &[VariableFilter], default_value: Option<&str>) -> Self {
    Self {
      variable_name: variable_name.to_owned(),
      description:description.to_owned(),
      prompt: prompt.to_owned(),
      filters: Vec::from_iter(filters.iter().map(|v| v.clone())),
      default_value: default_value.map(|v| v.to_owned())
    }
  }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct UserVariableKey {
  pub value: String
}

impl UserVariableKey {
  pub fn new(value: String) -> Self {
    UserVariableKey {
      value
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserVariableValue {
  pub value: String
}

impl UserVariableValue {
  pub fn new(value: String) -> Self {
    UserVariableValue {
      value
    }
  }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct VariableFilter {
  pub name: String,
  pub filter: FilterType
}

impl VariableFilter {

  #[cfg(test)]
  pub fn new(name: &str, filter: &FilterType) -> Self {
    Self {
      name: name.to_owned(),
      filter: filter.clone()
    }
  }

  #[cfg(test)]
  pub fn from_pairs(values: &[(&str, &FilterType)]) -> Vec<VariableFilter> {
    Vec::from_iter(
      values
        .iter()
        .map(|(n, f)| VariableFilter::new(n, f))
    )
  }
}


#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum FilterType {
  Camel,
  Cobol,
  Flat,
  Kebab,
  Lower,
  Noop,
  Pascal,
  Snake,
  Title,
  Upper
}

#[cfg(test)]
mod test {
  use pretty_assertions::assert_eq;
  use super::*;

  #[test]
  fn load_json_config() {
    let variables_config = r#"
      [
        {
          "variable_name": "project",
          "description": "Name of project",
          "prompt": "Please enter your project name",
          "default_value": "Some Project",
          "filters": [
            {
              "name":"python",
              "filter": "Snake"
            },
            { "name": "Command",
              "filter": "Pascal"
            }
          ]
        },
        {
          "variable_name": "plugin_description",
          "description": "Explain what your plugin is about",
          "prompt": "Please enter your plugin description"
        }
      ]
    "#;

     let variables: Vec<TemplateVariable> = serde_json::from_str(&variables_config).unwrap();
     assert_eq!(variables.len(), 2);

     let first = &variables[0];
     let expected_first = TemplateVariable {
        variable_name: "project".to_owned(),
        description: "Name of project".to_owned(),
        prompt: "Please enter your project name".to_owned(),
        default_value: Some("Some Project".to_owned()),
        filters: vec![
          VariableFilter {
            name: "python".to_owned(),
            filter: FilterType::Snake
          },
          VariableFilter {
            name: "Command".to_owned(),
            filter: FilterType::Pascal
          },
        ]
     };

     assert_eq!(first, &expected_first);

     let second = &variables[1];

     let expected_second = TemplateVariable {
        variable_name: "plugin_description".to_owned(),
        description: "Explain what your plugin is about".to_owned(),
        prompt: "Please enter your plugin description".to_owned(),
        default_value: None,
        filters: vec![]
     };

     assert_eq!(second, &expected_second)
  }
}
