use std::ffi::OsStr;
use std::io::{stdin, BufRead};
use std::{fs::create_dir, collections::HashMap, path::Path};

use tokens::UserSelection;
use walkdir::{WalkDir, DirEntry};
use std::fs::{self, File};
use std::io::Read;
use crate::models::*;
use crate::variables::*;
use crate::cli::Args;
use aho_corasick::AhoCorasick;
use crate::variable_extractor::VariableExtractor;
use crate::behaviours::VariableValidationResponse;

mod models;
mod variables;
mod tokens;
mod cli;
mod template_processor;
mod behaviours;
mod prod;
mod variable_extractor;

fn main() {
  run_zat()
}

fn run_zat() {
  let instance = prod::Prod;

  let extractor = VariableExtractor {
    value: instance
  };

  let cli_args = cli::get_cli_args();

  let template_dir = TemplateDir::new(&cli_args.template);
  let target_dir = TargetDir::new(&cli_args.destination);

  let template_path_exists = does_path_exist(&template_dir); // Move this to a behaviour
  let target_path_exists = does_path_exist(&target_dir); // Move this to a behaviour

  if template_path_exists && !target_path_exists {
    let variables_file = Path::new(&template_dir.path).join(".variables.prompt"); // Move this to a behaviour

    match extractor.extract_variables(&variables_file) {
     Ok(VariableValidationResponse::UserQuit) => println!("~ Goodbye"),
     Ok(VariableValidationResponse::Continue(user_tokens_supplied)) => {
        match template_processor::process_template(&template_dir, &target_dir, user_tokens_supplied.0) {
          Ok(_) => {},
          Err(e) => eprintln!("Could not generate template: {}", e.inner_error())
        }
      },
      Err(_) => eprintln!("got an error!") // TODO: Fix
      // Err(ZatError::SerdeError(e)) => eprintln!("Could not decode variables.prompt file: {}", e),
      // Err(ZatError::IOError(e)) => eprintln!("Error read variables.prompt file: {}", e),
      // Err(ZatError::OtherError(e)) => eprintln!("An error occurred processing the variables.prompt file: {}", e)
    }
  } else if !template_path_exists {
    eprintln!("Template path does not exist: {}", &template_dir.path)
  } else {
    eprintln!("Target path already exists: {}. Please supply an empty directory for the target", &target_dir.path)
  }
}

fn does_path_exist<A>(path: A) -> bool where
  A: AsRef<OsStr>
{
  Path::new(&path).exists()
}



