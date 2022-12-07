use crate::template_variable_expander::{ExpandedVariables, TemplateVariableExpander, ExpandedKey, ExpandedValue, DEFAULT_FILTER};
use crate::variables::{UserVariableKey, UserVariableValue, FilterType};
use std::collections::HashMap;
use crate::filter_applicator::FilterApplicator;


// struct ConvertCaseFilteration;

// impl FilterApplicator for ConvertCaseFilteration {

// }

// impl  ConvertCaseFilteration {
//   pub fn expand_filters(variables: &Vec<TemplateVariable>, user_inputs: &HashMap<String, String>) -> HashMap<String, String> {
//     let mut user_inputs_updated = user_inputs.clone();

//     for v in variables {
//       if let Some(variable_value) = user_inputs.get(&v.variable_name) {
//         for filter in &v.filters {
//           let filter_name = &filter.name;
//           let filter_type = &filter.filter;

//           let updated_value = apply_filter(filter_type, &variable_value);

//           let filter_key =
//             if filter_name == DEFAULT_FILTER { /* Default filter to apply to variable value */
//               v.variable_name.clone()
//             } else {
//               format!("{}__{}", &v.variable_name, &filter_name)
//             };

//           let _ = user_inputs_updated.insert(filter_key, updated_value);
//         }
//       }
//     }
//     user_inputs_updated
//   }

//   // See: https://docs.rs/convert_case/latest/convert_case/enum.Case.html
//   fn apply_filter(filter_type: &FilterType, value: &str) -> String {
//     match filter_type {
//       FilterType::Camel  => value.to_case(Case::Camel),  /* "My variable NAME" -> "myVariableName"   */
//       FilterType::Cobol  => value.to_case(Case::Cobol),  /* "My variable NAME" -> "MY-VARIABLE-NAME" */
//       FilterType::Flat   => value.to_case(Case::Flat),   /* "My variable NAME" -> "myvariablename"   */
//       FilterType::Kebab  => value.to_case(Case::Kebab),  /* "My variable NAME" -> "my-variable-name" */
//       FilterType::Lower  => value.to_case(Case::Lower),  /* "My variable NAME" -> "my variable name" */
//       FilterType::Noop   => value.to_owned(),            /* "My variable NAME" -> "My variable NAME" */
//       FilterType::Pascal => value.to_case(Case::Pascal), /* "My variable NAME" -> "MyVariableName"   */
//       FilterType::Snake  => value.to_case(Case::Snake),  /* "My variable NAME" -> "my_variable_name" */
//       FilterType::Title  => value.to_case(Case::Title),  /* "My variable NAME" -> "My Variable Name" */
//       FilterType::Upper  => value.to_case(Case::Upper),  /* "My variable NAME" -> "MY VARIABLE NAME" */
//     }
//   }
// }

struct DefaultTemplateVariableExpander {
  filter_applicator: Box<dyn FilterApplicator>
}

// Does it make sense to have this default filter_applicator?
// We have to supply a filter_applicator, so it would make sense to supply it on construction
// Assuming FilterNameFilterApplicator by default is not what we expect as a "default"
impl DefaultTemplateVariableExpander {
  // pub fn new() -> Self {
  //   Self {
  //     filter_applicator: Box::new(FilterNameFilterApplicator)
  //   }
  // }

  pub fn with_filter_applicator(filter_applicator: Box<dyn FilterApplicator>) -> Self {
    Self {
      filter_applicator
    }
  }
}


impl TemplateVariableExpander for DefaultTemplateVariableExpander {
    fn expand_filters(&self, variables: crate::variables::TemplateVariables, user_inputs: HashMap<UserVariableKey, UserVariableValue>) -> ExpandedVariables {

      let expanded_variables: HashMap<ExpandedKey, ExpandedValue> =
        variables
          .tokens
          .iter()
          .filter_map(|t|{
            let key = t.variable_name.to_owned();
            user_inputs
              .get(&UserVariableKey::new(key.clone()))
              .map(|v|{
                (t, ExpandedKey::new(key), ExpandedValue::new(v.value.to_owned()))
              })
          })
          .flat_map(|(t, k, v)|{
              let o_key = k.clone();
              let o_value = v.clone();

              let filtered_values =
                t
                  .filters
                  .iter()
                  .map(move |f|{
                    let filtered_value = self.filter_applicator.apply_filter(&f.filter, &v.value);
                    let filter_name =
                      if &f.name == DEFAULT_FILTER {
                        k.value.clone() //if the key name is __default__ then use the original key name
                      } else {
                        format!("{}_{}", k.value.clone(), f.name)
                      };

                    (ExpandedKey::new(filter_name.to_owned()), ExpandedValue::new(filtered_value))
                  });

                // Chain with original values, so they can get overwritten if need by by __default__
                // It's important to have the original values first in the chain otherwise they
                // will overwrite the filtered values.
                vec![(o_key, o_value)]
                  .into_iter()
                  .chain(filtered_values)

          })
          .collect();

        // for each filter defined:
        // 1. apply filter
        // 2. insert filtered value under filtered name (except with __default__)

      ExpandedVariables::new(expanded_variables)
    }
}


