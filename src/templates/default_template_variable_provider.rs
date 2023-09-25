use crate::shared_models::*;
use crate::templates::template_variable_provider::TemplateVariableProvider;
use crate::config::{UserConfig, VariableFile};
use super::variables::{TemplateVariable, TemplateVariables};
use std::fs::File;
use std::io::{Read, Write};

pub struct DefaultTemplateVariableProvider;

impl DefaultTemplateVariableProvider {
  pub fn new() -> Self {
    Self
  }
}

// TODO: Should we move the file checks to a fake file system?
// That would make it easier to test and lead to more reuse of code
impl TemplateVariableProvider for DefaultTemplateVariableProvider {
  fn get_tokens(&self, user_config: UserConfig) -> ZatResultX<TemplateVariables> {
    let variables_file: VariableFile = VariableFile::from(user_config.template_dir);

    let tokens: Vec<TemplateVariable> =
      if variables_file.does_exist() {
        let mut f = File::open(variables_file).map_err(|e| ZatErrorX::VariableOpenError(e.to_string()))?;
        let mut variables_json = String::new();

        f.read_to_string(&mut variables_json).map_err(|e| ZatErrorX::VariableReadError(e.to_string()))?;

        serde_json::from_str(&variables_json).map_err(|e| ZatErrorX::VariableDecodeError(e.to_string()))?
      } else {
        vec![]
      };

    Ok(
      TemplateVariables {
        tokens
      }
    )
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use tempfile::TempDir;
  use crate::config::{Filters, IgnoredFiles};
  use crate::args::template_directory::TemplateDir;
  use crate::args::target_directory::TargetDir;

  #[test]
  fn tokens_are_empty_if_variable_file_does_not_exist() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    drop(target_dir);

    let template_token_provider = DefaultTemplateVariableProvider::new();

    let user_config = UserConfig {
      template_dir: TemplateDir::new(&template_dir_path),
      target_dir: TargetDir::new(&target_dir_path),
      filters: Filters::default(),
      ignores: IgnoredFiles::default()
    };

    let tokens = template_token_provider.get_tokens(user_config).expect("Expected to get tokens");
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

    let template_config_provider = DefaultTemplateVariableProvider::new();

    let user_config = UserConfig {
      template_dir: TemplateDir::new(&template_dir_path),
      target_dir: TargetDir::new(&target_dir_path),
      filters: Filters::default(),
      ignores: IgnoredFiles::default()
    };

    let tokens = template_config_provider.get_tokens(user_config).expect("Expected to get tokens");
    assert_eq!(tokens.tokens.len(), 2);

    drop(variable_file);
  }

  #[test]
  fn fails_if_the_variable_file_cannot_be_decoded() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();
    let variable_file_path = template_dir.path().join(VariableFile::PATH);

    let mut variable_file = File::create(variable_file_path).unwrap();

    drop(target_dir);

    // invalid JSON
    let variables_config = r#"
      [
        {
          "variable_name": "project",
          "
      ]
    "#;

    writeln!(&mut variable_file, "{}", variables_config).unwrap();

    let template_config_provider = DefaultTemplateVariableProvider::new();

    let user_config = UserConfig {
      template_dir: TemplateDir::new(&template_dir_path),
      target_dir: TargetDir::new(&target_dir_path),
      filters: Filters::default(),
      ignores: IgnoredFiles::default()
    };

    match template_config_provider.get_tokens(user_config) {
      Err(ZatErrorX::VariableDecodeError(_)) => assert!(true),
      Err(other_error) => assert!(false, "Expected ZatErrorX::VariableDecodeError but got different error : {}", other_error.to_string()),
      Ok(value) => assert!(false, "Expected ZatErrorX::VariableDecodeError but got success with: {:?}", value)
    }

    drop(variable_file);
  }

}
