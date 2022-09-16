use std::path::Path;

use crate::models::*;
use crate::tokens::UserSelection;
use crate::variables::*;
use crate::behaviours::*;

enum UserVariableDecision {
  Exit,
  Continue(ValidatedUserVariableInputs)
}

pub struct VariableExtractor<T> {
  pub value: T
}

impl <T> VariableExtractor<T> where
  T: VariableSupplier + VariableInputs + VariableValidator + ExpandVariableFilters + AddTokensToVariables
{
  pub fn extract_variables(&self, variables_file: &Path) -> ZatResult2<VariableValidationResponse> {
    match T::load_variables_from_file(variables_file) {
      Ok(template_vars) => {
        match VariableExtractor::<T>::get_user_input(&template_vars) {
          UserVariableDecision::Continue(user_inputs) => {
            let expanded_variables = T::expand_filters(&template_vars, &user_inputs);
            let expanded_with_tokens = T::add_tokens_delimiters(&expanded_variables);
            Ok(VariableValidationResponse::Continue(expanded_with_tokens))
          },
          UserVariableDecision::Exit => Ok(VariableValidationResponse::UserQuit)
        }
      },
      Err(e) => Err(ZatError2::VariableExtractionError(e.to_string()))
    }
  }

  fn get_user_input(template_vars: &[TemplateVariable]) -> UserVariableDecision {
      let user_inputs = T::get_variable_inputs(template_vars);
      match T::validate_variables(&user_inputs) {
          VariableValidity::VariablesValid => { //We need to return the validated inputs from here
            UserVariableDecision::Continue(ValidatedUserVariableInputs(user_inputs.0))
          },
          VariableValidity::VariablesInvalidStop => UserVariableDecision::Exit,
          VariableValidity::VariablesInvalidRetry => VariableExtractor::<T>::get_user_input(template_vars)
      }
  }
}
