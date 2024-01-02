use crate::args::{ArgSupplier, DefaultUserConfigProvider, ZatCommand};
use crate::args::cli_arg_supplier::CliArgSupplier;
use crate::command::{BootstrapProject, ProcessTemplates};
use crate::error::ZatAction;


pub struct Workflow;

impl Workflow {

  pub fn execute() -> ZatAction {
    // Verifies that the source dir exists, and the destination does not and handles ignores (defaults and supplied).
    // Basically everything from the cli config.
    let config_provider = DefaultUserConfigProvider::new();
    let config = CliArgSupplier.get_args();

    match config.command {
      ZatCommand::Process(process_templates_args) => {
        ProcessTemplates::process(config_provider, process_templates_args)
      },

      ZatCommand::Bootstrap(bootstrap_project_args) => {
        BootstrapProject::process_bootstrap(bootstrap_project_args)
      }
    }
  }

}