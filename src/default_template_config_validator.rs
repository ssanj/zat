use std::collections::HashMap;

use crate::template_config_validator::{TemplateConfigValidator, TemplateVariableReview, ValidConfig};
use crate::variables::{UserVariableValue, UserVariableKey, TemplateVariables};
use crate::user_config_provider::UserConfig;

trait UserInputProvider {
  fn get_user_input(&self, variables: TemplateVariables) -> HashMap<UserVariableKey, UserVariableValue>;
}

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


struct DefaultTemplateConfigValidator {
  user_input_provider: Box<dyn UserInputProvider>
}


impl DefaultTemplateConfigValidator {

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

  use crate::{models::{TemplateDir, TargetDir}, user_config_provider::Ignores};

use super::*;

  #[test]
  fn returns_valid_user_input() {
    let hash_map_input: HashMap<String, String> = HashMap::new();

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


    let config_validator = DefaultTemplateConfigValidator::with_user_input_provider(Box::new(hash_map_input));

    let validation_result = config_validator.validate(user_config.clone(), template_variables);
    let expected_config =
      ValidConfig {
        user_variables: HashMap::new(),
        user_config
      };

    assert_eq!(validation_result, TemplateVariableReview::Accepted(expected_config))
  }
}

