use std::{collections::HashMap, path::Path};
use std::fs::{self, File};
use std::io::Read;
use std::io::{stdin, BufRead};

use crate::behaviours::*;
use crate::variables::TemplateVariable;

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

// TODO: We want to write the main logic once for both Prod and Test
// and then swap inner implementations to exercise the solution
impl Prod {
  fn print_user_input(user_input: &UserVariableInputs) {
      let token_map = &user_input.0;
      println!("\nSupplied Values\n---------------\n");

      for t in token_map.iter() {
        println!("{} -> {}", &t.0, &t.1)
      }
  }
}
