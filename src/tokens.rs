use std::collections::HashMap;

use crate::variables::{TemplateVariable, VariableFilter, FilterType};
use crate::models::ZatError;
use convert_case::{Case, Casing};
use std::io::{stdin, BufRead};
use std::path::Path;

use std::fs::{self, File};
use std::io::Read;

const DEFAULT_FILTER: &str = "__default__";


pub enum UserSelection {
  Exit,
  Continue(HashMap<String, String>)
}

pub enum VariablesCorrect {
  Yes,
  No,
  Exit
}


pub fn load_variables(variables_file: &Path) -> Result<UserSelection, ZatError> {
   if variables_file.exists() {
      println!("Loading variables file");
      let mut f = File::open(variables_file).map_err(|e| ZatError::IOError(e.to_string()))?;
      let mut variables_json = String::new();

      f.read_to_string(&mut variables_json).map_err(|e| ZatError::IOError(e.to_string()))?;

      let variables: Vec<TemplateVariable> = serde_json::from_str(&variables_json).map_err(|e| ZatError::SerdeError(e.to_string()))?;
      println!("loaded: {:?}", &variables);

      let user_selection = handle_user_input_and_selection(&variables);
      match user_selection {
        UserSelection::Continue(token_map) => {
          let updated_token_map = expand_filters(&variables, &token_map);

          println!("updated tokens: {:?}", updated_token_map);

          let updated_token_map_dollar_keys: HashMap<_, _> =
            updated_token_map
              .into_iter()
              .map(|(k, v)| (format!("${}$", k), v))
              .collect();

          println!("updated tokens dollar keys: {:?}", &updated_token_map_dollar_keys);
          Ok(UserSelection::Continue(updated_token_map_dollar_keys))
        },
        UserSelection::Exit => Ok(UserSelection::Exit),
      }
    } else {
      println!("No variables file");
      Ok(UserSelection::Continue(HashMap::new()))
    }
}

fn check_user_input() -> VariablesCorrect {
  // Check if variables are ok
  println!("Please confirm that the variable mappings are correct. Press [y]es, [n]o or e[x]it.");
  let mut user_response = String::new();
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
  handle.read_line(&mut user_response).expect("Could not read from stdin"); // Unexpected, so throw
  let line = user_response.lines().next().expect("Could not extract line from buffer"); // Unexpected, so throw

  match &line[..] {
    "y" => VariablesCorrect::Yes,
    "x" => VariablesCorrect::Exit,
    "n" => VariablesCorrect::No,
    _ => {
      println!("Invalid response :( Let's try that again.");
      println!("");
      check_user_input()
    }
  }
}

fn handle_user_input_and_selection(variables: &[TemplateVariable]) -> UserSelection {
  let token_map = get_user_input(&variables);
  print_user_input(&token_map);

  match check_user_input() {
    VariablesCorrect::No => handle_user_input_and_selection(variables),
    VariablesCorrect:: Yes =>  UserSelection::Continue(token_map),
    VariablesCorrect:: Exit => UserSelection::Exit
  }
}

fn print_user_input(token_map: &HashMap<String, String>) {
    println!("\nSupplied Values\n---------------\n");

    for t in token_map.iter() {
      println!("{} -> {}", &t.0, &t.1)
    }
}

fn get_user_input(variables: &[TemplateVariable]) -> HashMap<String, String> {
  let stdin = std::io::stdin();
  let mut token_map = HashMap::new();

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

  token_map
}

pub fn expand_filters(variables: &Vec<TemplateVariable>, user_inputs: &HashMap<String, String>) -> HashMap<String, String> {
  let mut user_inputs_updated = user_inputs.clone();

  for v in variables {
    if let Some(variable_value) = user_inputs.get(&v.variable_name) {
      for filter in &v.filters {
        let filter_name = &filter.name;
        let filter_type = &filter.filter;

        let updated_value = apply_filter(filter_type, &variable_value);

        let filter_key =
          if filter_name == DEFAULT_FILTER { /* Default filter to apply to variable value */
            v.variable_name.clone()
          } else {
            format!("{}__{}", &v.variable_name, &filter_name)
          };

        let _ = user_inputs_updated.insert(filter_key, updated_value);
      }
    }
  }
  user_inputs_updated
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

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn returns_empty_hash_if_no_matches() {
  let variables = vec![];
  let mut hash =  HashMap::new();
  let _ = hash.insert("something".to_owned(), "some value".to_owned());

  let result = expand_filters(&variables, &hash);
  assert_eq!(&result, &hash)
}

#[test]
fn returns_input_hash_if_no_filters() {
  let variables = vec![
    TemplateVariable {
        variable_name: "plugin_description".to_owned(),
        description: "Explain what your plugin is about".to_owned(),
        prompt: "Please enter your plugin description".to_owned(),
        filters: vec![]
     }
  ];
}

#[test]
fn returns_updated_input_hash_if_has_filters() {
  let variables = vec![
    TemplateVariable {
        variable_name: "project".to_owned(),
        description: "Explain what your project is about".to_owned(),
        prompt: "Please enter your project name".to_owned(),
        filters: vec![
          VariableFilter {
            name: "python".to_owned(),
            filter: FilterType::Snake
          },
          VariableFilter {
            name: "command".to_owned(),
            filter: FilterType::Pascal
          },
          VariableFilter {
            name: "heading".to_owned(),
            filter: FilterType::Title
          },
        ]
     }
  ];

  let mut hash =  HashMap::new();
  let _ = hash.insert("project".to_owned(), "my cool project".to_owned());

  let result = expand_filters(&variables, &hash);

  let expected_hash = HashMap::from(
    [
      ("project".to_owned(),  "my cool project".to_owned()),
      ("project__python".to_owned(),  "my_cool_project".to_owned()),
      ("project__command".to_owned(),  "MyCoolProject".to_owned()),
      ("project__heading".to_owned(),  "My Cool Project".to_owned()),
    ]
  );
  assert_eq!(&result, &expected_hash)
}

#[test]
fn returns_updated_input_hash_if_has_filters_with_default() {
  let variables = vec![
    TemplateVariable {
        variable_name: "project".to_owned(),
        description: "Explain what your project is about".to_owned(),
        prompt: "Please enter your project name".to_owned(),
        filters: vec![
          VariableFilter {
            name: "python".to_owned(),
            filter: FilterType::Snake
          },
          VariableFilter {
            name: DEFAULT_FILTER.to_owned(),
            filter: FilterType::Pascal
          },
          VariableFilter {
            name: "command".to_owned(),
            filter: FilterType::Pascal
          },
          VariableFilter {
            name: "heading".to_owned(),
            filter: FilterType::Title
          },
        ]
     }
  ];

  let mut hash =  HashMap::new();
  let _ = hash.insert("project".to_owned(), "my cool project".to_owned());

  let result = expand_filters(&variables, &hash);

  let expected_hash = HashMap::from(
    [
      ("project".to_owned(),  "MyCoolProject".to_owned()),
      ("project__python".to_owned(),  "my_cool_project".to_owned()),
      ("project__command".to_owned(),  "MyCoolProject".to_owned()),
      ("project__heading".to_owned(),  "My Cool Project".to_owned()),
    ]
  );
  assert_eq!(&result, &expected_hash)
}
