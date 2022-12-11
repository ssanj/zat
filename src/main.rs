use std::ffi::OsStr;
use std::io::{stdin, BufRead};
use std::{fs::create_dir, collections::HashMap, path::Path};

use tokens::UserSelection;
use walkdir::{WalkDir, DirEntry};
use std::fs::{self, File};
use std::io::Read;
use crate::models::*;
use crate::template_config_validator::{ValidConfig, TemplateVariableReview};
use crate::variables::*;
use crate::cli::Args;
use aho_corasick::AhoCorasick;


mod models;
mod variables;
mod tokens;
mod cli;
mod template_processor;
mod user_config_provider;
mod template_variable_provider;
mod template_config_validator;
mod template_selector;
mod template_proc;
mod template_renderer;
mod token_replacer;
mod shared_models;
mod default_user_config_provider;
mod default_template_variable_provider;
mod default_template_config_validator;
mod template_variable_expander;
mod default_template_variable_expander;
mod filter_applicator;
mod convert_case_filter_applicator;
mod aho_corasick_token_replacer;

fn main() {
  run_zat()
}

fn alternate_run_zat() {
  todo!()
}

fn run_zat() {
  use default_user_config_provider::DefaultUserConfigProvider;
  use user_config_provider::UserConfigProvider;

  use template_variable_provider::TemplateVariableProvider;
  use default_template_variable_provider::DefaultTemplateVariableProvider;

  use template_config_validator::TemplateConfigValidator;
  use default_template_config_validator::DefaultTemplateConfigValidator;

  use template_variable_expander::TemplateVariableExpander;
  use default_template_variable_expander::DefaultTemplateVariableExpander;
  use convert_case_filter_applicator::ConvertCaseFilterApplicator;

  let config_provider = DefaultUserConfigProvider::new();
  let user_config = config_provider.get_config().unwrap();

  let template_variable_provider = DefaultTemplateVariableProvider::new();
  let template_variables = template_variable_provider.get_tokens(user_config.clone()).unwrap();

  let template_config_validator = DefaultTemplateConfigValidator::new();
  let template_variable_review = template_config_validator.validate(user_config.clone(), template_variables.clone());

  let filter_applicator = ConvertCaseFilterApplicator;
  let template_variable_expander = DefaultTemplateVariableExpander::with_filter_applicator(Box::new(filter_applicator));

  println!("config: {:?}", user_config);
  println!("variables: {:?}", template_variables);
  println!("variable review: {:?}", template_variable_review);

  match template_variable_review {
    TemplateVariableReview::Accepted(ValidConfig { user_variables, user_config: _ }) => {
      let expanded_variables = template_variable_expander.expand_filters(template_variables.clone(), user_variables);
      println!("expanded variables: {:?}", expanded_variables);
    },
    TemplateVariableReview::Rejected => println!("The user rejected the variables.")
  }



  // let cli_args = cli::get_cli_args();

  // let template_dir = TemplateDir::new(&cli_args.template_dir);
  // let target_dir = TargetDir::new(&cli_args.target_dir);

  // let template_path_exists = does_path_exist(&template_dir);
  // let target_path_exists = does_path_exist(&target_dir);

  // if template_path_exists && !target_path_exists {
  //   let variables_file = Path::new(&template_dir.path).join(".variables.prompt");

  //   match tokens::load_variables(&variables_file) {
  //    Ok(UserSelection::Exit) => println!("~ Goodbye"),
  //    Ok(UserSelection::Continue(user_tokens_supplied)) => {
  //       match template_processor::process_template(&template_dir, &target_dir, user_tokens_supplied) {
  //         Ok(_) => {},
  //         Err(e) => eprintln!("Could not generate template: {}", e.inner_error())
  //       }
  //     },
  //     Err(ZatError::SerdeError(e)) => eprintln!("Could not decode variables.prompt file: {}", e),
  //     Err(ZatError::IOError(e)) => eprintln!("Error read variables.prompt file: {}", e),
  //     Err(ZatError::OtherError(e)) => eprintln!("An error occurred processing the variables.prompt file: {}", e)
  //   }
  // } else if !template_path_exists {
  //   eprintln!("Template path does not exist: {}", &template_dir.path)
  // } else {
  //   eprintln!("Target path already exists: {}. Please supply an empty directory for the target", &target_dir.path)
  // }
}

fn does_path_exist<A>(path: A) -> bool where
  A: AsRef<OsStr>
{
  Path::new(&path).exists()
}



