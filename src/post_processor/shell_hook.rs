use crate::{config::{UserConfig, ConfigShellHookStatus, TargetDir}, error::{ZatAction, ZatError}};

use super::PostProcessingHook;
use std::process::Command;
use std::path::Path;

pub struct ShellHook;

impl PostProcessingHook for ShellHook {

  fn run(&self, user_config: &UserConfig) -> ZatAction {
    match &user_config.shell_hook_status {
      ConfigShellHookStatus::NoShellHook => Ok(()),
      ConfigShellHookStatus::RunShellHook(shell_hook) => run_shell_hook(shell_hook, user_config)
    }
  }
}

fn run_shell_hook(shell_hook: &str, user_config: &UserConfig) -> Result<(), ZatError> {
    Command::new(shell_hook)
      .arg::<&Path>(<TargetDir as AsRef<Path>>::as_ref(&user_config.target_dir).as_ref())
      .status()
      .map_err(|e| ZatError::PostProcessingError(format!("Shell hook `{}` did not complete as expected: {}", shell_hook, e)))
      .map(|exit|{
        println!("Shell hook exited with {}", exit);
        ()
      })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_do_nothing_when_there_is_no_shell_hook() {
        let config = UserConfig::new("", "");
        assert_eq!(Ok(()), ShellHook.run(&config))
    }

    #[test]
    fn should_fail_when_the_shell_hook_should_exist() {
        let default_config = UserConfig::new("", "");

        let config = UserConfig {
          shell_hook_status: ConfigShellHookStatus::RunShellHook("some-script.sh".to_owned()),
          ..default_config
        };

        match ShellHook.run(&config) {
          Err(ZatError::PostProcessingError(error)) => {
            println!("shell hook error: {}", &error);
            assert!(error.starts_with("Shell hook `some-script.sh` did not complete as expected: No such file or directory"))
          },
          other => panic!("expected PostProcessingError but got: {:?}", other)
        }
    }
}
