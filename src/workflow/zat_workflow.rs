use crate::args::{ArgSupplier, DefaultUserConfigProvider, ZatCommand, UserConfigProvider};
use crate::args::cli_arg_supplier::CliArgSupplier;
use crate::command::{BootstrapProject, ProcessTemplates, ProcessRemoteTemplates};
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
        let user_config = config_provider.get_user_config(process_templates_args)?;
        ProcessTemplates::process(user_config)
      },

      ZatCommand::Bootstrap(bootstrap_project_args) => {
        BootstrapProject::process_bootstrap(bootstrap_project_args)
      },

      ZatCommand::ProcessRemote(process_remote_template_args) => {
        ProcessRemoteTemplates::process_remote(config_provider, process_remote_template_args)
      },
    }
  }

}
