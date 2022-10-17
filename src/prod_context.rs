use crate::shared_models::*;
use crate::user_config::*;
use crate::cli;
use crate::models::{TargetDir, TemplateDir};
use std::collections::HashMap;

pub struct Prod;

impl Prod {
  fn get_tokens() -> HashMap<String, String> {
    todo!()

    // if variables_file.exists() {
    //       println!("Loading variables file");
    //       let mut f = File::open(variables_file).map_err(|e| ZatError::IOError(e.to_string()))?;
    //       let mut variables_json = String::new();

    //       f.read_to_string(&mut variables_json).map_err(|e| ZatError::IOError(e.to_string()))?;

    //       let variables: Vec<TemplateVariable> = serde_json::from_str(&variables_json).map_err(|e| ZatError::SerdeError(e.to_string()))?;
    //       println!("loaded: {:?}", &variables);
      }
}

impl UserConfig for Prod {
  fn get_config() -> ZatResultX<Config> {
    let cli_args = cli::get_cli_args();

    let template_dir = TemplateDir::new(&cli_args.template);
    let target_dir = TargetDir::new(&cli_args.destination);

    let template_path_exists = &template_dir.does_exist();
    let target_path_exists = &target_dir.does_exist();

    if *template_path_exists && !(*target_path_exists) {
      let user_tokens = Prod::get_tokens(); //TODO: Get this from the user
      let ignores = Ignores { // TODO: Get this from the user
        files: vec![],
        directories: vec![],
      };

      Ok(
        Config {
          user_tokens,
          template_dir,
          target_dir,
          ignores
        }
      )
    } else if !template_path_exists {
      let error = format!("Template path does not exist: {}", &template_dir.path);
      Err(ZatErrorX::UserConfigError(error))
    } else {
      let error = format!("Target path already exists: {}. Please supply an empty directory for the target", &target_dir.path);
      Err(ZatErrorX::UserConfigError(error))
    }
  }
}



