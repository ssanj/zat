use std::{collections::HashSet, fmt::Display, fmt};

#[derive(Debug, Clone, PartialEq)]
pub struct IgnoredFiles {
  pub ignores: HashSet<String>,
}

impl IgnoredFiles {
  pub const DOT_GIT: &'static str  = ".git";
  pub const DEFAULT_IGNORES: [&'static str; 1] = [Self::DOT_GIT];

  pub fn default_ignores() -> Vec<String> {
    Self::DEFAULT_IGNORES
      .into_iter()
      .map(|v| v.to_owned())
      .collect()
  }
}

impl Display for IgnoredFiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.ignores.clone().into_iter().collect::<Vec<_>>().join(","))
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
