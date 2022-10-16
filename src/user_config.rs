use std::collections::HashMap;

use crate::models::{ZatResult, ZatError, ZatAction, SourceFile, TargetDir, TargetFile, TemplateDir};

#[derive(Debug, Clone)]
pub struct Ignores {
  files: Vec<String>,
  directories: Vec<String>,
}

pub struct Config {
  user_tokens: HashMap<String, String>,
  template_dir: TemplateDir,
  target_dir: TargetDir,
  ignores: Ignores
}

// Get user configuration
// Load token file (if any)
pub trait UserConfig {
  fn get_config() -> ZatResult<Config>;
}


// #[derive(Debug, Clone)]
// pub enum TemplateFileType {
//   File(SourceFile, TargetFile),
//   Dir(String)
// }

// #[derive(Debug, Clone)]
// pub enum Template {
//   File(SourceFile, TargetFile),
//   Dir(String)
// }


// pub trait TemplateConfigValidator {
//   fn validate(config: Config) -> ConfigState;
// }


// pub trait TemplateProcessor {
//   // fn process(&self, config: ValidConfig) -> ZatAction;
//   fn process(&self, config: ValidConfig, templates: Vec<TemplateFileType>) -> ZatResult<Vec<Template>>;
// }


// pub trait TemplateSelector {
//   fn select_templates(&self, config: ValidConfig) -> Vec<TemplateFileType>;
// }

// pub trait TemplateRender {
//   fn render(template: Template) -> ZatAction;
// }

// pub trait TokenReplacer {
//   fn replace_token(token: &str) -> String;
// }
