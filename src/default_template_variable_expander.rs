use crate::template_variable_expander::{ExpandedVariables, TemplateVariableExpander, ExpandedKey, ExpandedValue};
use crate::variables::{UserVariableKey, UserVariableValue, FilterType};
use std::collections::HashMap;

// TODO: Should this be exposed externally?
// Does it make sense for the external system to know about this?
trait FilterApplicator {
  fn apply(&self, filter_type: FilterType, value_to_filter: &str) -> String;
}

struct NoopFilterApplicator;

impl FilterApplicator for NoopFilterApplicator {

  fn apply(&self, filter_type: FilterType, value_to_filter: &str) -> String {
    value_to_filter.to_owned()
  }
}

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

impl DefaultTemplateVariableExpander {
  pub fn new() -> Self {
    Self {
      filter_applicator: Box::new(NoopFilterApplicator)
    }
  }

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

              t
                .filters
                .iter()
                .map(move |f|{
                  let filtered_value = self.filter_applicator.apply(f.filter.clone(), &v.value);
                  let filter_name = format!("{}_{}", k.value.clone(), f.name);
                  (ExpandedKey::new(filter_name.to_owned()), ExpandedValue::new(filtered_value))
                })
                .chain(vec![(o_key, o_value)])
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

    let variable_expander = DefaultTemplateVariableExpander::new();
    let user_inputs =
      HashMap::from(
        [
          (UserVariableKey::new("project".to_owned()), UserVariableValue::new("blah".to_owned()))
        ]
      );

    let expanded = variable_expander.expand_filters(variables, user_inputs);

    println!("{:?}", &expanded);
     // We expect project and project_command keys
     assert_eq!(expanded.expanded_variables.len(), 2);
  }

}
