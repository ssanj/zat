use super::ConfigShellHookStatus;
use super::TargetDir;
use super::RepositoryDir;
use super::TemplateFilesDir;
use super::Filters;
use super::IgnoredFiles;
use crate::logging::Lines;
use std::{format as s};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserConfig {
  pub repository_dir: RepositoryDir,
  pub template_files_dir: TemplateFilesDir,
  pub target_dir: TargetDir,
  pub filters: Filters,
  pub ignores: IgnoredFiles,
  pub verbose: bool,
  pub shell_hook_status: ConfigShellHookStatus
}

impl Lines for UserConfig {
  fn lines(&self) -> Vec<String> {
    vec!
      [
        s!("Template directory: {}", self.repository_dir.path()),
        s!("Template files directory: {}", self.template_files_dir.path()),
        s!("Target directory: {}", self.target_dir.path),
        s!("Filters used: {}", self.filters),
        s!("Ignored files and folders: {}", self.ignores),
        s!("Verbose: {}", self.verbose),
        s!("Shell hook file: {}", match self.shell_hook_status {
            ConfigShellHookStatus::NoShellHook => "No shell hook found",
            ConfigShellHookStatus::RunShellHook(_) => "Shell hook found",
        })
      ]
  }
}


impl UserConfig {

  #[cfg(test)]
  pub fn new(source_dir: &str, destination_dir: &str) -> Self {
    let repository_dir = RepositoryDir::new(source_dir);
    let template_files_dir = TemplateFilesDir::from(&repository_dir);

    Self {
      repository_dir,
      template_files_dir,
      target_dir: TargetDir::new(destination_dir),
      filters: Default::default(),
      ignores: Default::default(),
      verbose: Default::default(),
      shell_hook_status: Default::default(),
    }
  }
}
