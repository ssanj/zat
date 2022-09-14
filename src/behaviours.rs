use std::path::Path;
use std::collections::HashMap;
use crate::models::*;
use crate::variables::TemplateVariable;

pub enum VariableSupplierError {
  VariableFileNotFound(String),
  VariableFileCantBeOpened(String),
  CouldNotReadVariableFile(String),
  CouldNotDecodeVariableFile(String),
}

pub type VariableSupplierResult<T> = Result<T, VariableSupplierError>;

pub trait VariableSupplier {
  fn load_variables_from_file(variables_file: &Path) -> VariableSupplierResult<Vec<TemplateVariable>>;
}

pub enum VariableValidity {
  VariablesValid,
  VariablesInvalidStop,
  VariablesInvalidRetry
}

pub struct UserVariableInputs(pub HashMap<String, String>);
pub struct ValidatedUserVariableInputs(pub HashMap<String, String>);

pub trait VariableInputs {
  fn get_variable_inputs(variables: &[TemplateVariable]) -> UserVariableInputs;
}

pub trait VariableValidator {
  fn validate_variables(token_map: &UserVariableInputs) -> VariableValidity;
}

pub trait ExpandVariableFilters {
  fn expand_filters(variables: &[TemplateVariable], user_inputs: &UserVariableInputs) -> ValidatedUserVariableInputs;
}
