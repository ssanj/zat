use serde::Deserialize;

use crate::logging::Lines;
use std::format as s;
use super::{Choice, Plugin};

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct TemplateVariables {
  pub tokens: Vec<TemplateVariable>
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

  // TODO: Create enum with one of DefaultValue, PluginValue or ChoiceValue
  #[serde(default)] // use default value if not found in the input
  pub default_value: Option<String>,

  pub plugin: Option<Plugin>,

  #[serde(default)] // use default value if not found in the input
  pub choice: Vec<Choice>
}

impl TemplateVariable {

  #[cfg(test)]
  pub fn new(variable_name: &str, description: &str, prompt: &str, filters: &[VariableFilter], default_value: Option<&str>) -> Self {
    Self {
      variable_name: variable_name.to_owned(),
      description:description.to_owned(),
      prompt: prompt.to_owned(),
      filters: Vec::from_iter(filters.iter().map(|v| v.clone())),
      default_value: default_value.map(|v| v.to_owned()),
      plugin: None,
      choice: Default::default()
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
        },
        {
          "variable_name": "readme_type",
          "description": "Type of README",
          "prompt": "Please choose your type of README",
          "choice": [
            {
              "display": "Short",
              "description": "A shorter README",
              "value": "short"
            },
            {
              "display": "Long",
              "description": "A longer README",
              "value": "long"
            }
          ]
        },
        {
          "variable_name": "description",
          "description": "What your project is about",
          "prompt": "Please a description of your project",
          "plugin": {
            "id": "tests/plugins/success.sh",
            "args":[
              "Testing 123"
            ]
          }
        }
      ]
    "#;

    let variables: Vec<TemplateVariable> = serde_json::from_str(&variables_config).unwrap();

    let expected_first = TemplateVariable {
      variable_name: "project".to_owned(),
      description: "Name of project".to_owned(),
      prompt: "Please enter your project name".to_owned(),
      default_value: Some("Some Project".to_owned()),
      plugin: None,
      choice: Default::default(),
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

    let expected_second = TemplateVariable {
      variable_name: "plugin_description".to_owned(),
      description: "Explain what your plugin is about".to_owned(),
      prompt: "Please enter your plugin description".to_owned(),
      default_value: None,
      plugin: None,
      choice: Default::default(),
      filters: vec![]
    };

    let expected_third = TemplateVariable {
      variable_name: "readme_type".to_owned(),
      description: "Type of README".to_owned(),
      prompt: "Please choose your type of README".to_owned(),
      default_value: None,
      plugin: None,
      choice:
        vec![
          Choice::new("Short", "A shorter README", "short"),
          Choice::new("Long", "A longer README", "long"),
        ],
      filters: vec![]
    };

    let expected_fourth = TemplateVariable {
      variable_name: "description".to_owned(),
      description: "What your project is about".to_owned(),
      prompt: "Please a description of your project".to_owned(),
      default_value: None,
      plugin: Some(Plugin::new("tests/plugins/success.sh", &["Testing 123"])),
      choice: Vec::default(),
      filters: Vec::default()
    };

    let expected_variables =
      vec![
        expected_first,
        expected_second,
        expected_third,
        expected_fourth,
      ];

     assert_eq!(variables, expected_variables)
  }
}
