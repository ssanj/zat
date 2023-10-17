use crate::{config::{UserConfig, ConfigShellHookStatus, TargetDir}, error::{ZatAction, ZatError}};

use super::PostProcessingHook;
use std::process::Command;
use std::path::Path;

pub struct ShellHook;

impl PostProcessingHook for ShellHook {
  fn run(&self, user_config: &UserConfig) -> ZatAction {
    match &user_config.shell_hook_status {
      ConfigShellHookStatus::NoShellHook => Ok(()),
      ConfigShellHookStatus::RunShellHook(shell_hook) => {
          Command::new(shell_hook)
            .arg::<&Path>(<TargetDir as AsRef<Path>>::as_ref(&user_config.target_dir).as_ref())
            .status()
            .map_err(|e| ZatError::PostProcessingError(format!("Shell hook did not complete as expected: {}", e)))
            .map(|exit|{
              println!("Shell hook exited with {}", exit);
              ()
            })
      }
    }
  }
}
