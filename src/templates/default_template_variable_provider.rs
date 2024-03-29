use crate::error::*;
use super::TemplateVariableProvider;
use crate::config::UserConfig;
use crate::config::VariableFile;
use super::{TemplateVariable, TemplateVariables};
use std::fs::File;
use std::io::Read;

pub struct DefaultTemplateVariableProvider;

impl DefaultTemplateVariableProvider {
  pub fn new() -> Self {
    Self
  }
}

// TODO: Should we move the file checks to a fake file system?
// That would make it easier to test and lead to more reuse of code
impl TemplateVariableProvider for DefaultTemplateVariableProvider {
  fn get_tokens(&self, user_config: UserConfig) -> ZatResult<TemplateVariables> {
    let variables_file: VariableFile = VariableFile::from(user_config.repository_dir);
    let variable_file_path = variables_file.get_path().to_owned();

    if variables_file.does_exist() {
      let mut f = File::open(variables_file).map_err(|e| ZatError::variable_file_cant_be_opened(&variable_file_path, e.to_string().as_str()))?;
      let mut variables_json = String::new();

      f.read_to_string(&mut variables_json).map_err(|e| ZatError::variable_file_cant_be_read(&variable_file_path, e.to_string().as_str()))?;

      let tokens: Vec<TemplateVariable> = serde_json::from_str(&variables_json).map_err(|e| ZatError::variable_file_cant_be_decoded(&variable_file_path, e.to_string().as_str()))?;

      if !tokens.is_empty() {
        Ok(
          TemplateVariables {
            tokens
          }
        )
      } else {
        Err(ZatError::variable_file_has_no_variables_defined(&variable_file_path))
      }
    } else {
      Err(ZatError::variable_file_does_not_exist(&variable_file_path))
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use tempfile::TempDir;
  use crate::config::DOT_VARIABLES_PROMPT;
  use crate::error::variable_file_error_reason::VariableFileErrorReason;
  use super::super::FilterType;
  use super::super::VariableFilter;
  use pretty_assertions::assert_eq;
  use std::io::Write;

  #[test]
  fn tokens_are_loaded_from_variable_file() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();
    let variable_file_path = template_dir.path().join(DOT_VARIABLES_PROMPT);

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
          "prompt": "Please enter your plugin description",
          "default_value": "Some plugin description"
        }
      ]
    "#;

    writeln!(&mut variable_file, "{}", variables_config).unwrap();

    let template_config_provider = DefaultTemplateVariableProvider::new();

    let user_config = UserConfig::new(&template_dir_path, &target_dir_path);

    let expected_tokens =
      TemplateVariables {
        tokens:
          vec![
            TemplateVariable::new(
              "project",
              "Name of project",
              "Please enter your project name",
              VariableFilter::from_pairs(
                &[
                  ("python", &FilterType::Snake),
                  ("Command", &FilterType::Pascal)
                ]
              ).as_ref(),
              None
            ),
            TemplateVariable::new (
              "plugin_description",
              "Explain what your plugin is about",
              "Please enter your plugin description",
              &[],
              Some("Some plugin description")
            ),
          ]
      };

    let tokens = template_config_provider.get_tokens(user_config).expect("Expected to get tokens");

    assert_eq!(tokens, expected_tokens);

    drop(variable_file);
  }

  #[test]
  fn fails_if_the_variable_file_cannot_be_decoded() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();
    let variable_file_path = template_dir.path().join(DOT_VARIABLES_PROMPT);

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

    let user_config = UserConfig::new(&template_dir_path, &target_dir_path);

    match template_config_provider.get_tokens(user_config) {
      Err(ZatError::ProcessCommandError(ProcessCommandErrorReason::VariableFileError(VariableFileErrorReason::VariableDecodeError(..)))) => (),
      Err(other_error) => panic!("Expected ZatError::VariableDecodeError but got different error : {}", other_error),
      Ok(value) => panic!("Expected ZatError::VariableDecodeError but got success with: {:?}", value)
    }

    drop(variable_file);
  }

  #[test]
  fn fails_if_the_variable_file_cannot_be_found() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();
    drop(target_dir);

    let template_config_provider = DefaultTemplateVariableProvider::new();

    let user_config = UserConfig::new(&template_dir_path, &target_dir_path);

    match template_config_provider.get_tokens(user_config) {
      Err(ZatError::ProcessCommandError(ProcessCommandErrorReason::VariableFileError(VariableFileErrorReason::VariableFileNotFound(..)))) => (),
      Err(other_error) => panic!("Expected ZatError::VariableFileNotFound but got different error : {}", other_error),
      Ok(value) => panic!("Expected ZatError::VariableFileNotFound but got success with: {:?}", value)
    }
  }

  #[test]
  fn fails_if_the_variable_file_has_no_variables_defined() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();
    let variable_file_path = template_dir.path().join(DOT_VARIABLES_PROMPT);

    let mut variable_file = File::create(variable_file_path).unwrap();

    drop(target_dir);

    let variables_config = r#"
      [
      ]
    "#;

    writeln!(&mut variable_file, "{}", variables_config).unwrap();

    let template_config_provider = DefaultTemplateVariableProvider::new();

    let user_config = UserConfig::new(&template_dir_path, &target_dir_path);

    match template_config_provider.get_tokens(user_config) {
      Err(ZatError::ProcessCommandError(ProcessCommandErrorReason::VariableFileError(VariableFileErrorReason::VariableFileHasNoVariableDefinitions(..)))) => (),
      Err(other_error) => panic!("Expected ZatError::VariableFileHasNoVariableDefinitions but got different error : {}", other_error),
      Ok(value) => panic!("Expected ZatError::VariableFileHasNoVariableDefinitions but got success with: {:?}", value)
    }

    drop(variable_file);
  }

}
