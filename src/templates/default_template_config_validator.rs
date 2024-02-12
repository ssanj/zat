use std::collections::HashMap;
use std::io::BufRead;

use super::{TemplateConfigValidator, TemplateVariableReview, ValidConfig};
use super::{UserVariableValue, UserVariableKey, TemplateVariables};
use crate::config::UserConfig;
use crate::templates::PluginRunResult;
use ansi_term::Colour::{Yellow, Green, Blue};
use ansi_term::Style;
use std::{println as p, format as s};
use crate::logging::Logger;

// This is a support trait to TemplateConfigValidator, so we define it here as opposed to in its own module.
trait UserInputProvider {
  fn get_user_input(&self, variables: TemplateVariables) -> HashMap<UserVariableKey, UserVariableValue>;
}

trait UserTemplateVariableValidator {
  fn review_user_template_variables(&self, user_config: UserConfig, variables: HashMap<UserVariableKey, UserVariableValue>) -> TemplateVariableReview;
}

enum UserVariablesValidity {
  Valid,
  Invalid
}

struct Cli;

enum DynamicValueType {
  DefaultValue(String, String),
  PluginValue(String, String),
  Neither,
}

struct DynamicPair(String, String);


impl UserInputProvider for Cli {
    fn get_user_input(&self, template_variables: TemplateVariables) -> HashMap<UserVariableKey, UserVariableValue> {
      let stdin = std::io::stdin();
      let mut token_map = HashMap::new();

      for v in template_variables.tokens {
        p!("");
        let default_value = {
          match v.default_value {
            Some(default_value) => default_value,
            None  => "".to_owned()
          }
        };

        let plugin_result_value: Option<PluginRunResult> =
          v
            .plugin
            .and_then(|pl| {
              match pl.result {
                crate::templates::PluginRunStatus::NotRun => None,
                crate::templates::PluginRunStatus::Run(run_result) => Some(run_result),
              }
            });


        // TODO: Refactor the plugin and default value handling
        let default_type =
          if default_value.is_empty() {
            None
          } else {
            Some(
              DynamicPair(s!(". Press {} to accept the default value of: {}.", Style::new().underline().paint("enter"), Green.paint(&default_value)),
              default_value.to_owned()
              )
            )
          };

        let plugin_type =
          if let Some(ref plugin_result) = plugin_result_value {
            Some(
              DynamicPair(s!(". Press {} to accept the plugin result value of: {}.", Style::new().underline().paint("enter"), Green.paint(&plugin_result.result)),
               plugin_result.clone().result
              )
            )
          } else {
            None
          };

        let dynamic_value = match (default_type, plugin_type) {
          (Some(DynamicPair(dstring, dvalue)), None) => DynamicValueType::DefaultValue(dstring, dvalue),
          (None, Some(DynamicPair(pstring, pvalue))) => DynamicValueType::PluginValue(pstring, pvalue.to_owned()),
          (Some(_), Some(DynamicPair(pstring, pvalue))) => DynamicValueType::PluginValue(pstring, pvalue.to_owned()), // plugin overrides default
          (None, None) => DynamicValueType::Neither,
        };

        match &dynamic_value {
          DynamicValueType::DefaultValue(dstring, _) => p!("{}{}", Yellow.paint(v.prompt), dstring),
          DynamicValueType::PluginValue(pstring, _) => p!("{}{}", Yellow.paint(v.prompt), pstring),
          DynamicValueType::Neither => p!("{}", Yellow.paint(v.prompt)),
        }


        let mut variable_value = String::new();
        if let Ok(read_count) = stdin.read_line(&mut variable_value) {
          if read_count > 0 { //read at least one character
            let _ = variable_value.pop(); // remove newline
            if !variable_value.is_empty() {
              token_map.insert(UserVariableKey::new(v.variable_name.clone()), UserVariableValue::new(variable_value));
            } else {
              match dynamic_value {
                DynamicValueType::DefaultValue(_, dvalue) => {
                  token_map.insert(UserVariableKey::new(v.variable_name.clone()), UserVariableValue::new(dvalue));
                },
                DynamicValueType::PluginValue(_, pvalue) => {
                  token_map.insert(UserVariableKey::new(v.variable_name.clone()), UserVariableValue::new(pvalue));
                },
                DynamicValueType::Neither => (),
              }
            }
          }
        }
      }

      token_map
    }
}

