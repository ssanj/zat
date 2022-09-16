use std::{collections::HashMap, path::Path};
use std::fs::{self, File};
use std::io::Read;
use std::io::{stdin, BufRead};
use convert_case::{Case, Casing};

use crate::behaviours::*;
use crate::variables::{TemplateVariable, VariableFilter, FilterType};

struct Prod;

impl VariableSupplier for Prod {
  fn load_variables_from_file(variables_file: &Path) -> VariableSupplierResult<Vec<TemplateVariable>> {
    if variables_file.exists() {
      println!("Loading variables file");
      let mut f = File::open(variables_file).map_err(|e| VariableSupplierError::VariableFileCantBeOpened(e.to_string()))?;
      let mut variables_json = String::new();

      f.read_to_string(&mut variables_json).map_err(|e| VariableSupplierError::CouldNotReadVariableFile(e.to_string()))?;

      let variables: Vec<TemplateVariable> = serde_json::from_str(&variables_json).map_err(|e| VariableSupplierError::CouldNotDecodeVariableFile(e.to_string()))?;

      Ok(variables)
    } else {
      Err(VariableSupplierError::VariableFileNotFound(format!("Could not find variable file: {}", variables_file.display())))
    }
  }
}

impl VariableInputs for Prod {
  fn get_variable_inputs(variables: &[TemplateVariable]) -> UserVariableInputs {
    let stdin = std::io::stdin();
    let mut token_map = HashMap::new();
    println!("");

    for v in variables {
      println!("{}. {}", v.description, v.prompt);
      let mut variable_value = String::new();
      if let Ok(read_count) = stdin.read_line(&mut variable_value) {
        if read_count > 0 { //read at least one character
          let _ = variable_value.pop(); // remove newline
          if !variable_value.is_empty() {
            token_map.insert(v.variable_name.clone(), variable_value);
          }
        }
      }
    }

    UserVariableInputs(token_map)
  }
}

impl VariableValidator for Prod {
  fn validate_variables(user_inputs: &UserVariableInputs) -> VariableValidity {
    Prod::print_user_input(user_inputs);

    println!("Please confirm that the variable mappings are correct: y/n. Press any other key to exit");
    let mut user_response = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut user_response).expect("Could not read from stdin"); // Unexpected, so throw
    let line = user_response.lines().next().expect("Could not extract line from buffer"); // Unexpected, so throw

    match &line[..] {
      "y" => VariableValidity::VariablesValid,
      "n" => VariableValidity::VariablesInvalidRetry,
      _ => VariableValidity::VariablesInvalidStop
    }
  }
}

impl ExpandVariableFilters for Prod {
  fn expand_filters(variables: &[TemplateVariable], user_inputs: &ValidatedUserVariableInputs) -> ValidatedUserVariableInputsFiltersExpanded {
    let mut user_inputs_updated = user_inputs.0.clone();
    let token_map = &user_inputs.0;

    for v in variables {
      if let Some(variable_value) = token_map.get(&v.variable_name) {
        for filter in &v.filters {
          let filter_name = &filter.name;
          let filter_type = &filter.filter;

          let updated_value = Prod::apply_filter(filter_type, &variable_value);

          let filter_key =
            if filter_name == Prod::DEFAULT_FILTER { /* Default filter to apply to variable value */
              v.variable_name.clone()
            } else {
              format!("{}__{}", &v.variable_name, &filter_name)
            };

          let _ = user_inputs_updated.insert(filter_key, updated_value);
        }
      }
    }
    ValidatedUserVariableInputsFiltersExpanded(user_inputs_updated)
  }
}

impl AddTokensToVariables for Prod {
  fn add_tokens_delimiters(user_input: &ValidatedUserVariableInputsFiltersExpanded) -> ValidatedUserVariableInputsFiltersExpandedWithTokens {
    ValidatedUserVariableInputsFiltersExpandedWithTokens(
      user_input
        .0
        .clone()
        .into_iter()
        .map(|(k, v)| (format!("${}$", k), v))
        .collect()
    )
  }
}
// TODO: We want to write the main logic once for both Prod and Test
// and then swap inner implementations to exercise the solution
impl Prod {

  const DEFAULT_FILTER: &'static str = "__default__";

  fn print_user_input(user_input: &UserVariableInputs) {
      let token_map = &user_input.0;
      println!("\nSupplied Values\n---------------\n");

      for t in token_map.iter() {
        println!("{} -> {}", &t.0, &t.1)
      }
  }

  // See: https://docs.rs/convert_case/latest/convert_case/enum.Case.html
  fn apply_filter(filter_type: &FilterType, value: &str) -> String {
    match filter_type {
      FilterType::Camel  => value.to_case(Case::Camel),  /* "My variable NAME" -> "myVariableName"   */
      FilterType::Cobol  => value.to_case(Case::Cobol),  /* "My variable NAME" -> "MY-VARIABLE-NAME" */
      FilterType::Flat   => value.to_case(Case::Flat),   /* "My variable NAME" -> "myvariablename"   */
      FilterType::Kebab  => value.to_case(Case::Kebab),  /* "My variable NAME" -> "my-variable-name" */
      FilterType::Lower  => value.to_case(Case::Lower),  /* "My variable NAME" -> "my variable name" */
      FilterType::Noop   => value.to_owned(),            /* "My variable NAME" -> "My variable NAME" */
      FilterType::Pascal => value.to_case(Case::Pascal), /* "My variable NAME" -> "MyVariableName"   */
      FilterType::Snake  => value.to_case(Case::Snake),  /* "My variable NAME" -> "my_variable_name" */
      FilterType::Title  => value.to_case(Case::Title),  /* "My variable NAME" -> "My Variable Name" */
      FilterType::Upper  => value.to_case(Case::Upper),  /* "My variable NAME" -> "MY VARIABLE NAME" */
    }
  }
}
