use std::collections::HashMap;

use crate::cli::Args;
use crate::shared_models::*;
use crate::template_token_provider::{TemplateTokenProvider, TemplateTokens};
use crate::user_config_provider::*;
use crate::cli;
use crate::models::{TargetDir, TemplateDir};
use crate::variables::TemplateVariable;
use std::fs::File;
use std::io::{Read, Write};

// impl Prod {
//   fn get_tokens() -> HashMap<String, String> {
//     todo!()

//     // if variables_file.exists() {
//     //       println!("Loading variables file");
//     //       let mut f = File::open(variables_file).map_err(|e| ZatError::IOError(e.to_string()))?;
//     //       let mut variables_json = String::new();

//     //       f.read_to_string(&mut variables_json).map_err(|e| ZatError::IOError(e.to_string()))?;

//     //       let variables: Vec<TemplateVariable> = serde_json::from_str(&variables_json).map_err(|e| ZatError::SerdeError(e.to_string()))?;
//     //       println!("loaded: {:?}", &variables);
//       }
// }

trait ArgSupplier {
  fn get_args(&self) -> Args;
}

struct Cli;

impl ArgSupplier for Cli {
  fn get_args(&self) -> Args {
    cli::get_cli_args()
  }
}

struct UnimplementedType;

impl ArgSupplier for UnimplementedType {
  fn get_args(&self) -> Args {
    todo!()
  }
}


pub struct Prod {
  arg_supplier: Box<dyn ArgSupplier>
}

impl Prod {
  fn new() -> Self {
    let cli = Cli;
    let arg_supplier = Box::new(cli);
    Prod::with_args_supplier(arg_supplier)
  }

  fn with_args_supplier(arg_supplier: Box<dyn ArgSupplier>) -> Self {
    Prod {
      arg_supplier
    }
  }
}

impl UserConfigProvider for Prod {
  fn get_config(&self) -> ZatResultX<UserConfig> {
    let args = self.arg_supplier.get_args();

    let template_dir = TemplateDir::new(&args.template_dir);
    let target_dir = TargetDir::new(&args.target_dir);

    let template_dir_exists = &template_dir.does_exist();
    let target_dir_exists = &target_dir.does_exist();

    if *template_dir_exists && !(*target_dir_exists) {

      let ignores = Ignores::default(); // TODO: Get this from the user

      Ok(
        UserConfig {
          // user_tokens,
          template_dir,
          target_dir,
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

// TODO: Should we move the file checks to a fake file system?
// That would make it easier to test and lead to more reuse of code
impl TemplateTokenProvider for Prod {
  fn get_tokens(&self, user_config: UserConfig) -> ZatResultX<TemplateTokens> {
      let variables_file: VariableFile = VariableFile::from(user_config.template_dir);

      if variables_file.does_exist() {
        let mut f = File::open(variables_file).map_err(|e| ZatErrorX::VariableReadError(e.to_string()))?;
        let mut variables_json = String::new();

        f.read_to_string(&mut variables_json).map_err(|e| ZatErrorX::VariableReadError(e.to_string()))?;

        let variables: Vec<TemplateVariable> = serde_json::from_str(&variables_json).map_err(|e| ZatErrorX::VariableDecodeError(e.to_string()))?;

        Ok(
          TemplateTokens{
            tokens: variables
          }
        )

      } else {
        Ok(
          TemplateTokens {
            tokens: vec![]
          }
        )
    }
  }
}

// TODO: How can I separate the tests for each module into their on mods?
// At the moment since Prod implements it all we need to keep it in the same
// file as Prod.
// Maybe we use types like FakeXYZ and RealXYZ. Then they can live in different files
#[cfg(test)]
mod tests {

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

    // Delete target_dir because it should not exist
    // We only create it to get a random directory name
    drop(target_dir);

    let args = TestArgs {
      args: Args {
        template_dir: template_dir_path.clone(),
        target_dir: target_dir_path.clone()
      }
    };

    let prod = Prod::with_args_supplier(Box::new(args));
    let config = prod.get_config().expect("Could not get config");

    let expected_template_dir = TemplateDir::new(&template_dir_path);
    let expected_ignores = Ignores::default();


    assert_eq!(config.template_dir, expected_template_dir);
    assert_eq!(&config.target_dir.path, &target_dir_path);
    assert_eq!(config.ignores, expected_ignores)
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
        target_dir: target_dir_path.clone()
      }
    };

    let prod = Prod::with_args_supplier(Box::new(args));
    match prod.get_config() {
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
        target_dir: target_dir_path.clone()
      }
    };

    let prod = Prod::with_args_supplier(Box::new(args));
    match prod.get_config() {
      Ok(_) => assert!(false, "get_config should fail if the target directory does exist"),
      Err(error) => {
        let expected_error = format!("Target directory should not exist, as it will be created: {}. Please supply an empty directory for the target", target_dir_path);
        assert_eq!(error, ZatErrorX::UserConfigError(expected_error))
      }
    }
  }

  #[test]
  fn tokens_are_empty_if_variable_file_does_not_exist() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    drop(target_dir);

    let prod = Prod::new();

    let user_config = UserConfig {
      template_dir: TemplateDir::new(&template_dir_path),
      target_dir: TargetDir::new(&target_dir_path),
      ignores: Ignores::default()
    };

    let tokens = prod.get_tokens(user_config).expect("Expected to get tokens");
    assert!(tokens.tokens.is_empty())
  }

  #[test]
  fn tokens_are_loaded_from_variable_file() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();
    let variable_file_path = template_dir.path().join(VariableFile::PATH);

    let mut variable_file = File::create(variable_file_path).unwrap();

    drop(target_dir);

    let variables_config = r#"
      [
        {
          "variable_name": "project",
          "description": "Name of project",
          "prompt": "Please enter your project name",
              "filters": [
                {
                  "name":"python",
                  "filter": "Snake"
                },
                { "name": "Command",
                  "filter": "Pascal"
                }
              ]
        },
        {
          "variable_name": "plugin_description",
          "description": "Explain what your plugin is about",
          "prompt": "Please enter your plugin description"
        }
      ]
    "#;

    writeln!(&mut variable_file, "{}", variables_config).unwrap();

    let prod = Prod::with_args_supplier(Box::new(UnimplementedType));

    let user_config = UserConfig {
      template_dir: TemplateDir::new(&template_dir_path),
      target_dir: TargetDir::new(&target_dir_path),
      ignores: Ignores::default()
    };

    let tokens = prod.get_tokens(user_config).expect("Expected to get tokens");
    assert_eq!(tokens.tokens.len(), 2);

    drop(variable_file);
  }


}
