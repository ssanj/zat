use std::path::Path;
use std::todo;
use crate::error::*;
use super::UserConfigProvider;
use super::cli::Args;
use super::cli;
use crate::config::UserConfig;
use crate::config::IgnoredFiles;
use crate::config::DOT_VARIABLES_PROMPT;
use crate::config::Filters;
use crate::config::TargetDir;
use crate::config::TemplateDir;
use crate::config::TemplateFilesDir;

pub trait ArgSupplier {
  fn get_args(&self) -> Args;
}

struct Cli;

impl ArgSupplier for Cli {
  fn get_args(&self) -> Args {
    cli::get_cli_args()
  }
}

#[derive(Debug, Clone, PartialEq)]
enum TemplateDirStatus {
  Exists,
  DoesNotExist
}

#[derive(Debug, Clone, PartialEq)]
enum TemplateDirTemplateFileStatus {
  Exists,
  DoesNotExist
}

#[derive(Debug, Clone, PartialEq)]
enum TargetDirStatus {
  Exists,
  DoesNotExist
}

#[derive(Debug, Clone, PartialEq)]
enum ShellHookStatus {
  Exists,
  DoesNotExist,
}

pub struct DefaultUserConfigProvider {
  arg_supplier: Box<dyn ArgSupplier>
}

impl DefaultUserConfigProvider {
  pub fn new() -> Self {
    let cli = Cli;
    let arg_supplier = Box::new(cli);
    DefaultUserConfigProvider::with_args_supplier(arg_supplier)
  }

  pub fn with_args_supplier(arg_supplier: Box<dyn ArgSupplier>) -> Self {
    Self {
      arg_supplier
    }
  }

  fn get_shell_hook_status(template_dir: &TemplateDir) -> ShellHookStatus {
    let shell_hook = template_dir.shell_hook_file();
    let shell_hook_exists = shell_hook.exists();

    if shell_hook_exists {
      ShellHookStatus::Exists
    } else {
      ShellHookStatus::DoesNotExist
    }
  }
}

impl UserConfigProvider for DefaultUserConfigProvider {
  fn get_config(&self) -> ZatResult<UserConfig> {
    let args = self.arg_supplier.get_args();

    let template_dir = TemplateDir::new(&args.template_dir);
    let target_dir = TargetDir::new(&args.target_dir);
    let template_files_dir = TemplateFilesDir::from(&template_dir);

    let template_dir_exists =
      if template_dir.does_exist() {
        TemplateDirStatus::Exists
      } else {
        TemplateDirStatus::DoesNotExist
      };

    let target_dir_exists =
      if target_dir.does_exist() {
        TargetDirStatus::Exists
      } else {
        TargetDirStatus::DoesNotExist
      };

    let template_files_dir_exists =
      if template_files_dir.does_exist() {
        TemplateDirTemplateFileStatus::Exists
      } else {
        TemplateDirTemplateFileStatus::DoesNotExist
      };


    let shell_hook_file_status =
      DefaultUserConfigProvider::get_shell_hook_status(&template_dir);


    let default_ignores = vec![DOT_VARIABLES_PROMPT.to_owned(), ".git".to_owned()];

    let ignores_with_defaults =
      default_ignores
        .into_iter()
        .chain(args.ignores.into_iter()); // use default ignores with user-supplied ignores

    let ignores = IgnoredFiles::from(ignores_with_defaults);

    match (template_dir_exists, template_files_dir_exists, target_dir_exists) {
      (TemplateDirStatus::DoesNotExist, _, _) => {
        let error = format!("Template directory does not exist: {}. It should exist so we can read the templates.", &template_dir.path());
        Err(ZatError::UserConfigError(error))
      },
      (TemplateDirStatus::Exists, TemplateDirTemplateFileStatus::DoesNotExist, _) => {
        let error = format!("Template directory does not have a 'template' subfolder. Expected this path to exist: {}. This is where we read the templates from.", &template_files_dir.path());
        Err(ZatError::UserConfigError(error))
      },
      (TemplateDirStatus::Exists, TemplateDirTemplateFileStatus::Exists, TargetDirStatus::Exists) => {
        let error = format!("Target directory should not exist, as it will be created: {}. Please supply an empty directory for the target", &target_dir.path);
        Err(ZatError::UserConfigError(error))
      },
      (TemplateDirStatus::Exists, TemplateDirTemplateFileStatus::Exists, TargetDirStatus::DoesNotExist) => {

        let filters = Filters::default();
        // TODO: Add executable status to the UserConfig - parse don't validate
        Ok(
          UserConfig {
            template_dir,
            template_files_dir: template_files_dir.clone(),
            target_dir,
            filters,
            ignores
          }
        )
      },
    }
  }
}


#[cfg(test)]
mod tests {
  use std::{vec, collections::HashSet};

  use crate::config::TEMPLATE_FILES_DIR;
  use super::*;
  use tempfile::TempDir;
  use super::super::test_util::{temp_dir_with, temp_dir_with_file_pair};

  struct TestArgs{
    args: Args
  }

  impl ArgSupplier for TestArgs {
    fn get_args(&self) -> Args {
      self.args.clone()
    }
  }


  #[test]
  fn config_is_loaded() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = temp_dir_with(TEMPLATE_FILES_DIR);

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    let ignores =
      vec![
        "blah".to_owned(),
        "blee/".to_owned(),
        ".blue".to_owned()
      ];

    // Delete target_dir because it should not exist
    // We only create it to get a random directory name
    drop(target_dir);

    let args = TestArgs {
      args: Args {
        template_dir: template_dir_path.clone(),
        target_dir: target_dir_path.clone(),
        ignores: ignores
      }
    };

