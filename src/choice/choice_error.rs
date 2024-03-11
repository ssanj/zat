use std::fmt::{self, Display};
use std::format as s;

#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceError {
  CouldNotReadInput(String),
  NotANumber(String),
  OutOfBounds(usize),
}

impl Display for ChoiceError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let prefix = match self {
      ChoiceError::CouldNotReadInput(input) => s!("Could not read input: {}", input),
      ChoiceError::NotANumber(input) => s!("Expected a number for choice, but got: {}", input),
      ChoiceError::OutOfBounds(number) => s!("Invalid index supplied: {}", number),
    };
    write!(f, "{}", prefix)
  }
}