impl UserTemplateVariableValidator for Cli {
    fn review_user_template_variables(&self, user_config: UserConfig, user_variables: HashMap<UserVariableKey, UserVariableValue>) -> TemplateVariableReview {
        Cli::print_user_input(&user_variables);
        match  Cli::check_user_input() {
          UserVariablesValidity::Valid => {
            let valid_config = ValidConfig::new(user_variables, user_config);
            TemplateVariableReview::Accepted(valid_config)
          },
          UserVariablesValidity::Invalid => TemplateVariableReview::Rejected,
        }
    }
}


impl Cli {

  fn print_user_input(user_variables: &HashMap<UserVariableKey, UserVariableValue>) {
    Logger::info("Please confirm the variable mappings below are correct.");

    for t in user_variables.iter() {
      p!("{} -> {}", Blue.paint(&t.0.value), Green.paint(&t.1.value))
    }
  }

  fn check_user_input() -> UserVariablesValidity {
    // Check if variables are ok
    Logger::coloured(
      &s!("{}{}{}",
        Yellow.paint("Press "),
        Style::new().bold().paint("y"),
        Yellow.paint(" if correct, and any other key if not.")
      )
    );

    let mut user_response = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut user_response).expect("Could not read from stdin"); // Unexpected, so throw
    let line = user_response.lines().next().expect("Could not extract line from buffer"); // Unexpected, so throw

    match &line[..] {
      "y" => UserVariablesValidity::Valid,
      _ => UserVariablesValidity::Invalid,
    }
  }
}


pub struct DefaultTemplateConfigValidator {
  user_input_provider: Box<dyn UserInputProvider>,
  user_template_variable_validator: Box<dyn UserTemplateVariableValidator>,
}


impl DefaultTemplateConfigValidator {

  pub fn new() -> Self {
    DefaultTemplateConfigValidator {
      user_input_provider: Box::new(Cli),
      user_template_variable_validator: Box::new(Cli)
    }
  }

  #[cfg(test)]
  fn with_all_dependencies(user_input_provider: Box<dyn UserInputProvider>, user_template_variable_validator: Box<dyn UserTemplateVariableValidator>) -> Self {
    DefaultTemplateConfigValidator {
      user_input_provider,
      user_template_variable_validator
    }
  }
}

impl TemplateConfigValidator for DefaultTemplateConfigValidator {

  fn validate(&self, user_config: UserConfig, template_variables: TemplateVariables) -> TemplateVariableReview {
      let user_variables = self.user_input_provider.get_user_input(template_variables);
      self.user_template_variable_validator.review_user_template_variables(user_config, user_variables)
  }
}

#[cfg(test)]
mod tests {

  use super::super::TemplateVariable;
  use super::*;
  use pretty_assertions::assert_eq;
  use crate::config::user_config::UserConfig;

  impl UserInputProvider for HashMap<String, String> {
    fn get_user_input(&self, variables: TemplateVariables) -> HashMap<UserVariableKey, UserVariableValue> {

      let pairs =
        variables
        .tokens
        .into_iter()
        .filter_map(|tv| {
          self.get(&tv.variable_name)
            .map(|variable|{
              (UserVariableKey::new(tv.variable_name.to_owned()), UserVariableValue::new(variable.to_owned()))
            })
        });

        HashMap::from_iter(pairs)
    }
  }


  struct RejectedUserTemplateVariables;

