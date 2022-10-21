use crate::cli::Args;
use crate::shared_models::*;
use crate::user_config::*;
use crate::cli;
use crate::models::{TargetDir, TemplateDir};
use std::collections::HashMap;


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

impl UserConfig for Prod {
  fn get_config(&self) -> ZatResultX<Config> {
    let args = self.arg_supplier.get_args();

    let template_dir = TemplateDir::new(&args.template_dir);
    let target_dir = TargetDir::new(&args.target_dir);

    let template_dir_exists = &template_dir.does_exist();
    let target_dir_exists = &target_dir.does_exist();

    if *template_dir_exists && !(*target_dir_exists) {

      let ignores = Ignores::default(); // TODO: Get this from the user

      Ok(
        Config {
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


}
