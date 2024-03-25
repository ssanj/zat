mod args;
mod templates;
mod error;
mod config;
mod token_expander;
mod processor;
mod post_processor;
mod macros;
mod logging;
mod workflow;
mod command;
mod plugin;
mod choice;

use error::ZatAction;
use logging::Logger;
use workflow::Workflow;

use std::process::ExitCode;

fn main() -> ExitCode {
  match run_zat() {
    Ok(_) => {
      Logger::success("Zat completed successfully.");
      ExitCode::SUCCESS
    },
    Err(error) => {
      Logger::error("Zat failed an with error.", error.to_string());
      ExitCode::FAILURE
    },
  }
}

fn run_zat() -> ZatAction {
  Workflow::execute()
}


