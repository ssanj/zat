use std::fs;
use std::path::Path;
use std::format as s;

use crate::args::BootstrapProjectArgs;
use crate::error::{ZatError, ZatAction};
use crate::config::{RepositoryDir, DOT_VARIABLES_PROMPT, TemplateFilesDir};
use crate::logging::Logger;
use crate::spath;

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
            "filter": "Noop"
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

  const README_MD_TMPL: &'static str =
    r#"
      # $project$

      Welcome to your bootstrap project. This is a template file, because it has the `.tmpl` extension. A template file will have any tokens it references, replaced by values supplied by the user when this template is processed. __project__ and __description__  are tokens used in this file. They are defined in the `.variables.zat-prompt` file at the root of this project.

      ## Summary

      $description$
    "#;

  const PROJECT_CONFIG_CONF: &'static str =
    r#"
      //The $project__underscore$ token will be replace in this file's name when the template is processed. Note this is not a template file and as such any tokens defined within the file will not be replaced; Tokens in file and directory names will always get replaced irrespectively.
    "#;

  pub fn process_bootstrap(bootstrap_project_args: BootstrapProjectArgs) -> ZatAction {
    let repository_directory = RepositoryDir::new(&bootstrap_project_args.repository_dir);

    if repository_directory.does_exist() {
      Err(ZatError::bootstrap_repository_dir_should_not_exist(&bootstrap_project_args.repository_dir))
    } else {
      let repository_path = Path::new(repository_directory.path());
      Self::create_directory(repository_path)?;
      Self::create_file(repository_path.join(DOT_VARIABLES_PROMPT), Self::VARIABLE_FILE)?;

      let template_files_dir = TemplateFilesDir::from(&repository_directory);
      let template_files_dir_path = Path::new(template_files_dir.path());
      Self::create_directory(template_files_dir_path)?;
      Self::create_file(template_files_dir_path.join("README.md.tmpl"), Self::README_MD_TMPL)?;
      Self::create_file(template_files_dir_path.join("$project__underscore$_config.conf"), Self::PROJECT_CONFIG_CONF)?;

      Logger::info(&s!("Zat created a bootstrap repository at `{}`.", spath!(&repository_path)));
      Logger::info(&s!("Process the bootstrap repository with: `zat process --repository-dir {} --target-dir <YOUR_TARGET_DIRECTORY>`", spath!(&repository_path)));

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
