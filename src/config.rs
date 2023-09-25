use std::collections::HashSet;
use std::path::Path;
use crate::args::target_directory::TargetDir;
use crate::args::template_directory::TemplateDir;


#[derive(Debug, Clone, PartialEq)]
pub struct IgnoredFiles {
  pub ignores: HashSet<String>,
}

impl IgnoredFiles {
  pub const DOT_VARIABLES_DOT_PROMPT: &'static str  = ".variables.prompt";
  pub const DOT_GIT: &'static str  = ".git";
  pub const DEFAULT_IGNORES: [&str; 2] = [Self::DOT_VARIABLES_DOT_PROMPT, Self::DOT_GIT];

  pub fn default_ignores() -> Vec<String> {
    Self::DEFAULT_IGNORES
      .into_iter()
      .map(|v| v.to_owned())
      .collect()
  }
}

impl Default for IgnoredFiles {
  fn default() -> Self {
    IgnoredFiles {
      ignores:
        HashSet::from_iter(Self::default_ignores()),
    }
  }
}

impl <F> From<F> for IgnoredFiles
  where F: Iterator<Item = String>
{
  fn from(values: F) -> Self {
    IgnoredFiles {
      ignores: HashSet::from_iter(values)
    }
  }
}


#[derive(Debug, Clone)]
pub struct VariableFile {
  path: String
}

impl VariableFile {

  pub const PATH: &'static str  = ".variables.prompt";

  pub fn does_exist(&self) -> bool {
    Path::new(&self.path).exists()
  }
}

impl From<TemplateDir> for VariableFile {
  fn from(template_dir: TemplateDir) -> Self {
      let variables_file = template_dir.join(VariableFile::PATH);
      VariableFile {
        path: variables_file.display().to_string()
      }
  }
}

impl AsRef<Path> for VariableFile {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.path)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Filters {
  pub values: Vec<String>,
}


impl Default for Filters {
    fn default() -> Self {
        Self {
          values: vec![]
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserConfigX {
  pub template_dir: TemplateDir,
  pub target_dir: TargetDir,
  pub filters: Filters,
  pub ignores: IgnoredFiles
}

impl UserConfigX {
  pub fn new(source_dir: &str, destination_dir: &str) -> Self {
    Self {
      template_dir: TemplateDir::new(source_dir),
      target_dir: TargetDir::new(destination_dir),
      filters: Default::default(),
      ignores: Default::default()

    }
  }
}
