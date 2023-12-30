use std::fs;
use std::path::Path;

use crate::args::BootstrapProjectArgs;
use crate::error::{ZatError, ZatAction};
use crate::config::{TargetDir, TemplateDir, DOT_VARIABLES_PROMPT, TemplateFilesDir};

pub struct BootstrapProject;

impl BootstrapProject {

  const VARIABLE_FILE: &'static str =
    r#"
    [
      {
        "variable_name": "project",
        "description": "Name of project",
        "prompt": "Please enter your project name",
        "filters": [
          { "name": "__default__",
            "filter": "Pascal"
          },
          { "name": "underscore",
            "filter": "Snake"
          }
        ]
      },
      {
        "variable_name": "description",
        "description": "What your project is about",
        "prompt": "Please a description of your project",
        "default_value": "Some project description"
      }
    ]
  "#;

  const README_MD: &'static str =
    r#"
      # $project$

      Welcome to your bootstrap project. This is a template file, because it has the `.tmpl` extension. A template file will have any tokens defined, replaced by values supplied by the user. __project__ is a token used in this file. It is defined in the `.variables.zat-prompt` file at the root of this project.
    "#;

  pub fn process_bootstrap(bootstrap_project_args: BootstrapProjectArgs) -> ZatAction {
    let repository_directory = TemplateDir::new(&bootstrap_project_args.repository_dir);

    if repository_directory.does_exist() {
      Err(ZatError::bootstrap_repository_dir_should_not_exist(&bootstrap_project_args.repository_dir))
    } else {
      let repository_path = Path::new(repository_directory.path());
      Self::create_directory(repository_path)?;
      Self::create_file(repository_path.join(DOT_VARIABLES_PROMPT), Self::VARIABLE_FILE)?;

      let template_files_dir = TemplateFilesDir::from(&repository_directory);
      let template_files_dir_path = Path::new(template_files_dir.path());
      Self::create_directory(template_files_dir_path)?;
      Self::create_file(template_files_dir_path.join("README.md"), Self::README_MD)?;

      // display a message on how to run the template

      Ok(())
    }
  }

  fn create_directory<P: AsRef<Path> + Clone>(path: P) -> ZatAction {
    fs::create_dir_all(path.clone()).map_err(|e| ZatError::could_not_create_bootstrap_repository(e, &AsRef::<Path>::as_ref(&path).to_string_lossy()))
  }

  fn create_file<P: AsRef<Path> + Clone, C: AsRef<[u8]>>(file_path: P, contents: C) -> ZatAction {
    fs::write(file_path.clone(), contents).map_err(|e| ZatError::could_not_create_bootstrap_file(e, &AsRef::<Path>::as_ref(&file_path).to_string_lossy()))
  }
}
