use serde::Deserialize;

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

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TemplateVariable {
  pub variable_name: String,
  pub description: String,
  pub prompt: String,
  #[serde(default)] // use default value if not found in the input
  pub filters: Vec<VariableFilter>
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
  pub filter: FilterType // make this an ADT
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
        filters: vec![]
     };

     assert_eq!(second, &expected_second)
  }
}
