use super::TargetDir;
use super::TemplateDir;
use super::Filters;
use super::IgnoredFiles;


#[derive(Debug, Clone, PartialEq)]
pub struct UserConfig {
  pub template_dir: TemplateDir,
  pub target_dir: TargetDir,
  pub filters: Filters,
  pub ignores: IgnoredFiles
}

impl UserConfig {
  pub fn new(source_dir: &str, destination_dir: &str) -> Self {
    Self {
      template_dir: TemplateDir::new(source_dir),
      target_dir: TargetDir::new(destination_dir),
      filters: Default::default(),
      ignores: Default::default()

    }
  }
}