    let user_config_provider = DefaultUserConfigProvider::with_args_supplier(Box::new(args));
    let config = user_config_provider.get_config().expect("Could not get config");

    let expected_template_dir = TemplateDir::new(&template_dir_path);
    let expected_template_files_dir = TemplateFilesDir::from(&expected_template_dir);
    let expected_filters = Filters::default();
    let mut expected_ignores =
      vec![
        "blah".to_owned(),
        "blee/".to_owned(),
        ".blue".to_owned(),
      ];

    expected_ignores.append(&mut IgnoredFiles::default_ignores());

    assert_eq!(config.template_dir, expected_template_dir);
    assert_eq!(config.template_files_dir, expected_template_files_dir);
    assert_eq!(&config.target_dir.path, &target_dir_path);
    assert_eq!(config.filters, expected_filters);

    let actual_ignores_set: HashSet<String> = HashSet::from_iter(config.ignores.ignores);
    let expected_ignores_set: HashSet<String> = HashSet::from_iter(expected_ignores);

    assert_eq!(actual_ignores_set, expected_ignores_set)
  }

  #[test]
  fn config_uses_default_ignores_if_not_supplied() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = temp_dir_with(TEMPLATE_FILES_DIR);

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    let ignores = vec![];

    // Delete target_dir because it should not exist
    // We only create it to get a random directory name
    drop(target_dir);

    let args = TestArgs {
      args: Args {
        template_dir: template_dir_path.clone(),
        target_dir: target_dir_path.clone(),
        ignores: ignores
      }
    };

    let user_config_provider = DefaultUserConfigProvider::with_args_supplier(Box::new(args));
    let config = user_config_provider.get_config().expect("Could not get config");

    let expected_template_dir = TemplateDir::new(&template_dir_path);
    let expected_template_files_dir = TemplateFilesDir::from(&expected_template_dir);
    let expected_filters = Filters::default();
    let expected_ignores = IgnoredFiles::default_ignores();

    assert_eq!(config.template_dir, expected_template_dir);
    assert_eq!(config.template_files_dir, expected_template_files_dir);
    assert_eq!(&config.target_dir.path, &target_dir_path);
    assert_eq!(config.filters, expected_filters);

    let actual_ignores_set: HashSet<String> = config.ignores.ignores;
    let expected_ignores_set: HashSet<String> = HashSet::from_iter(expected_ignores);

    assert_eq!(actual_ignores_set, expected_ignores_set)
  }

  #[test]
  fn config_fails_to_load_if_template_dir_does_not_exist() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    drop(target_dir);
    drop(template_dir);

    let args = TestArgs {
      args: Args {
        template_dir: template_dir_path.clone(),
        target_dir: target_dir_path.clone(),
        ignores: vec![]
      }
    };

    let user_config_provider = DefaultUserConfigProvider::with_args_supplier(Box::new(args));
    match user_config_provider.get_config() {
      Ok(_) => assert!(false, "get_config should fail if the template directory does not exist"),
      Err(error) => {
        let expected_error = format!("Template directory does not exist: {}. It should exist so we can read the templates.", template_dir_path);
        assert_eq!(error, ZatError::UserConfigError(expected_error))
      }
    }
  }


  #[test]
  fn config_fails_to_load_if_template_files_dir_does_not_exist() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();


    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    let args = TestArgs {
      args: Args {
        template_dir: template_dir_path.clone(),
        target_dir: target_dir_path.clone(),
        ignores: vec![]
      }
    };

    let user_config_provider = DefaultUserConfigProvider::with_args_supplier(Box::new(args));
    match user_config_provider.get_config() {
      Ok(_) => assert!(false, "get_config should fail if the template files directory does not exist"),
      Err(error) => {
        let expected_error = format!("Template directory does not have a 'template' subfolder. Expected this path to exist: {}/template. This is where we read the templates from.", template_dir_path);
        assert_eq!(error, ZatError::UserConfigError(expected_error))
      }
    }

    drop(target_dir);
    drop(template_dir);
  }

  #[test]
  fn config_fails_to_load_if_target_dir_exists() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = temp_dir_with(TEMPLATE_FILES_DIR);

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    let args = TestArgs {
      args: Args {
        template_dir: template_dir_path.clone(),
        target_dir: target_dir_path.clone(),
        ignores: vec![]
      }
    };

    let user_config_provider = DefaultUserConfigProvider::with_args_supplier(Box::new(args));
    match user_config_provider.get_config() {
      Ok(_) => assert!(false, "get_config should fail if the target directory does exist"),
      Err(error) => {
        let expected_error = format!("Target directory should not exist, as it will be created: {}. Please supply an empty directory for the target", target_dir_path);
        assert_eq!(error, ZatError::UserConfigError(expected_error))
      }
    }
  }


  mod shell_hook {
    use crate::{config::SHELL_HOOK_FILE, args::test_util::create_file_in};

    use super::*;

    #[test]
    fn shell_hook_not_found() {
      let temp_dir = TempDir::new().unwrap();
      let template_dir = TemplateDir::new(temp_dir.path().to_str().unwrap());

      let result = DefaultUserConfigProvider::get_shell_hook_status(&template_dir);

      assert_eq!(result, ShellHookStatus::DoesNotExist);
    }

    #[test]
    fn shell_hook_found() {
      let temp_dir = TempDir::new().unwrap();
      let template_dir = TemplateDir::new(temp_dir.path().to_str().unwrap());

      let content = b"testing";
      let _ = create_file_in(template_dir.as_ref(), SHELL_HOOK_FILE, content, None);

      let result = DefaultUserConfigProvider::get_shell_hook_status(&template_dir);

      assert_eq!(result, ShellHookStatus::Exists);
    }
  }

}
