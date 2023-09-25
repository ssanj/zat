use crate::shared_models::*;
use crate::args::user_config_provider::*;
use super::cli::Args;
use super::cli;
use crate::config::user_config::UserConfig;
use crate::config::ignored_files::IgnoredFiles;
use crate::config::variable_file::VariableFile;
use crate::config::filters::Filters;
use crate::config::target_directory::TargetDir;
use crate::config::template_directory::TemplateDir;

pub trait ArgSupplier {
  fn get_args(&self) -> Args;
}

struct Cli;

impl ArgSupplier for Cli {
  fn get_args(&self) -> Args {
    cli::get_cli_args()
  }
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
}

impl UserConfigProvider for DefaultUserConfigProvider {
  fn get_config(&self) -> ZatResultX<UserConfig> {
    let args = self.arg_supplier.get_args();

    let template_dir = TemplateDir::new(&args.template_dir);
    let target_dir = TargetDir::new(&args.target_dir);

    let template_dir_exists = &template_dir.does_exist();
    let target_dir_exists = &target_dir.does_exist();

    let default_ignores = vec![VariableFile::PATH.to_owned(), ".git".to_owned()];

    let ignores_with_defaults =
      default_ignores
        .into_iter()
        .chain(args.ignores.into_iter());

    let ignores = IgnoredFiles::from(ignores_with_defaults);

    if *template_dir_exists && !(*target_dir_exists) {

      let filters = Filters::default(); // TODO: Get this from the user

      Ok(
        UserConfig {
          // user_tokens,
          template_dir,
          target_dir,
          filters,
          ignores
        }
      )
    } else if !template_dir_exists {
      let error = format!("Template directory does not exist: {}. It should exist so we can read the templates.", &template_dir.path);
      Err(ZatErrorX::UserConfigError(error))
    } else {
      let error = format!("Target directory should not exist, as it will be created: {}. Please supply an empty directory for the target", &target_dir.path);
      Err(ZatErrorX::UserConfigError(error))
    }
  }
}


// TODO: How can I separate the tests for each module into their on mods?
// At the moment since Prod implements it all we need to keep it in the same
// file as Prod.
// Maybe we use types like FakeXYZ and RealXYZ. Then they can live in different files
#[cfg(test)]
mod tests {

  use std::{vec, collections::HashSet};

use super::*;
  use tempfile::TempDir;

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
    let template_dir = TempDir::new().unwrap();

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
    let expected_filters = Filters::default();
    let mut expected_ignores =
      vec![
        "blah".to_owned(),
        "blee/".to_owned(),
        ".blue".to_owned(),
      ];

    expected_ignores.append(&mut IgnoredFiles::default_ignores());

    assert_eq!(config.template_dir, expected_template_dir);
    assert_eq!(&config.target_dir.path, &target_dir_path);
    assert_eq!(config.filters, expected_filters);

    let actual_ignores_set: HashSet<String> = HashSet::from_iter(config.ignores.ignores);
    let expected_ignores_set: HashSet<String> = HashSet::from_iter(expected_ignores);

    assert_eq!(actual_ignores_set, expected_ignores_set)
  }

  #[test]
  fn config_uses_default_ignores_if_not_supplied() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

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
    let expected_filters = Filters::default();
    let expected_ignores = IgnoredFiles::default_ignores();

    assert_eq!(config.template_dir, expected_template_dir);
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
        assert_eq!(error, ZatErrorX::UserConfigError(expected_error))
      }
    }
  }

  #[test]
  fn config_fails_to_load_if_target_dir_exists() {
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
      Ok(_) => assert!(false, "get_config should fail if the target directory does exist"),
      Err(error) => {
        let expected_error = format!("Target directory should not exist, as it will be created: {}. Please supply an empty directory for the target", target_dir_path);
        assert_eq!(error, ZatErrorX::UserConfigError(expected_error))
      }
    }
  }

}
