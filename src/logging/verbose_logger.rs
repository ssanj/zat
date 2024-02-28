use crate::config::UserConfig;
use crate::templates::{TemplateVariables, ValidConfig};
use super::Printer;
use crate::token_expander::TokenizedKeysExpandedVariables;

pub struct VerboseLogger;

impl VerboseLogger {
  pub(crate) fn log_user_config(user_config: &UserConfig) {
    if user_config.verbose {
      Printer::print_verbose("User configuration", user_config);
    }
  }

  pub(crate) fn log_template_variables(user_config: &UserConfig, template_variables: &TemplateVariables) {
    if user_config.verbose {
      Printer::print_verbose("Template variables", template_variables);
    }
  }

  pub(crate) fn log_template_variables_after_plugins_run(user_config: &UserConfig, template_variables: &TemplateVariables) {
    if user_config.verbose {
      Printer::print_verbose("Template variables after plugins have run", template_variables);
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

  pub(crate) fn log_files_to_process(user_config: &UserConfig, files_to_process: Vec<String>) {
    if user_config.verbose {
      Printer::print_verbose_strings("Files to process", files_to_process)
    }
  }

  /// log_header is meant to be used with one or more log_content calls.
  pub(crate) fn log_header(user_config: &UserConfig, header: &str) {
    if user_config.verbose {
      Printer::heading_only(header);
    }
  }

  pub(crate) fn log_content(user_config: &UserConfig, content: &str) {
    if user_config.verbose {
      Printer::content_only(content);
    }
  }
}
