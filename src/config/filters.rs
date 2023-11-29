use std::fmt;

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

impl fmt::Display for Filters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.values.join(","))
    }
}
