use serde::Deserialize;

use crate::logging::Lines;
use std::format as s;
use super::{Choice, Plugin, Scope};
use super::ArgType;

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct TemplateVariables {
  pub tokens: Vec<TemplateVariable>
}

impl TemplateVariables {
  pub fn new(tokens: Vec<TemplateVariable>) -> Self {
    Self {
      tokens
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
                s!("Default value: {}", token.default_value.as_deref().unwrap_or("-")),
                s!("Plugin: {}", token.plugin.as_ref().map(Self::plugin_lines).unwrap_or_else(|| "-".to_owned())),
                s!("Choices: {}", Self::choice_lines(&token, &token.choice.iter().collect::<Vec<_>>())),
                s!("Scopes: {}", Self::scope_lines(&token.scopes.map(|v| v.iter().cloned().collect::<Vec<_>>()))),

                s!(""),
              ]
          })
          .collect()
    }
}

impl TemplateVariables {

  fn plugin_lines(plugin: &Plugin) -> String {
    let id = &plugin.id;
    let args: String = match &plugin.args {
        Some(ArgType::ArgLine(args)) => args.join(" "),
        Some(ArgType::MutlipleArgs(args)) => {
          args
            .iter()
            .map(|a|{
              s!("{}{} {}", a.prefix, a.name, a.value)
            })
            .collect::<Vec<_>>()
            .join(" ")
        },
        None => "".to_owned(),
    };

    let run_status = match &plugin.result {
      super::PluginRunStatus::NotRun => "Not run".to_owned(),
      super::PluginRunStatus::Run(result) => result.result.clone(),
    };

    [
      "".to_owned(),
      s!("Id: {id}"),
      s!("Args: {args}"),
      s!("Status: {run_status}"),
    ].join("\n    ")
  }

  fn choice_lines(_token: &TemplateVariable, choices: &[&Choice]) -> String {

    if choices.is_empty() {
      "-".to_owned()
    } else {
      let items =
        choices
          .iter()
          .map(|c| {
            s!("Display: {}, Description: {}, Value: {}", c.display, c.description, c.value)
          })
          .collect::<Vec<_>>();

      let item_str = items.join("\n      ");

      s!("{item_str}")
    }
  }

  fn scope_lines(maybe_scopes: &Option<Vec<Scope>>) -> String {

    match maybe_scopes {
        Some(xs) if xs.is_empty() => "-".to_owned(),
        Some(scopes) => {
          let items =
            scopes
              .iter()
              .map(|scope| {
                s!("{}", scope)
              })
              .collect::<Vec<_>>();

          let header = "      ";
          let item_str = items.join(s!("\n{header}").as_str());

          s!("{header}{item_str}")
        },
        None => "-".to_owned(),
    }
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
  pub choice: Vec<Choice>,

  pub scopes: Option<Vec<Scope>>
}

impl TemplateVariable {

  #[cfg(test)]
  pub fn new(variable_name: &str, description: &str, prompt: &str, filters: &[VariableFilter], default_value: Option<&str>) -> Self {
    Self {
      variable_name: variable_name.to_owned(),
      description:description.to_owned(),
      prompt: prompt.to_owned(),
      filters: Vec::from_iter(filters.iter().cloned()),
      default_value: default_value.map(|v| v.to_owned()),
      plugin: None,
      choice: Default::default(),
      scopes: Option::default()
    }
  }

  #[cfg(test)]
  pub fn with_scopes(variable_name: &str, scopes: Vec<Scope>) -> Self {

    Self {
      variable_name: variable_name.to_owned(),
      description: s!("{}-description", variable_name),
      prompt: s!("{}-prompt", variable_name).to_owned(),
      filters: Vec::default(),
      default_value: Option::default(),
      plugin: Option::default(),
      choice: Default::default(),
      scopes: Some(scopes)
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

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct UserChoiceKey {
  pub value: String
}

impl UserChoiceKey {
  pub fn new(value: String) -> Self {
    Self {
      value
    }
  }
}

impl From<&str> for UserChoiceKey {
  fn from(value: &str) -> Self {
      Self {
        value: value.to_owned()
      }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserChoiceValue {
  pub value: Choice
}

impl UserChoiceValue {
  pub fn new(value: Choice) -> Self {
    Self {
      value
    }
  }
}

impl From<(&str, &str, &str)> for UserChoiceValue {

  fn from(value: (&str, &str, &str)) -> Self {
    UserChoiceValue::new(Choice::new(value.0, value.1, value.2))
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
  use super::super::scope::IncludeChoiceValue;

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
          },
          "scopes": [
            {
              "choice": "readme_type",
              "value": "short"
            }
          ]
        }
      ]
    "#;

    let variables: Vec<TemplateVariable> = serde_json::from_str(variables_config).unwrap();

    let expected_first = TemplateVariable {
      variable_name: "project".to_owned(),
      description: "Name of project".to_owned(),
      prompt: "Please enter your project name".to_owned(),
      default_value: Some("Some Project".to_owned()),
      plugin: None,
      choice: Default::default(),
      scopes: Option::default(),
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
      filters: vec![],
      scopes: Option::default(),
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
      filters: vec![],
      scopes: Option::default(),
    };

    let expected_scope =
      vec![
        Scope::IncludeChoiceValueScope(IncludeChoiceValue::new("readme_type", "short"))
      ];

    let expected_fourth = TemplateVariable {
      variable_name: "description".to_owned(),
      description: "What your project is about".to_owned(),
      prompt: "Please a description of your project".to_owned(),
      default_value: None,
      plugin: Some(Plugin::new("tests/plugins/success.sh", &["Testing 123"])),
      choice: Vec::default(),
      filters: Vec::default(),
      scopes: Some(expected_scope)
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