#[cfg(test)]
mod tests {

  use super::*;
  use crate::{variables::TemplateVariables, default_template_variable_expander::DefaultTemplateVariableExpander};

  struct FilterNameFilterApplicator;

  //A simple filter that just prepends the filter name the to user-supplied value
  impl FilterApplicator for FilterNameFilterApplicator {

    fn apply_filter(&self, filter_type: &FilterType, value_to_filter: &str) -> String {
       format!("{:?}-{}", filter_type, value_to_filter)
    }
  }

  #[test]
  fn filter_is_generated() {
    let variables_config = r#"
      [
        {
          "variable_name": "project",
          "description": "Name of project",
          "prompt": "Please enter your project name",
              "filters": [
                { "name": "Command",
                  "filter": "Pascal"
                }
              ]
        }
      ]
    "#;

    let variables: TemplateVariables =
      TemplateVariables {
        tokens: serde_json::from_str(&variables_config).unwrap()
      };

    let variable_expander = DefaultTemplateVariableExpander::with_filter_applicator(Box::new(FilterNameFilterApplicator));
    let user_inputs =
      HashMap::from(
        [
          (UserVariableKey::new("project".to_owned()), UserVariableValue::new("blah".to_owned()))
        ]
      );

    let expanded = variable_expander.expand_filters(variables, user_inputs);

    println!("{:?}", &expanded);
    let expanded_variables = expanded.expanded_variables;
     // We expect project and project_command keys

     let user_project_key = ExpandedKey::new("project".to_owned());
     let user_project_value = ExpandedValue::new("blah".to_owned());

     let filter_project_command_key = ExpandedKey::new("project_Command".to_owned());
     let filter_project_command_value = ExpandedValue::new("Pascal-blah".to_owned());

     assert_eq!(expanded_variables.len(), 2);
     assert_eq!(expanded_variables.get(&user_project_key), Some(&user_project_value));
     assert_eq!(expanded_variables.get(&filter_project_command_key), Some(&filter_project_command_value));
  }


  #[test]
  fn filter_is_generated_for_default() {
    let variables_config = r#"
      [
        {
          "variable_name": "project",
          "description": "Name of project",
          "prompt": "Please enter your project name",
              "filters": [
                { "name": "Command",
                  "filter": "Pascal"
                },
                { "name": "__default__",
                  "filter": "Snake"
                }
              ]
        }
      ]
    "#;

    let variables: TemplateVariables =
      TemplateVariables {
        tokens: serde_json::from_str(&variables_config).unwrap()
      };

    let variable_expander = DefaultTemplateVariableExpander::with_filter_applicator(Box::new(FilterNameFilterApplicator));
    let user_inputs =
      HashMap::from(
        [
          (UserVariableKey::new("project".to_owned()), UserVariableValue::new("blah".to_owned()))
        ]
      );

    let expanded = variable_expander.expand_filters(variables, user_inputs);

    println!("{:?}", &expanded);
    let expanded_variables = expanded.expanded_variables;
     // We expect project and project_command keys

     let user_project_key = ExpandedKey::new("project".to_owned());
     let user_project_value = ExpandedValue::new("Snake-blah".to_owned());

     let filter_project_command_key = ExpandedKey::new("project_Command".to_owned());
     let filter_project_command_value = ExpandedValue::new("Pascal-blah".to_owned());

     assert_eq!(expanded_variables.len(), 2);
     assert_eq!(expanded_variables.get(&user_project_key), Some(&user_project_value));
     assert_eq!(expanded_variables.get(&filter_project_command_key), Some(&filter_project_command_value));
  }

  #[test]
  fn filter_is_not_generated_if_not_supplied() {
    let variables_config = r#"
      [
        {
          "variable_name": "MainClass",
          "description": "Name of main class",
          "prompt": "Please enter your main class"
        }
      ]
    "#;

    let variables: TemplateVariables =
      TemplateVariables {
        tokens: serde_json::from_str(&variables_config).unwrap()
      };

    let variable_expander = DefaultTemplateVariableExpander::with_filter_applicator(Box::new(FilterNameFilterApplicator));
    let user_inputs =
      HashMap::from(
        [
          (UserVariableKey::new("MainClass".to_owned()), UserVariableValue::new("blue".to_owned()))
        ]
      );

    let expanded = variable_expander.expand_filters(variables, user_inputs);

    println!("{:?}", &expanded);
    let expanded_variables = expanded.expanded_variables;
     // We expect project and project_command keys

    let user_main_class_key = ExpandedKey::new("MainClass".to_owned());
    let user_main_class_value = ExpandedValue::new("blue".to_owned());

    assert_eq!(expanded_variables.len(), 1);
    assert_eq!(expanded_variables.get(&user_main_class_key), Some(&user_main_class_value));
  }

}
