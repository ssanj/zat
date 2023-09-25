use super::target_directory::TargetDir;
use super::template_directory::TemplateDir;
use super::filters::Filters;
use super::ignored_files::IgnoredFiles;


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
