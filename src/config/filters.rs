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

