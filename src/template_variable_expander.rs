use std::collections::HashMap;
use crate::variables::{UserVariableValue, UserVariableKey, TemplateVariables};

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct ExpandedKey {
  pub value: String
}


impl ExpandedKey {
  pub fn new(value: String) -> Self {
    ExpandedKey {
      value
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpandedValue {
  pub value: String
}

impl ExpandedValue {
  pub fn new(value: String) -> Self {
    ExpandedValue {
      value
    }
  }
}


#[derive(Debug)]
pub struct ExpandedVariables {
  pub expanded_variables: HashMap<ExpandedKey, ExpandedValue>
}

impl ExpandedVariables {
  pub fn new(expanded_variables: HashMap<ExpandedKey, ExpandedValue>) -> Self {
    Self {
      expanded_variables
    }
  }
}


/// Expands the user-supplied key/values along with any specified filters.
///
/// # Arguments
///
///  * `variables` - Variables and filters defined in the variables file
///  * `user_inputs` - User-supplied values for variables defined in the variables file
///
/// ## Contract:
///  - If a filter is specified for a variable, the requested filter should be applied to the the user-supplied value
///  - The filtered values will have a key of the form: $variable_name_filtername$, unless it's a default key. See below.
///
///   where:
///    * `variable_name` - the name supplied to the variable by the user in the variables file  (`variable_name`)
///    * `filtername` - is the `name` of the filter in the variables file for this variable (`filters[*].name`)
///
/// - In case of a default filter (`filters[*].name = "__default__"`), the filtered value will have a key of the format: $variable_name$.
pub trait TemplateVariableExpander {
  // Take ownership of TemplateVariables and User Variables as they will be replaced by ExpandedVariables
  fn expand_filters(&self, variables: TemplateVariables, user_inputs: HashMap<UserVariableKey, UserVariableValue>) -> ExpandedVariables;
}
