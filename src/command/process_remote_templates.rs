use crate::config::RepositoryDir;
use crate::error::{ZatAction, ZatError, ZatResult};
use crate::args::{ProcessRemoteTemplatesArgs, ProcessTemplatesArgs, UserConfigProvider};
use crate::logging::Logger;
use std::process::Command;
use std::format as s;
use tempfile::TempDir;
use url::Url;
use std::fs;
use super::ProcessTemplates;


pub struct ProcessRemoteTemplates;


impl ProcessRemoteTemplates {

  pub fn process_remote(config_provider: impl UserConfigProvider, process_remote_template_args : ProcessRemoteTemplatesArgs) -> ZatAction {
    let checkout_directory: TempDir = Self::create_checkout_directory(&process_remote_template_args.repository_url)?;

    let checkout_directory_path = checkout_directory.path().to_string_lossy().to_string();
    let repository_directory = RepositoryDir::new(&checkout_directory_path);
    clone_git_repository(&process_remote_template_args, &repository_directory)?;

    // Invoke the regular ProcessTemplates::process at this point
    let process_template_args = create_process_templates_args(repository_directory, process_remote_template_args);
    let user_config = config_provider.get_user_config(process_template_args)?;
    let result = ProcessTemplates::process(user_config);

    checkout_directory
      .close()
      .unwrap_or_else(|e| Logger::warn(&s!("Could not remove temporary folder '{}', reason: {}", checkout_directory_path, e)));

    result
  }


  fn create_checkout_directory(repository_url: &str) -> ZatResult<TempDir> {
    let url = Url::parse(repository_url)
      .map_err(|e| ZatError::invalid_remote_repository_url(e.to_string(), repository_url))?;

    let hostname = &url.host_str().ok_or_else(|| ZatError::unsupported_hostname(url.as_str()))?;
    let path = &url.path();

    // We can't use Path to join the pieces here, because the 'path' segment has a leading '/' which
    // clears the rest of the path. This is documented in Path.join.
    let repository_path = s!("zat-{}{}_", &hostname, &path.replace('/', "_"));

    let checkout_dir =
      tempfile::Builder::new()
        .prefix(&repository_path)
        .tempdir()
        .map_err(|e| ZatError::could_not_create_checkout_directory(e.to_string()))?;

    fs::create_dir_all(checkout_dir.path())
      .map_err(|e| ZatError::could_not_create_checkout_directory_structure(e.to_string(), &repository_path, &url))
      .map(|_| {
        checkout_dir
      })
  }
}

fn create_process_templates_args(repository_directory: RepositoryDir, process_remote_templates_args: ProcessRemoteTemplatesArgs) -> ProcessTemplatesArgs {
  ProcessTemplatesArgs {
    repository_dir: repository_directory.path().to_owned(),
    target_dir: process_remote_templates_args.target_dir,
    ignores: process_remote_templates_args.ignores,
    verbose: process_remote_templates_args.verbose
  }
}

fn clone_git_repository(process_remote_template_args: &ProcessRemoteTemplatesArgs, repository_dir: &RepositoryDir) -> ZatAction {

  let status_result =
    Command::new("git")
      .env("GIT_TERMINAL_PROMPT" , "0")
      .arg("clone")
      .arg(&process_remote_template_args.repository_url)
      .arg(repository_dir.path())
      .status();

  let program = s!("GIT_TERMINAL_PROMPT=0 git clone {} {}", &process_remote_template_args.repository_url, &repository_dir.path());

  let status = status_result.map_err(|e| {
    ZatError::git_clone_error(e.to_string(), &program, &process_remote_template_args.repository_url, repository_dir.path())
  })?;

  // TODO: Write a function to generate this from Command.
  let program_2 = s!("GIT_TERMINAL_PROMPT=0 git clone {}", &process_remote_template_args.repository_url);

  if !status.success() {
    Err(
      ZatError::git_clone_status_error(status.code(), &program_2, &process_remote_template_args.repository_url)
    )
  } else {
    Ok(())
  }
}

