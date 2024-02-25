use super::{ExpandedVariables, TemplateVariableExpander, ExpandedKey, ExpandedValue, DEFAULT_FILTER, FilterApplicator};
use crate::templates::{TemplateVariables, UserVariableKey, UserVariableValue};
use std::collections::HashMap;

pub struct DefaultTemplateVariableExpander {
  filter_applicator: Box<dyn FilterApplicator>
}


impl DefaultTemplateVariableExpander {

  pub fn with_filter_applicator(filter_applicator: Box<dyn FilterApplicator>) -> Self {
    Self {
      filter_applicator
    }
  }
}


impl TemplateVariableExpander for DefaultTemplateVariableExpander {
    fn expand_filters(&self, variables: TemplateVariables, user_inputs: HashMap<UserVariableKey, UserVariableValue>) -> ExpandedVariables {

      let expanded_variables: HashMap<ExpandedKey, ExpandedValue> =
        variables
          .tokens
          .iter()
          .filter_map(|t|{
            let key = t.variable_name.clone();
            user_inputs
              .get(&UserVariableKey::new(key.clone())) // The user supplied the value for this filter key
              .map(|v|{
                (t, ExpandedKey::new(&key), ExpandedValue::new(&v.value)) // These are not really expanded, we just put them into the Expanded containers to align the types
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
                    let filtered_value = self.filter_applicator.apply_filter(&f.filter, &v.value); // Apply the filter to the value supplied by the user
                    let filter_name =
                      if f.name == DEFAULT_FILTER {
                        k.value.clone() //if the key name is __default__ then use the original KEYNAME
                      } else {
                        format!("{}__{}", k.value.clone(), f.name) // otherwise the key name is KEYNAME__FILTERNAME
                      };

                    (ExpandedKey::new(&filter_name), ExpandedValue::new(&filtered_value))
                  });

                // Chain with original values, so they can get overwritten if needed by __default__
                // It's important to have the original values first in the chain otherwise they
                // will overwrite the filtered values. (Why? Because default filters with (__default__) will have the same name as the original, and overwrite them)
                vec![(o_key, o_value)]
                  .into_iter()
                  .chain(filtered_values)

          })
          .collect();

        // for each filter defined:
        // 1. apply filter with the user-supplied value
        // 2. insert filtered value under filtered name (except with __default__) KEYNAME_FILTERNAME

      ExpandedVariables::new(expanded_variables)
    }
}


#[cfg(test)]
mod tests {

  use super::*;
  use super::super::DefaultTemplateVariableExpander;
  use crate::templates::{TemplateVariables, FilterType};


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

     let user_project_key = ExpandedKey::new("project");
     let user_project_value = ExpandedValue::new("blah");

     let filter_project_command_key = ExpandedKey::new("project__Command");
     let filter_project_command_value = ExpandedValue::new("Pascal-blah");

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

     let user_project_key = ExpandedKey::new("project");
     let user_project_value = ExpandedValue::new("Snake-blah");

     let filter_project_command_key = ExpandedKey::new("project__Command");
     let filter_project_command_value = ExpandedValue::new("Pascal-blah");

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

    let user_main_class_key = ExpandedKey::new("MainClass");
    let user_main_class_value = ExpandedValue::new("blue");

    assert_eq!(expanded_variables.len(), 1);
    assert_eq!(expanded_variables.get(&user_main_class_key), Some(&user_main_class_value));
  }

}
