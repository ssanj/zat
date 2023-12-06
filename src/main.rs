mod args;
mod templates;
mod error;
mod config;
mod token_expander;
mod processor;
mod post_processor;
mod macros;
mod logging;

use std::process::ExitCode;

use args::UserConfigProvider;
use args::DefaultUserConfigProvider;

use error::ZatAction;

use templates::TemplateVariableProvider;
use templates::DefaultTemplateVariableProvider;
use templates::TemplateConfigValidator;
use templates::DefaultTemplateConfigValidator;
use templates::TemplateVariableReview;
use templates::ValidConfig;

use token_expander::ExpandFilters;
use token_expander::DefaultExpandFilters;

use crate::processor::ProcessTemplates;
use crate::processor::DefaultProcessTemplates;

use crate::post_processor::PostProcessingHook;
use crate::post_processor::ShellHook;

use ansi_term::Color::{Red, Yellow};
use logging::VerboseLogger;
use std::{println as p, eprintln as e};

fn main() -> ExitCode {
  match run_zat() {
    Ok(_) => {
      p!("\n{}", Yellow.paint("Zat completed successfully."));
      ExitCode::SUCCESS
    },
    Err(error) => {
      e!("\n{}{}", Red.paint("Zat failed an with error."), error);
      ExitCode::FAILURE
    },
  }
}

fn run_zat() -> ZatAction {
  // Verifies that the source dir exists, and the destination does not and handles ignores (defaults and supplied).
  // Basically everything from the cli config.
  let config_provider = DefaultUserConfigProvider::new();
  let user_config = config_provider.get_config()?;
  VerboseLogger::log_user_config(&user_config);

  // Reads the .variables.zat-prompt file into TemplateVariables
  let template_variable_provider = DefaultTemplateVariableProvider::new();
  let template_variables = template_variable_provider.get_tokens(user_config.clone())?;
  VerboseLogger::log_template_variables(&user_config, &template_variables);

  // Ask for the user for the value of each variable
  // Then verify all the variables supplied are correct
  let template_config_validator = DefaultTemplateConfigValidator::new();
  let template_variable_review = template_config_validator.validate(user_config.clone(), template_variables.clone());

  match template_variable_review {
    TemplateVariableReview::Accepted(vc) => {
      VerboseLogger::log_user_supplied_variables(&user_config, &vc);
      let user_variables = vc.user_variables;
      let expand_filters = DefaultExpandFilters::new();
      let tokenized_key_expanded_variables = expand_filters.expand_filers(template_variables, user_variables);

      VerboseLogger::expanded_tokens(&user_config, &tokenized_key_expanded_variables);
      DefaultProcessTemplates.process_templates(user_config.clone(), tokenized_key_expanded_variables)?;

      // Run post-processor if one exists
      ShellHook.run(&user_config)?
    },
    TemplateVariableReview::Rejected => p!("\n{}", Red.paint("The user rejected the variables."))
  }

  Ok(())
}


