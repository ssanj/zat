use std::collections::HashMap;

use crate::template_config_validator::{TemplateConfigValidator, TemplateVariableReview, ValidConfig};
use crate::variables::{UserVariableValue, UserVariableKey, TemplateVariables};
use crate::user_config_provider::UserConfig;

// This is a support trait to TemplateConfigValidator, so we define it here as opposed to in its own module.
trait UserInputProvider {
  fn get_user_input(&self, variables: TemplateVariables) -> HashMap<UserVariableKey, UserVariableValue>;
}

//TODO: Should we move this into a common models package?
struct Cli;


impl UserInputProvider for Cli {
    fn get_user_input(&self, template_variables: TemplateVariables) -> HashMap<UserVariableKey, UserVariableValue> {
      let stdin = std::io::stdin();
      let mut token_map = HashMap::new();
      println!("");

      for v in template_variables.tokens {
        println!("{}. {}", v.description, v.prompt);
        let mut variable_value = String::new();
        if let Ok(read_count) = stdin.read_line(&mut variable_value) {
          if read_count > 0 { //read at least one character
            let _ = variable_value.pop(); // remove newline
            if !variable_value.is_empty() {
              token_map.insert(UserVariableKey::new(v.variable_name.clone()), UserVariableValue::new(variable_value));
            }
          }
        }
      }

      token_map
    }
}

pub struct DefaultTemplateConfigValidator {
  user_input_provider: Box<dyn UserInputProvider>
}


impl DefaultTemplateConfigValidator {

  pub fn new() -> Self {
    DefaultTemplateConfigValidator {
      user_input_provider: Box::new(Cli)
    }
  }

  fn with_user_input_provider(user_input_provider: Box<dyn UserInputProvider>) -> Self {
    DefaultTemplateConfigValidator {
      user_input_provider
    }
  }
}

impl TemplateConfigValidator for DefaultTemplateConfigValidator {

  fn validate(&self, user_config: UserConfig, template_variables: TemplateVariables) -> TemplateVariableReview {
      let user_variables = self.user_input_provider.get_user_input(template_variables);

      let valid_config =
        ValidConfig {
            user_variables,
            user_config
        };

      TemplateVariableReview::Accepted(valid_config)
  }
}

#[cfg(test)]
mod tests {

  use crate::{models::{TemplateDir, TargetDir}, user_config_provider::Ignores, variables::TemplateVariable};
  use super::*;
  use pretty_assertions::assert_eq;


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


  struct RejectedUserInput;


  impl TemplateConfigValidator for RejectedUserInput {
    fn validate(&self, _user_config: UserConfig, _template_variabless: TemplateVariables) -> TemplateVariableReview {
        TemplateVariableReview::Rejected
    }
  }


  impl Default for RejectedUserInput {
    fn default() -> Self {
        RejectedUserInput
    }
  }

  fn template_variable_from_name(name: &str) -> TemplateVariable {
    TemplateVariable {
      variable_name: name.to_owned(),
      description: String::default(),
      prompt: String::default(),
      filters: Vec::default(),
    }
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

    let user_config =
      UserConfig {
        template_dir: TemplateDir::new("template_dir"),
        target_dir: TargetDir::new("target_idr"),
        ignores: Ignores::default()
    };


    let config_validator = DefaultTemplateConfigValidator::with_user_input_provider(Box::new(hash_map_input));

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
    let template_variables =
      TemplateVariables {
        tokens: vec![]
      };

    let user_config =
      UserConfig {
        template_dir: TemplateDir::new("template_dir"),
        target_dir: TargetDir::new("target_idr"),
        ignores: Ignores::default()
    };


    let validation_result = RejectedUserInput.validate(user_config, template_variables);

    assert_eq!(validation_result, TemplateVariableReview::Rejected)
  }
}

