use std::collections::HashMap;

use crate::variables::{TemplateVariable, VariableFilter, FilterType};

pub fn expand_filters(variables: Vec<TemplateVariable>, user_inputs: &HashMap<String, String>) -> HashMap<String, String> {
  let mut user_inputs_updated = user_inputs.clone();

  for v in variables {
    if let Some(variable_value) = user_inputs.get(&v.variable_name) {
      for filter in v.filters {
        let filter_name = filter.name;
        let filter_type = filter.filter;

        let updated_value = apply_filter(filter_type, &variable_value);
        let filter_key = format!("{}__{}", &v.variable_name, &filter_name);
        user_inputs_updated.insert(filter_key, updated_value);
      }
    }
  }
  user_inputs_updated
}

pub fn apply_filter(filter_type: FilterType, value: &str) -> String {
  match filter_type {
    Noop => value.to_owned(),
    _ => todo!()
  }
}

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn returns_empty_hash_if_no_matches() {
  let variables = vec![];
  let mut hash =  HashMap::new();
  let _ = hash.insert("something".to_owned(), "some value".to_owned());

  let result = expand_filters(variables, &hash);
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
            filter: FilterType::Noop
          }
        ]
     }
  ];

  let mut hash =  HashMap::new();
  let _ = hash.insert("project".to_owned(), "my cool project".to_owned());

  let result = expand_filters(variables, &hash);

  let expected_hash = HashMap::from(
    [
      ("project".to_owned(),  "my cool project".to_owned()),
      ("project__python".to_owned(),  "my cool project".to_owned())
    ]
  );
  assert_eq!(&result, &expected_hash)
}
