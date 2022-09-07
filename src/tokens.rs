use std::collections::HashMap;

use crate::variables::{TemplateVariable, VariableFilter, FilterType};
use convert_case::{Case, Casing};

const DEFAULT_FILTER: &str = "__default__";

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
