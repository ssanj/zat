use std::path::Path;
use std::collections::HashMap;
use crate::models::*;
use crate::variables::TemplateVariable;
use std::fmt;

#[derive(Debug, Clone)]
pub enum VariableSupplierError {
  VariableFileNotFound(String),
  VariableFileCantBeOpened(String),
  CouldNotReadVariableFile(String),
  CouldNotDecodeVariableFile(String),
}

impl fmt::Display for VariableSupplierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let message = match self {
        VariableSupplierError::VariableFileNotFound(e) => format!("Variable file could not be found: {}", e),
        VariableSupplierError::VariableFileCantBeOpened(e) => format!("Variable file could not be opened: {}", e),
        VariableSupplierError::CouldNotReadVariableFile(e) => format!("Variable file could not be read: {}", e),
        VariableSupplierError::CouldNotDecodeVariableFile(e) => format!("Variable file could not be decoded as JSON: {}", e)
      };

      write!(f, "{}", message)
    }
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


// TODO: Wrap the inner types
// HashMap<VariableName, Tokenised(FiltersApplied(ValidateInput(String)))>
pub struct UserVariableInputs(pub HashMap<String, String>);
pub struct ValidatedUserVariableInputs(pub HashMap<String, String>);
pub struct ValidatedUserVariableInputsFiltersExpanded(pub HashMap<String, String>);
pub struct ValidatedUserVariableInputsFiltersExpandedWithTokens(pub HashMap<String, String>);


pub enum VariableValidationResponse {
  Continue(ValidatedUserVariableInputsFiltersExpandedWithTokens),
  UserQuit
}


pub trait VariableInputs {
  fn get_variable_inputs(variables: &[TemplateVariable]) -> UserVariableInputs;
}

pub trait VariableValidator {
  fn validate_variables(token_map: &UserVariableInputs) -> VariableValidity;
}

pub trait ExpandVariableFilters {
  fn expand_filters(variables: &[TemplateVariable], user_inputs: &ValidatedUserVariableInputs) -> ValidatedUserVariableInputsFiltersExpanded;
}


pub trait AddTokensToVariables {
  fn add_tokens_delimiters(user_input: &ValidatedUserVariableInputsFiltersExpanded) -> ValidatedUserVariableInputsFiltersExpandedWithTokens;
}
