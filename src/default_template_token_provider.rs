use crate::shared_models::*;
use crate::template_token_provider::{TemplateTokenProvider, TemplateTokens};
use crate::user_config_provider::*;
use crate::models::{TargetDir, TemplateDir};
use crate::variables::TemplateVariable;
use std::fs::File;
use std::io::{Read, Write};

struct DefaultTemplateTokenProvider;

impl DefaultTemplateTokenProvider {
  pub fn new() -> Self {
    Self
  }
}

// TODO: Should we move the file checks to a fake file system?
// That would make it easier to test and lead to more reuse of code
impl TemplateTokenProvider for DefaultTemplateTokenProvider {
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

#[cfg(test)]
mod tests {

  use crate::template_token_provider;

  use super::*;
  use tempfile::TempDir;

  #[test]
  fn tokens_are_empty_if_variable_file_does_not_exist() {
    let target_dir = TempDir::new().unwrap();
    let template_dir = TempDir::new().unwrap();

    let template_dir_path = template_dir.path().display().to_string();
    let target_dir_path = target_dir.path().display().to_string();

    drop(target_dir);

    let template_token_provider = DefaultTemplateTokenProvider::new();

    let user_config = UserConfig {
      template_dir: TemplateDir::new(&template_dir_path),
      target_dir: TargetDir::new(&target_dir_path),
      ignores: Ignores::default()
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

    let template_config_provider = DefaultTemplateTokenProvider::new();

    let user_config = UserConfig {
      template_dir: TemplateDir::new(&template_dir_path),
      target_dir: TargetDir::new(&target_dir_path),
      ignores: Ignores::default()
    };

    let tokens = template_config_provider.get_tokens(user_config).expect("Expected to get tokens");
    assert_eq!(tokens.tokens.len(), 2);

    drop(variable_file);
  }


}
