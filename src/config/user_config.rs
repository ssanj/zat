use super::TargetDir;
use super::TemplateDir;
use super::TemplateFilesDir;
use super::Filters;
use super::IgnoredFiles;


#[derive(Debug, Clone, PartialEq)]
pub struct UserConfig {
  pub template_dir: TemplateDir,
  pub template_files_dir: TemplateFilesDir,
  pub target_dir: TargetDir,
  pub filters: Filters,
  pub ignores: IgnoredFiles
}

impl UserConfig {
  pub fn new(source_dir: &str, destination_dir: &str) -> Self {
    let template_dir = TemplateDir::new(source_dir);
    let template_files_dir = template_dir.template_files_path();

    Self {
      template_dir,
      template_files_dir,
      target_dir: TargetDir::new(destination_dir),
      filters: Default::default(),
      ignores: Default::default()
    }
  }
}
