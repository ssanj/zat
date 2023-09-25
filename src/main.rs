mod args;
mod cli;
mod templates;

mod shared_models;
mod config;
mod token_expander;
mod processor;

mod source_file;
mod destination_file;

use std::eprintln;

use args::default_user_config_provider::DefaultUserConfigProvider;
use args::user_config_provider::UserConfigProvider;

use shared_models::ZatActionX;
use templates::template_variable_provider::TemplateVariableProvider;
use templates::default_template_variable_provider::DefaultTemplateVariableProvider;

use templates::template_config_validator::TemplateConfigValidator;
use templates::default_template_config_validator::DefaultTemplateConfigValidator;
use templates::template_config_validator::TemplateVariableReview;
use templates::template_config_validator::ValidConfig;

use token_expander::expand_filters::ExpandFilters;
use token_expander::default_expand_filters::DefaultExpandFilters;

use crate::processor::process_templates::ProcessTemplates;
use crate::processor::default_process_templates::DefaultProcessTemplates;

fn main() {
  match run_zat() {
    Ok(_) => println!("Zat completed successfully."),
    Err(error) => eprintln!("Zat failed with the following error: \n  {}", error),
  }
}

fn run_zat() -> ZatActionX {
  // Verifies that the source dir exists, and the destination does not and handles ignores (defaults and supplied).
  // Basically everything from the cli config.
  let config_provider = DefaultUserConfigProvider::new();
  let user_config = config_provider.get_config()?;

  // Reads the .variables.prompt file into TemplateVariables
  let template_variable_provider = DefaultTemplateVariableProvider::new();
  let template_variables = template_variable_provider.get_tokens(user_config.clone())?;

  // Ask for the user for the value of each variable
  // Then verify all the variables supplied are correct
  let template_config_validator = DefaultTemplateConfigValidator::new();
  let template_variable_review = template_config_validator.validate(user_config.clone(), template_variables.clone());

  println!("config: {:?}", user_config);
  println!("variables: {:?}", template_variables);
  println!("variable review: {:?}", template_variable_review);

  match template_variable_review {
    TemplateVariableReview::Accepted(ValidConfig { user_variables, user_config: _ }) => {
      let expand_filters = DefaultExpandFilters::new();
      let tokenized_key_expanded_variables = expand_filters.expand_filers(template_variables, user_variables);
      println!("tokenized variables: {:?}", &tokenized_key_expanded_variables);

      DefaultProcessTemplates.process_templates(user_config, tokenized_key_expanded_variables)?;
    },
    TemplateVariableReview::Rejected => println!("The user rejected the variables.")
  }

  Ok(())
}



