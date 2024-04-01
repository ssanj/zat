
use crate::config::user_config::MenuStyle;
use crate::config::ConfigShellHookStatus;
use crate::error::*;
use super::ChoiceMenuStyle;
use super::UserConfigProvider;
use super::cli::ProcessTemplatesArgs;
use crate::config::UserConfig;
use crate::config::IgnoredFiles;
use crate::config::Filters;
use crate::config::TargetDir;
use crate::config::RepositoryDir;
use crate::config::TemplateFilesDir;


#[derive(Debug, Clone, PartialEq)]
enum RepositoryDirStatus {
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

pub struct DefaultUserConfigProvider;

impl DefaultUserConfigProvider {
  pub fn new() -> Self {
    DefaultUserConfigProvider
  }
}

impl DefaultUserConfigProvider {
  fn get_shell_hook_status(template_dir: &RepositoryDir) -> ShellHookStatus {
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

  fn get_user_config(&self, args: ProcessTemplatesArgs) -> ZatResult<UserConfig> {
    let repository_dir = RepositoryDir::new(&args.repository_dir);
    let target_dir = TargetDir::new(&args.target_dir);
    let template_files_dir = TemplateFilesDir::from(&repository_dir);

    let repository_dir_exists =
      if repository_dir.does_exist() {
        RepositoryDirStatus::Exists
      } else {
        RepositoryDirStatus::DoesNotExist
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
      DefaultUserConfigProvider::get_shell_hook_status(&repository_dir);

    let ignores_with_defaults =
      IgnoredFiles::default_ignores()
        .into_iter()
        .chain(args.ignores); // use default ignores with user-supplied ignores

    let ignores = IgnoredFiles::from(ignores_with_defaults);

    let verbose = args.verbose;

    let choice_menu_style = args.choice_menu_style;

    match (repository_dir_exists, template_files_dir_exists, target_dir_exists) {
      (RepositoryDirStatus::DoesNotExist, _, _) => {
        Err(ZatError::template_dir_does_not_exist(repository_dir.path()))
      },
      (RepositoryDirStatus::Exists, TemplateDirTemplateFileStatus::DoesNotExist, _) => {
        Err(ZatError::template_files_dir_does_not_exist(template_files_dir.path()))
      },
      (RepositoryDirStatus::Exists, TemplateDirTemplateFileStatus::Exists, TargetDirStatus::Exists) => {
        Err(ZatError::target_dir_should_not_exist(&target_dir.path))
      },
      (RepositoryDirStatus::Exists, TemplateDirTemplateFileStatus::Exists, TargetDirStatus::DoesNotExist) => {

        let filters = Filters::default();

        let shell_hook_status = match shell_hook_file_status {
          ShellHookStatus::Exists => ConfigShellHookStatus::RunShellHook(repository_dir.shell_hook_file().to_string_lossy().to_string()),
          ShellHookStatus::DoesNotExist => ConfigShellHookStatus::NoShellHook
        };

        let menu_style = match choice_menu_style {
            Some(ChoiceMenuStyle::Numbered) => MenuStyle::Numbered,
            Some(ChoiceMenuStyle::Selection) => MenuStyle::Selection,
            None => MenuStyle::Numbered,
        };


        Ok(
          UserConfig {
            repository_dir,
            template_files_dir: template_files_dir.clone(),
            target_dir,
            filters,
            ignores,
            verbose,
            shell_hook_status,
            menu_style
          }
        )
      },
    }
  }
}


#[cfg(test)]
mod tests {
  use std::{vec, collections::HashSet};

  use crate::{args::cli::ChoiceMenuStyle, config::TEMPLATE_FILES_DIR};
  use super::*;
  use tempfile::TempDir;
  use super::super::test_util::temp_dir_with;
  use std::format as s;
  use crate::error::user_config_error_reason::UserConfigErrorReason;


  /// Returns UserConfig or panics on any errors.
  fn get_user_config(user_config_provider: impl UserConfigProvider, process_templates_args: ProcessTemplatesArgs) -> UserConfig {
    user_config_provider.get_user_config(process_templates_args).expect("Could not load user config")
  }

  fn get_user_config_fallable(user_config_provider: impl UserConfigProvider, process_templates_args: ProcessTemplatesArgs) -> ZatResult<UserConfig> {
      user_config_provider.get_user_config(process_templates_args)
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

    let args =
      ProcessTemplatesArgs {
        repository_dir: template_dir_path.clone(),
        target_dir: target_dir_path.clone(),
        ignores,
        verbose: false,
        choice_menu_style: Some(ChoiceMenuStyle::Numbered)
      };

    let user_config_provider = DefaultUserConfigProvider;
    let config = get_user_config(user_config_provider, args);

    let expected_repository_dir = RepositoryDir::new(&template_dir_path);
    let expected_template_files_dir = TemplateFilesDir::from(&expected_repository_dir);
    let expected_filters = Filters::default();
    let mut expected_ignores =
      vec![
        "blah".to_owned(),
        "blee/".to_owned(),
        ".blue".to_owned(),
      ];

    expected_ignores.append(&mut IgnoredFiles::default_ignores());

    assert_eq!(config.repository_dir, expected_repository_dir);
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
    let repository_dir = temp_dir_with(TEMPLATE_FILES_DIR);

    let repository_dir_path = repository_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    let ignores = vec![];

    // Delete target_dir because it should not exist
    // We only create it to get a random directory name
    drop(target_dir);

    let args =
      ProcessTemplatesArgs {
        repository_dir: repository_dir_path.clone(),
        target_dir: target_dir_path.clone(),
        ignores,
        verbose: false,
        choice_menu_style: Some(ChoiceMenuStyle::Numbered)
      };

    let user_config_provider = DefaultUserConfigProvider;
    let config = get_user_config(user_config_provider, args);

    let expected_repository_dir = RepositoryDir::new(&repository_dir_path);
    let expected_template_files_dir = TemplateFilesDir::from(&expected_repository_dir);
    let expected_filters = Filters::default();
    let expected_ignores = IgnoredFiles::default_ignores();

    assert_eq!(config.repository_dir, expected_repository_dir);
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
    let repository_dir = TempDir::new().unwrap();

    let repository_dir_path = repository_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    drop(target_dir);
    drop(repository_dir);

    let args =
      ProcessTemplatesArgs {
        repository_dir: repository_dir_path.clone(),
        target_dir: target_dir_path.clone(),
        ignores: vec![],
        verbose: false,
        choice_menu_style: Some(ChoiceMenuStyle::Numbered)
      };

    let user_config_provider = DefaultUserConfigProvider;
    match get_user_config_fallable(user_config_provider, args) {
      Ok(_) => panic!("get_config should fail if the repository directory does not exist"),
      Err(error) => {

        let expected_error = s!("The Zat repository directory '{}' does not exist. It should exist so Zat can read the template configuration.", repository_dir_path);
        let expected_fix = s!("Please create the Zat repository directory '{}' with the Zat folder structure. See `zat --help` for more.", repository_dir_path);
        assert_eq!(error, ZatError::ProcessCommandError(ProcessCommandErrorReason::UserConfigError(UserConfigErrorReason::RepositoryDirDoesNotExist(expected_error, expected_fix))))
      }
    }
  }


  #[test]
  fn config_fails_to_load_if_template_files_dir_does_not_exist() {
    let target_dir = TempDir::new().unwrap();
    let repository_dir = TempDir::new().unwrap();


    let repository_dir_path = repository_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    let args =
      ProcessTemplatesArgs {
        repository_dir: repository_dir_path.clone(),
        target_dir: target_dir_path.clone(),
        ignores: vec![],
        verbose: false,
        choice_menu_style: Some(ChoiceMenuStyle::Numbered)
      };

    let user_config_provider = DefaultUserConfigProvider;
    match get_user_config_fallable(user_config_provider, args) {
      Ok(_) => panic!("get_config should fail if the template files directory does not exist"),
      Err(error) => {
        let expected_error = s!("The Zat template files directory '{}/template' does not exist. It should exist so Zat can read the template files.", repository_dir_path);
        let expected_fix = s!("Please create the Zat template files directory '{}/template' with the necessary template files. See `zat --help` for more details.", repository_dir_path);
        assert_eq!(error, ZatError::ProcessCommandError(ProcessCommandErrorReason::UserConfigError(UserConfigErrorReason::TemplateFilesDirDoesNotExist(expected_error, expected_fix))))
      }
    }

    drop(target_dir);
    drop(repository_dir);
  }

  #[test]
  fn config_fails_to_load_if_target_dir_exists() {
    let target_dir = TempDir::new().unwrap();
    let repository_dir = temp_dir_with(TEMPLATE_FILES_DIR);

    let template_dir_path = repository_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    let args =
      ProcessTemplatesArgs {
        repository_dir: template_dir_path.clone(),
        target_dir: target_dir_path.clone(),
        ignores: vec![],
        verbose: false,
        choice_menu_style: Some(ChoiceMenuStyle::Numbered)
      };

    let user_config_provider = DefaultUserConfigProvider;
    match get_user_config_fallable(user_config_provider, args) {
      Ok(_) => panic!("get_config should fail if the target directory does exist"),
      Err(error) => {
        let expected_error = s!("The target directory '{}' should not exist. It will be created when Zat processes the template files.", target_dir_path);
        let expected_fix = "Please supply an empty directory for the target.".to_owned();
        assert_eq!(error, ZatError::ProcessCommandError(ProcessCommandErrorReason::UserConfigError(UserConfigErrorReason::TargetDirectoryShouldNotExist(expected_error, expected_fix))))
      }
    }
  }


  mod shell_hook {
    use crate::{config::SHELL_HOOK_FILE, args::test_util::create_file_in};

    use super::*;

    #[test]
    fn shell_hook_not_found() {
      let temp_dir = TempDir::new().unwrap();
      let repository_dir = RepositoryDir::new(temp_dir.path().to_str().unwrap());

      let result = DefaultUserConfigProvider::get_shell_hook_status(&repository_dir);

      assert_eq!(result, ShellHookStatus::DoesNotExist);
    }

    #[test]
    fn shell_hook_found() {
      let temp_dir = TempDir::new().unwrap();
      let repository_dir = RepositoryDir::new(temp_dir.path().to_str().unwrap());

      let content = b"testing";
      let _ = create_file_in(repository_dir.as_ref(), SHELL_HOOK_FILE, content, None);

      let result = DefaultUserConfigProvider::get_shell_hook_status(&repository_dir);

      assert_eq!(result, ShellHookStatus::Exists);
    }
  }

}
