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
      .map_err(|e| ZatError::post_processing_hook_failed(shell_hook, e.to_string()))
      .and_then(|exit|{
        match exit.code() {
          Some(0) => {
            println!("Shell hook exited successfully");
            Ok(())
          },
          Some(other) => Err(ZatError::post_processing_hook_completed_with_non_zero_status(shell_hook, other)),
          None => Err(ZatError::post_processing_hook_was_shutdown(shell_hook))
        }
      })
}

#[cfg(test)]
mod tests {
    use crate::args::test_util::create_file_in;
    use super::*;
    use crate::{spath, assert_error_with};
    use crate::error::post_processing_error_reason::PostProcessingErrorReason;
    use std::format as s;

    #[test]
    fn should_do_nothing_when_there_is_no_shell_hook() {
        let config = default_config();
        assert_eq!(Ok(()), ShellHook.run(&config))
    }

    #[test]
    fn should_fail_when_the_shell_hook_should_exist_but_doesnt() {
        let config = config_with_shell_hook(default_config());

        let assert_error_ends_with =
          |error: String| assert!(error.ends_with("failed with an error."));

        assert_error_with!(
          ShellHook.run(&config),
          Err(ZatError::PostProcessingError(PostProcessingErrorReason::ExecutionError(error, ..))) => error,
          assert_error_ends_with
        )
    }

    #[test]
    fn should_fail_when_the_shell_hook_is_not_executable() {
        let source_dir =
          tempfile::TempDir::new()
            .unwrap();

        let target_dir =
          tempfile::TempDir::new()
            .unwrap();

        let shell_hook = source_dir.path().join("some-script.sh");
        let shell_hook_content = b"blee";
        let _ = create_file_in(source_dir.path(),  spath!(&shell_hook), shell_hook_content, None);

        println!("shell hook file exists: {}", shell_hook.exists());

        let config =
          config_with_shell_hook(
            config_with_source_and_target(source_dir.path(), target_dir.path()));

        println!("config: {:?}", &config);

        let assert_error_ends_with = |error_message: String| {
          let expected_error = s!("failed with an error.");
          assert!(error_message.ends_with(expected_error.as_str()), "Assertion did not match. Error received: {}", error_message.as_str())
        };

       assert_error_with!{
          ShellHook.run(&config),
          Err(ZatError::PostProcessingError(PostProcessingErrorReason::ExecutionError(error, ..))) => error,
          assert_error_ends_with
        }
    }

    #[test]
    fn should_run_shell_hook() {
        let source_dir =
          tempfile::TempDir::new()
            .unwrap();

        let target_dir =
          tempfile::TempDir::new()
            .unwrap();

        let shell_hook = source_dir.path().join("some-script.sh");
        let shell_hook_content = b"#!/bin/bash\ntouch \"$1\"/testing.txt";
        let _ = create_file_in(source_dir.path(),  spath!(&shell_hook), shell_hook_content, Some(0o755));

        println!("shell hook file exists: {}", shell_hook.exists());

        let config =
          config_with_shell_hook(
            config_with_source_and_target(source_dir.path(), target_dir.path()));

        println!("config: {:?}", &config);

        match ShellHook.run(&config) {
          Ok(_) => assert!(target_dir.path().join("testing.txt").exists()),
          other => panic!("expected Ok(..) but got: {:?}", other)
        }
    }

    fn default_config() -> UserConfig {
        UserConfig::new("", "")
    }

    fn config_with_source_and_target(source_dir: &Path, target_dir: &Path) -> UserConfig {
        UserConfig::new(spath!(&source_dir), spath!(&target_dir))
    }

    fn config_with_shell_hook(config: UserConfig) -> UserConfig {
        UserConfig {
          shell_hook_status: ConfigShellHookStatus::RunShellHook(spath!(config.template_dir.join("some-script.sh")).to_owned()),
          ..config
        }
    }
}
