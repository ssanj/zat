use std::path::Path;
use std::todo;

use crate::config::RepositoryDir;
use crate::error::{ZatAction, ZatError, ZatResult};
use crate::args::ProcessRemoteTemplatesArgs;
use crate::logging::Logger;
use std::process::Command;
use std::format as s;
use dirs::home_dir;
use url::Url;
use std::fs::{self, FileType, Metadata};
use crate::spath;

pub struct ProcessRemoteTemplates;

enum RepositoryDirType {
  Existing(String),
  Created(String)
}

impl ProcessRemoteTemplates {

  pub fn process_remote(process_remote_template_args : ProcessRemoteTemplatesArgs) -> ZatAction {
    let home_dir = Self::create_home_directory()?;
    let repository_dir_status = Self::create_repository_directory(&process_remote_template_args.repository_url)?;

    let repository_directory = match repository_dir_status {
        RepositoryDirType::Existing(repository_dir) => RepositoryDir::new(&repository_dir),
        RepositoryDirType::Created(repository_dir) => {
          clone_git_repository(&process_remote_template_args, &repository_dir)?;
          RepositoryDir::new(&repository_dir)
        },
    };

    // Create ProcessTemplateArgs
    // Call ProcessTemplates::process(..)

    todo!()
  }

  fn create_home_directory() -> ZatResult<String> {
    let home_dir = home_dir().ok_or_else(|| ZatError::home_directory_does_not_exist())?;

    // We choose not to create the home_dir if it does not exist. It seems a bit much to create
    // a user's home directory for a simple tool.
    let file_type =
      fs::metadata(&home_dir)
        .or_else(|e| Err(ZatError::could_not_get_home_directory_metadata(e.to_string(), spath!(home_dir))))
        .map(|md: Metadata| md.file_type())?;

    if file_type.is_dir() {
      Ok(spath!(&home_dir).clone())
    } else {
      Err(ZatError::home_directory_is_not_a_directory(spath!(home_dir)))
    }
  }

  fn create_repository_directory(repository_url: &str) -> ZatResult<RepositoryDirType> {
    let url = Url::parse(repository_url)
      .map_err(|e| ZatError::invalid_remote_repository_url(e.to_string(), repository_url))?;

    let hostname = &url.host().ok_or_else(|| ZatError::unsupported_hostname(&url.as_str()))?;
    let path = &url.path();

    let repository_path = s!("{}{}", hostname, path);
    // We may want to Git pull on this directory in the future, maybe based on a flag.
    // For the moment we just use it as a cache.
    if Path::new(&repository_path).exists() {
      Ok(RepositoryDirType::Existing(repository_path))
    } else {
      fs::create_dir_all(&repository_path)
        .or_else(|e| Err(ZatError::could_not_create_local_repository_directory(e.to_string(), &repository_path)))
        .map(|_| {
          Logger::info(&s!("Created local checkout '{}' for remote repository '{}'", &repository_path, &repository_url));
          RepositoryDirType::Created(repository_path)
        })
    }
  }
}

fn clone_git_repository(process_remote_template_args: &ProcessRemoteTemplatesArgs, repository_dir: &str) -> ZatAction {
  // let status =
  //   Command::new("git")
  //       .env("GIT_TERMINAL_PROMPT" , "0")
  //       .arg("clone")
  //       .arg(&process_remote_template_args.repository_url)
  //       .arg(s!("/Users/sanj/ziptemp/test-zat-templates/checked-out/{}", &process_remote_template_args.repository_url)) //TODO: where do we extract this out to?
  //       .status()
  //       .expect("clone failed");
  todo!()
}



