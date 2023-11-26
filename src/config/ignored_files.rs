use std::collections::HashSet;
use super::DOT_VARIABLES_PROMPT;

#[derive(Debug, Clone, PartialEq)]
pub struct IgnoredFiles {
  pub ignores: HashSet<String>,
}

impl IgnoredFiles {
  pub const DOT_GIT: &'static str  = ".git";
  pub const DEFAULT_IGNORES: [&'static str; 2] = [DOT_VARIABLES_PROMPT, Self::DOT_GIT];

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
