use crate::{config::{UserConfig, ConfigShellHookStatus}, error::ZatAction};

use super::PostProcessingHook;

pub struct ShellHook;

impl PostProcessingHook for ShellHook {
  fn run(&self, user_config: &UserConfig) -> ZatAction {

    let outcome = match &user_config.shell_hook_status {
      ConfigShellHookStatus::NoShellHook => "No shell hook found.".to_owned(),
      ConfigShellHookStatus::RunShellHook(shell_hook) => format!("Shell hook found: {}, executing", shell_hook)
    };

    println!("{}", outcome);

    Ok(())
  }
}
