use crate::template_config_validator::ValidConfig;
use crate::models::{SourceFile, TargetFile};


#[derive(Debug, Clone)]
pub enum TemplateFileType {
  File(SourceFile, TargetFile),
  Dir(String)
}

pub trait TemplateSelector {
  fn select_templates(&self, config: ValidConfig) -> Vec<TemplateFileType>;
}
