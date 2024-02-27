use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Filters {
  pub values: Vec<String>,
}


impl fmt::Display for Filters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.values.join(","))
    }
}
