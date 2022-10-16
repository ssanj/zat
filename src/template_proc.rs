use crate::models::ZatResult;
use crate::template_config_validator::ValidConfig;
use crate::template_selector::TemplateFileType;
use crate::models::{SourceFile, TargetFile};

// TODO: This seems to be a copy of TemplateFileType. Fix
#[derive(Debug, Clone)]
pub enum Template {
  File(SourceFile, TargetFile),
  Dir(String)
}


pub trait TemplateProcessor {
  // fn process(&self, config: ValidConfig) -> ZatAction;
  fn process(&self, config: ValidConfig, templates: Vec<TemplateFileType>) -> ZatResult<Vec<Template>>;
}