  struct AcceptedUserTemplateVariables {
    user_config: UserConfig,
    user_variables: HashMap<UserVariableKey, UserVariableValue>
  }

  impl From<&AcceptedUserTemplateVariables> for ValidConfig {
    fn from(field: &AcceptedUserTemplateVariables) -> Self {
        ValidConfig {
            user_variables: field.user_variables.clone(),
            user_config: field.user_config.clone(),
        }
    }
  }




  impl UserTemplateVariableValidator for RejectedUserTemplateVariables {
    fn review_user_template_variables(&self, _user_config_: UserConfig, _variables_: HashMap<UserVariableKey, UserVariableValue>) -> TemplateVariableReview {
        TemplateVariableReview::Rejected
    }
  }

  impl UserTemplateVariableValidator for AcceptedUserTemplateVariables {
    fn review_user_template_variables(&self, _user_config_: UserConfig, _variables_: HashMap<UserVariableKey, UserVariableValue>) -> TemplateVariableReview {
      let valid_config: ValidConfig = ValidConfig::from(self);
      TemplateVariableReview::Accepted(valid_config)
    }
  }


  impl Default for RejectedUserTemplateVariables {
    fn default() -> Self {
        RejectedUserTemplateVariables
    }
  }

  fn template_variable_from_name(name: &str) -> TemplateVariable {
    TemplateVariable {
      variable_name: name.to_owned(),
      description: String::default(),
      prompt: String::default(),
      default_value: None,
      plugin: None,
      filters: Vec::default(),
    }
  }

  fn user_template_variables(key_values: &[(&str, &str)]) -> HashMap<UserVariableKey, UserVariableValue> {
    key_values.into_iter().map(|kv|{
      (UserVariableKey::new(kv.0.to_owned()), UserVariableValue::new(kv.1.to_owned()))
    }).collect()
  }


  #[test]
  fn returns_valid_user_input() {
    let hash_map_input: HashMap<String, String> =
      HashMap::from([
        ("token1".to_owned(), "value1".to_owned()),
        ("token2".to_owned(), "value2".to_owned())
      ]);

    let template_variables =
      TemplateVariables {
        tokens: vec![
          template_variable_from_name("tokenA"),
          template_variable_from_name("tokenB"),
          template_variable_from_name("token1"),
          template_variable_from_name("token2"),
          template_variable_from_name("tokenC")
        ]
      };


    let validated_user_variables =
      user_template_variables(
        &[
          ("token1", "value1"),
          ("token2", "value2"),
        ]
      );


    let user_config =
      UserConfig::new("template_dir", "target_idr");

    let user_template_variables =
      AcceptedUserTemplateVariables {
        user_config: user_config.clone(),
        user_variables: validated_user_variables,
      };

    let config_validator = DefaultTemplateConfigValidator::with_all_dependencies(Box::new(hash_map_input), Box::new(user_template_variables));

    let validation_result = config_validator.validate(user_config.clone(), template_variables);

    let expected_config =
      ValidConfig {
        user_variables:
          HashMap::from([
            (UserVariableKey::new("token1".to_owned()), UserVariableValue::new("value1".to_owned())),
            (UserVariableKey::new("token2".to_owned()), UserVariableValue::new("value2".to_owned()))
          ]),
        user_config
      };

    assert_eq!(validation_result, TemplateVariableReview::Accepted(expected_config))
  }


  #[test]
  fn returns_rejected_input() {
    let hash_map_input = HashMap::new();
    let user_variable_validator = RejectedUserTemplateVariables::default();
    let config_validator = DefaultTemplateConfigValidator::with_all_dependencies(Box::new(hash_map_input), Box::new(user_variable_validator));
    let template_variables = TemplateVariables::default();

    let user_config =
      UserConfig::new("template_dir", "target_idr");

    let validation_result = config_validator.validate(user_config, template_variables);

    assert_eq!(validation_result, TemplateVariableReview::Rejected)
  }

}

