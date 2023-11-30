use crate::config::UserConfig;
use crate::templates::{TemplateVariables, ValidConfig};
use super::Printer;
use crate::token_expander::TokenizedKeysExpandedVariables;

pub struct Logger;

impl Logger {
  pub fn log_user_config(user_config: &UserConfig) {
    if user_config.verbose {
      Printer::print_verbose("User configuration", user_config);
    }
  }

  pub fn log_template_variables(user_config: &UserConfig, template_variables: &TemplateVariables) {
    if user_config.verbose {
      Printer::print_verbose("Template variables", template_variables);
    }
  }

  pub(crate) fn log_user_supplied_variables(user_config: &UserConfig, user_supplied_values: &ValidConfig) {
    if user_config.verbose {
      Printer::print_verbose("User Supplied Values", user_supplied_values);
    }
  }

  pub(crate) fn expanded_tokens(user_config: &UserConfig, expanded_tokens: &TokenizedKeysExpandedVariables) {
    if user_config.verbose {
      Printer::print_verbose("Expanded tokens", expanded_tokens)
    }
  }

}
