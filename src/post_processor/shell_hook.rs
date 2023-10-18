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
    use crate::{args::test_util::create_file_in};

    use super::*;

    #[test]
    fn should_do_nothing_when_there_is_no_shell_hook() {
        let config = default_config();
        assert_eq!(Ok(()), ShellHook.run(&config))
    }

    #[test]
    fn should_fail_when_the_shell_hook_should_exist_but_doesnt() {
        let config = config_with_shell_hook(default_config());

        match ShellHook.run(&config) {
          Err(ZatError::PostProcessingError(error)) => {
            println!("shell hook error: {}", &error);
            assert!(error.starts_with("Shell hook `/some-script.sh` did not complete as expected: No such file or directory"))
          },
          other => panic!("expected PostProcessingError but got: {:?}", other)
        }
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
        let _ = create_file_in(source_dir.path(),  &shell_hook.as_path().to_string_lossy().to_string(), shell_hook_content, None);

        println!("shell hook file exists: {}", shell_hook.exists());

        let config =
          config_with_shell_hook(
            config_with_source_and_target(source_dir.path(), target_dir.path()));

        println!("config: {:?}", &config);

        match ShellHook.run(&config) {
          Err(ZatError::PostProcessingError(error)) => {
            println!("shell hook error: {}", &error);
            let expected_error = format!("Shell hook `{}` did not complete as expected: Permission denied", shell_hook.to_string_lossy().to_string());
            assert!(error.starts_with(expected_error.as_str()))
          },
          other => panic!("expected PostProcessingError but got: {:?}", other)
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
        let _ = create_file_in(source_dir.path(),  &shell_hook.as_path().to_string_lossy().to_string(), shell_hook_content, Some(0o755));

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
        UserConfig::new(&source_dir.to_string_lossy().to_string(), &target_dir.to_string_lossy().to_string())
    }

    fn config_with_target_dir(config: UserConfig, target_dir: &Path) -> UserConfig {
      UserConfig {
        target_dir: TargetDir::from(target_dir),
        ..config
      }
    }

    fn config_with_shell_hook(config: UserConfig) -> UserConfig {
        UserConfig {
          shell_hook_status: ConfigShellHookStatus::RunShellHook(format!("{}/{}", config.template_dir.path(), "some-script.sh")),
          ..config
        }
    }
}
