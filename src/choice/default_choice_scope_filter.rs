use std::collections::HashMap;
use super::ChoiceScopeFilter;
use crate::templates::{scope::{ExcludeChoice, ExcludeChoiceValue, IncludeChoice}, IncludeChoiceValue, Scope, TemplateVariables, UserChoiceKey, UserChoiceValue};

pub struct DefaultChoiceScopeFilter;

impl ChoiceScopeFilter for DefaultChoiceScopeFilter {

    fn filter_scopes(choices: &HashMap<UserChoiceKey, UserChoiceValue>, variables: &mut TemplateVariables) {

      //filter variables that have don't have a scope or where the scope matches one of the choices
      variables
        .tokens
        .retain(|v| {
          match &v.scopes {
            Some(scopes) => Self::filter_scopes_by_choices(choices, scopes),
            None => true // No scopes so include everything
          }
      });
    }
}

enum ChoicesAndScopesDefined {
  ChoicesOnly,
  ScopesOnly,
  ChoicesAndScopes,
  BothMissing,
}

impl DefaultChoiceScopeFilter {

  fn filter_scopes_by_choices(choices: &HashMap<UserChoiceKey, UserChoiceValue>, scopes: &[Scope]) -> bool {
      // Not having any choices defined is a special case.
      // If there are any "include" scopes they should not be included when we don't have the choices defined that should filter them in. Likewise any "exclude" scopes should be included as there are no choices to exclude them.

      let choices_and_scopes = {
        match (choices.is_empty(), scopes.is_empty()) {
          (true, true)   => ChoicesAndScopesDefined::BothMissing,
          (true, false)  => ChoicesAndScopesDefined::ScopesOnly,
          (false, true)  => ChoicesAndScopesDefined::ChoicesOnly,
          (false, false) => ChoicesAndScopesDefined::ChoicesAndScopes,
        }
      };

      match choices_and_scopes {
        ChoicesAndScopesDefined::ChoicesOnly => true, // No scopes to filter by, so include everything
        ChoicesAndScopesDefined::ScopesOnly => {
          scopes
            .iter()
            .map(|scope| match scope {
              // We don't have a matching choice, so these should be excluded
              Scope::IncludeChoiceScope(..)      => false,
              Scope::IncludeChoiceValueScope(..) => false,
              // We don't have a matching choice, so these should be included
              Scope::ExcludeChoiceScope(..)      => true,
              Scope::ExcludeChoiceValueScope(..) => true,
            })
            .fold(false, |acc, v| acc || v)
        },
        ChoicesAndScopesDefined::ChoicesAndScopes => {
          scopes
            .into_iter()
            .any(|scope| Self::is_scope_included(choices, scope)) // If any one of these returns true, the variable is included.
            // - An include that matches always overrides an exclude that matches
            // - An exclude that does not match (is included) always overrides and include that does not match (is excluded)
        },
        ChoicesAndScopesDefined::BothMissing => true, // no choices or scopes, so include everything
    }
  }

  fn is_scope_included(choices: &HashMap<UserChoiceKey, UserChoiceValue>, scope: &Scope) -> bool {
      if choices.is_empty() {
        return true
      } else {
        match scope {
          Scope::IncludeChoiceValueScope(IncludeChoiceValue { choice, value }) => {
            let key = UserChoiceKey::new(choice.to_owned());
            choices
              .get(&key)
              .filter(|v| v.value.value.as_str() == value )
              .is_some()
          },
          Scope::IncludeChoiceScope(IncludeChoice { choice }) => {
            let key = UserChoiceKey::new(choice.to_owned());
            choices
              .get(&key)
              .is_some()
          },
          Scope::ExcludeChoiceValueScope(ExcludeChoiceValue { choice, not_value }) => {
            let key = UserChoiceKey::new(choice.to_owned());

            // We want to include the scope if
            // 1. The value of the choice key's value match does not match the scope key's and value
            // 2. There is no matching choice key
            choices
              .get(&key)
              .filter(|v| v.value.value.as_str() == not_value )
              .is_none()
          },
          Scope::ExcludeChoiceScope(ExcludeChoice { not_choice }) => {
            let key = UserChoiceKey::new(not_choice.to_owned());

            // We want to include the scope if
            // 1. There is no matching choice key
            choices
              .get(&key)
              .is_none()
          },
        }
      }
  }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{FilterType, TemplateVariable, VariableFilter};
    use std::format as s;

    mod is_scope_included {
      use super::*;

      #[test]
      fn without_choices_should_always_return_true() {
          let no_choices = HashMap::new();
          let scope_include_choice_value = Scope::IncludeChoiceValueScope(IncludeChoiceValue::new("choice-key", "choice-value"));
          let scope_include_choice = Scope::IncludeChoiceScope(IncludeChoice::new("choice-key"));
          let scope_exclude_choice = Scope::ExcludeChoiceScope(ExcludeChoice::new("choice-key"));
          let scope_exclude_choice_value = Scope::ExcludeChoiceValueScope(ExcludeChoiceValue::new("choice-key", "choice-value"));

          assert!(DefaultChoiceScopeFilter::is_scope_included(&no_choices, &scope_include_choice_value));
          assert!(DefaultChoiceScopeFilter::is_scope_included(&no_choices, &scope_include_choice));
          assert!(DefaultChoiceScopeFilter::is_scope_included(&no_choices, &scope_exclude_choice));
          assert!(DefaultChoiceScopeFilter::is_scope_included(&no_choices, &scope_exclude_choice_value));
      }

      mod include_choice_value {
        use super::*;

        #[test]
        fn with_a_matching_choice_returns_true() {
          let choices =
            HashMap::from_iter(
              [
                (UserChoiceKey::from("choice-key"), UserChoiceValue::from(("", "", "choice-value")))
              ]
            );

          let scope_include_choice_value = Scope::IncludeChoiceValueScope(IncludeChoiceValue::new("choice-key", "choice-value"));
          assert!(DefaultChoiceScopeFilter::is_scope_included(&choices, &scope_include_choice_value));
        }

        #[test]
        fn with_a_mismatched_choice_value_returns_false() {
          let choices =
            HashMap::from_iter(
              [
                (UserChoiceKey::from("choice-key"), UserChoiceValue::from(("", "", "choice-value-other")))
              ]
            );

          let scope_include_choice_value = Scope::IncludeChoiceValueScope(IncludeChoiceValue::new("choice-key", "choice-value"));
          assert!(!DefaultChoiceScopeFilter::is_scope_included(&choices, &scope_include_choice_value));
        }

        #[test]
        fn with_a_mismatched_choice_key_returns_false() {
          let choices =
            HashMap::from_iter(
              [
                (UserChoiceKey::from("choice-key"), UserChoiceValue::from(("", "", "choice-value")))
              ]
            );

          let scope_include_choice_value = Scope::IncludeChoiceValueScope(IncludeChoiceValue::new("choice-key-other", "choice-value"));
          assert!(!DefaultChoiceScopeFilter::is_scope_included(&choices, &scope_include_choice_value));
        }
      }

      mod include_choice {
        use super::*;

        #[test]
        fn with_a_matching_choice_returns_true() {
          let choices =
            HashMap::from_iter(
              [
                (UserChoiceKey::from("choice-key"), UserChoiceValue::from(("", "", "choice-value")))
              ]
            );

          let scope_include_choice = Scope::IncludeChoiceScope(IncludeChoice::new("choice-key"));

          assert!(DefaultChoiceScopeFilter::is_scope_included(&choices, &scope_include_choice));
        }

        #[test]
        fn with_a_missing_choice_value_returns_false() {
          let choices =
            HashMap::from_iter(
              [
                (UserChoiceKey::from("choice-key-other"), UserChoiceValue::from(("", "", "choice-value")))
              ]
            );

          let scope_include_choice = Scope::IncludeChoiceScope(IncludeChoice::new("choice-key"));
          assert!(!DefaultChoiceScopeFilter::is_scope_included(&choices, &scope_include_choice));
        }
      }

      mod exclude_choice_value {
        use super::*;

        #[test]
        fn with_a_excluded_choice_value_returns_false() {
          let choices =
            HashMap::from_iter(
              [
                (UserChoiceKey::from("choice-key"), UserChoiceValue::from(("", "", "choice-value")))
              ]
            );

          let scope_exclude_choice_value = Scope::ExcludeChoiceValueScope(ExcludeChoiceValue::new("choice-key", "choice-value"));
          assert!(!DefaultChoiceScopeFilter::is_scope_included(&choices, &scope_exclude_choice_value));
        }

        #[test]
        fn with_a_mismatched_choice_value_returns_true() {
          let choices =
            HashMap::from_iter(
              [
                (UserChoiceKey::from("choice-key"), UserChoiceValue::from(("", "", "choice-value-other")))
              ]
            );

          let scope_exclude_choice_value = Scope::ExcludeChoiceValueScope(ExcludeChoiceValue::new("choice-key", "choice-value"));
          assert!(DefaultChoiceScopeFilter::is_scope_included(&choices, &scope_exclude_choice_value));
        }

        #[test]
        fn with_a_missing_choice_key_returns_true() {
          let choices =
            HashMap::from_iter(
              [
                (UserChoiceKey::from("choice-key-other"), UserChoiceValue::from(("", "", "choice-value")))
              ]
            );

          let scope_exclude_choice_value = Scope::ExcludeChoiceValueScope(ExcludeChoiceValue::new("choice-key", "choice-value"));
          assert!(DefaultChoiceScopeFilter::is_scope_included(&choices, &scope_exclude_choice_value));
        }
      }

      mod exclude_choice {
        use super::*;

        #[test]
        fn with_a_excluded_choice_returns_false() {
          let choices =
            HashMap::from_iter(
              [
                (UserChoiceKey::from("choice-key"), UserChoiceValue::from(("", "", "choice-value")))
              ]
            );

          let scope_exclude_choice = Scope::ExcludeChoiceScope(ExcludeChoice::new("choice-key"));
          assert!(!DefaultChoiceScopeFilter::is_scope_included(&choices, &scope_exclude_choice));
        }

        #[test]
        fn with_a_missing_choice_returns_true() {
          let choices =
            HashMap::from_iter(
              [
                (UserChoiceKey::from("choice-key"), UserChoiceValue::from(("", "", "choice-value-other")))
              ]
            );

          let scope_exclude_choice = Scope::ExcludeChoiceScope(ExcludeChoice::new("choice-key-other"));
          assert!(DefaultChoiceScopeFilter::is_scope_included(&choices, &scope_exclude_choice));
        }
      }
    }

    mod filter_scopes {
      use pretty_assertions::assert_eq;
      use super::*;

      #[test]
      fn without_choices_returns_all_excludes_and_non_scoped_variables_only() {
        let no_choices = HashMap::new();
        let variable_1 =
          TemplateVariable::with_scopes("variable_1", vec![Scope::new_include_choice("my-included-choice")]);
        let variable_2 =
          TemplateVariable::with_scopes("variable_2", vec![Scope::new_include_choice_value("my-included-choice-value-key", "my-included-choice-value-value")]);

        let variable_3 =
          TemplateVariable::with_scopes("variable_3", vec![Scope::new_exclude_choice("my-excluded-choice")]);

        let variable_4 =
          TemplateVariable::with_scopes("variable_4", vec![Scope::new_exclude_choice_value("my-excluded-choice-value-key", "my-excluded-choice-value-value")]);

        let variable_5 =
          TemplateVariable::new("variable_5", "variable_5-description", "variable_5-prompt", &[], Some("my-value"));

        let variable_6 =
          TemplateVariable::with_scopes("variable_6", vec![Scope::new_include_choice("my-other-included-choice")]);


        let tokens =
          vec![
            variable_1,
            variable_2,
            variable_3.clone(),
            variable_4.clone(),
            variable_5.clone(),
            variable_6,
          ];

        let mut template_variables =
          TemplateVariables::new(tokens);

        let expected_tokens =
          vec![
            variable_3, // excluded, so include when there are no choices to exclude them
            variable_4, // excluded, so include when there are no choices to exclude them
            variable_5, // no scope
          ];

        let expected_template_variables =
          TemplateVariables::new(expected_tokens);

        DefaultChoiceScopeFilter::filter_scopes(&no_choices, &mut template_variables);

        assert_eq!(expected_template_variables, template_variables)
      }


      #[test]
      fn without_scopes_returns_all_variables() {
        let choices =
          (0 .. 5)
            .into_iter()
            .map(|n| {
              (UserChoiceKey::from("choice-1"), UserChoiceValue::from(("choice-1-display", "choice-1-desc", "choice-1-value")))
            })
            .collect::<Vec<_>>();

        let choices_map = HashMap::from_iter(choices);

        let tokens =
          (0 .. 5)
            .into_iter()
            .map(|n| {
              TemplateVariable::new(
                s!("variable_{n}").as_str(),
                s!("variable_{n}-description").as_str(),
                s!("variable_{n}-prompt").as_str(),
                &[VariableFilter::new(s!("filter-{n}").as_str(), &FilterType::Camel)],
                Some(s!("my-{n}-value").as_str())
              )
          })
          .collect::<Vec<_>>();

        let mut template_variables =
          TemplateVariables::new(tokens);

        let expected_template_variables =
          template_variables.clone();

        DefaultChoiceScopeFilter::filter_scopes(&choices_map, &mut template_variables);

        assert_eq!(expected_template_variables, template_variables)
      }


      #[test]
      fn without_scopes_or_choices_returns_all_variables() {
        let choices_map = HashMap::new();

        let tokens =
          (0 .. 5)
            .into_iter()
            .map(|n| {
              TemplateVariable::new(
                s!("variable_{n}").as_str(),
                s!("variable_{n}-description").as_str(),
                s!("variable_{n}-prompt").as_str(),
                &[VariableFilter::new(s!("filter-{n}").as_str(), &FilterType::Camel)],
                Some(s!("my-{n}-value").as_str())
              )
          })
          .collect::<Vec<_>>();

        let mut template_variables =
          TemplateVariables::new(tokens);

        let expected_template_variables =
          template_variables.clone();

        DefaultChoiceScopeFilter::filter_scopes(&choices_map, &mut template_variables);

        assert_eq!(expected_template_variables, template_variables)
      }


      #[test]
      fn with_matching_scope_returns_matching_variables() {
        let choices =
          (0 .. 5)
            .into_iter()
            .map(|n| {
              (
                UserChoiceKey::from(s!("choice-{n}").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-{n}-display").as_str(),
                    s!("choice-{n}-desc").as_str(),
                    s!("choice-{n}-value").as_str()))
                )
            })
            .collect::<Vec<_>>();

        let choices_map = HashMap::from_iter(choices);

        let tokens =
          (0 .. 5)
            .into_iter()
            .map(|n| {
              TemplateVariable::with_scopes(
                s!("variable_{n}").as_str(),
                vec![Scope::new_include_choice(s!("choice-{}", n + 2).as_str())]
              )
          })
          .collect::<Vec<_>>();

        let mut template_variables =
          TemplateVariables::new(tokens.clone());

        let expected_template_variables =
          TemplateVariables::new(
            vec![
              tokens.get(0).cloned().unwrap(), // included by choice-2
              tokens.get(1).cloned().unwrap(), // included by choice-3
              tokens.get(2).cloned().unwrap(), // included by choice-4
            ]
          );

        DefaultChoiceScopeFilter::filter_scopes(&choices_map, &mut template_variables);

        assert_eq!(expected_template_variables, template_variables)
      }

      #[test]
      fn with_matching_scope_value_returns_matching_variables() {
        let choices =
          vec![
            (
                UserChoiceKey::from(s!("choice-x").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-x-display").as_str(),
                    s!("choice-x-desc").as_str(),
                    s!("choice-x-value").as_str()))
            ),
            (
                UserChoiceKey::from(s!("choice-2").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-2-display").as_str(),
                    s!("choice-2-desc").as_str(),
                    s!("choice-2-value").as_str()))
            )
          ];

        let choices_map = HashMap::from_iter(choices);

        let tokens =
          (0 .. 5)
            .into_iter()
            .map(|n| {
              TemplateVariable::with_scopes(
                s!("variable_{n}").as_str(),
                vec![Scope::new_include_choice_value(s!("choice-{}", n).as_str(), s!("choice-{}-value", n).as_str())]
              )
          })
          .collect::<Vec<_>>();


        let mut template_variables =
          TemplateVariables::new(tokens.clone());

        let expected_template_variables =
          TemplateVariables::new(
            vec![
              tokens.get(2).cloned().unwrap(), // included by choice-2, choice-2-value
            ]
          );

        DefaultChoiceScopeFilter::filter_scopes(&choices_map, &mut template_variables);

        assert_eq!(expected_template_variables, template_variables)
      }

      #[test]
      fn with_matching_exclude_scope_returns_all_but_excluded_variables() {
        let choices =
          vec![
            (
                UserChoiceKey::from(s!("choice-x").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-x-display").as_str(),
                    s!("choice-x-desc").as_str(),
                    s!("choice-x-value").as_str()))
            ),
            (
                UserChoiceKey::from(s!("choice-2").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-2-display").as_str(),
                    s!("choice-2-desc").as_str(),
                    s!("choice-2-value").as_str()))
            )
          ];

        let choices_map = HashMap::from_iter(choices);

        let tokens =
          (0 .. 5)
            .into_iter()
            .map(|n| {
              TemplateVariable::with_scopes(
                s!("variable_{n}").as_str(),
                vec![Scope::new_exclude_choice(s!("choice-{}", n).as_str())]
              )
          })
          .collect::<Vec<_>>();


        let mut template_variables =
          TemplateVariables::new(tokens.clone());

        let expected_template_variables =
          TemplateVariables::new(
              tokens
                .into_iter()
                .filter(|i| i.variable_name != "variable_2")
                .collect::<Vec<_>>(), //Variable 0, 1, 3, 4. Variable 2 is excluded
          );

        DefaultChoiceScopeFilter::filter_scopes(&choices_map, &mut template_variables);

        assert_eq!(expected_template_variables, template_variables)
      }


      #[test]
      fn with_matching_exclude_scope_value_returns_all_but_excluded_variables() {
        let choices =
          vec![
            (
                UserChoiceKey::from(s!("choice-x").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-x-display").as_str(),
                    s!("choice-x-desc").as_str(),
                    s!("choice-x-value").as_str()))
            ),
            (
                UserChoiceKey::from(s!("choice-2").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-2-display").as_str(),
                    s!("choice-2-desc").as_str(),
                    s!("choice-2-value").as_str()))
            )
          ];

        let choices_map = HashMap::from_iter(choices);

        let tokens =
          (0 .. 5)
            .into_iter()
            .map(|n| {
              TemplateVariable::with_scopes(
                s!("variable_{n}").as_str(),
                vec![Scope::new_exclude_choice_value(s!("choice-{}", n).as_str(), s!("choice-{}-value", n).as_str())]
              )
          })
          .collect::<Vec<_>>();


        let mut template_variables =
          TemplateVariables::new(tokens.clone());

        let expected_template_variables =
          TemplateVariables::new(
              tokens
                .into_iter()
                .filter(|i| i.variable_name != "variable_2")
                .collect::<Vec<_>>(), //Variable 0, 1, 3, 4. Variable 2 is excluded
          );

        DefaultChoiceScopeFilter::filter_scopes(&choices_map, &mut template_variables);

        assert_eq!(expected_template_variables, template_variables)
      }


      // includes always override excludes.
      // excludes have to be unanimous.
      #[test]
      fn with_include_overrides_exclude_for_variable_scope() {
        let choices =
          vec![
            (
                UserChoiceKey::from(s!("choice-x").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-x-display").as_str(),
                    s!("choice-x-desc").as_str(),
                    s!("choice-x-value").as_str()))
            ),
            (
                UserChoiceKey::from(s!("choice-2").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-2-display").as_str(),
                    s!("choice-2-desc").as_str(),
                    s!("choice-2-value").as_str()))
            )
          ];

        let choices_map = HashMap::from_iter(choices);

        let tokens =
          (0 .. 5)
            .into_iter()
            .map(|n| {
              if n == 2 {
                TemplateVariable::with_scopes(
                  s!("variable_2").as_str(),
                  vec![
                    Scope::new_exclude_choice_value("choice-2", "choice-2-value"), // exclude for choice-2
                    Scope::new_include_choice_value("choice-2", "choice-2-value"), // include for choice-2
                  ]
                )
              } else {
                TemplateVariable::with_scopes(
                  s!("variable_{n}").as_str(),
                  vec![]
                )
              }
          })
          .collect::<Vec<_>>();


        let mut template_variables =
          TemplateVariables::new(tokens.clone());

        let expected_template_variables =
          TemplateVariables::new(
              tokens
                .into_iter()
                .collect::<Vec<_>>(),
          );

        DefaultChoiceScopeFilter::filter_scopes(&choices_map, &mut template_variables);

        assert_eq!(expected_template_variables, template_variables)
      }


      #[test]
      fn with_a_mix_of_scopes_and_choices() {
        let choices =
          vec![
            (
                UserChoiceKey::from(s!("choice-x").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-x-display").as_str(),
                    s!("choice-x-desc").as_str(),
                    s!("choice-x-value").as_str()))
            ),
            (
                UserChoiceKey::from(s!("choice-2").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-2-display").as_str(),
                    s!("choice-2-desc").as_str(),
                    s!("choice-2-value").as_str()))
            ),
            (
                UserChoiceKey::from(s!("choice-y").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-y-display").as_str(),
                    s!("choice-y-desc").as_str(),
                    s!("choice-y-value").as_str()))
            ),
            (
                UserChoiceKey::from(s!("choice-z").as_str()),
                UserChoiceValue::from(
                  (
                    s!("choice-z-display").as_str(),
                    s!("choice-z-desc").as_str(),
                    s!("choice-z-value").as_str()))
            ),
          ];

        let choices_map = HashMap::from_iter(choices);

        let tokens =
          vec![
              // included, include choice value overrides exclude choice value, irrespective of order
              TemplateVariable::with_scopes(
                s!("variable_x").as_str(),
                vec![ // include, then exclude
                  Scope::new_include_choice_value("choice-x", "choice-x-value"),
                  Scope::new_exclude_choice_value("choice-2", "choice-2-value"),
                ]

              ),
              // included, include choice value overrides exclude choice value, irrespective of order
              TemplateVariable::with_scopes(
                s!("variable_2").as_str(),
                vec![ // exclude, then include
                  Scope::new_exclude_choice_value("choice-2", "choice-2-value"),
                  Scope::new_include_choice_value("choice-x", "choice-x-value"),
                ]
              ),
              // included, because excludes only work if a choice is not present
              // excluded, because include choice not found
              // if at least one scope matches, then include
              TemplateVariable::with_scopes(
                s!("variable_w").as_str(),
                vec![ // exclude, then include. Since we don't have a choice-w, the exclude is negated (because an include) and the include does not work because we don't have choice-w
                  Scope::new_exclude_choice_value("choice-w", "choice-w-value-1"),
                  Scope::new_include_choice_value("choice-w", "choice-w-value-2"),
                ]
              ),
              // excluded, choice value exclude
              TemplateVariable::with_scopes(
                s!("variable_3").as_str(),
                vec![
                  Scope::new_exclude_choice_value("choice-2", "choice-2-value"),
                ]
              ),
              // included - no scope
              TemplateVariable::new(
                s!("variable_4").as_str(),
                s!("variable_4_description").as_str(),
                s!("variable_4_prompt").as_str(),
                &[],
                Option::default()
              ),
              // included, choice include
              TemplateVariable::with_scopes(
                s!("variable_y").as_str(),
                vec![
                  Scope::new_include_choice("choice-y"),
                ]
              ),
              // excluded, choice exclude
              TemplateVariable::with_scopes(
                s!("variable_z").as_str(),
                vec![
                  Scope::new_exclude_choice("choice-z"),
                ]
              ),
              // excluded, choice include not found
              TemplateVariable::with_scopes(
                s!("variable_a").as_str(),
                vec![
                  Scope::new_include_choice("choice-a"),
                ]
              ),
              // included, choice exclude not found (exclude negated)
              TemplateVariable::with_scopes(
                s!("variable_b").as_str(),
                vec![
                  Scope::new_exclude_choice("choice-b"),
                ]
              ),
          ];


        let mut template_variables =
          TemplateVariables::new(tokens.clone());

        let expected_template_variables =
          TemplateVariables::new(
              vec![
                tokens.get(0).unwrap().clone(), // variable_x
                tokens.get(1).unwrap().clone(), // variable_2
                tokens.get(2).unwrap().clone(), // variable_w
                // variable_3 is excluded
                tokens.get(4).unwrap().clone(), // variable_4
                tokens.get(5).unwrap().clone(), // variable_y
                // variable_z is excluded
                // variable_a is excluded
                tokens.get(8).unwrap().clone()  // variable_b
              ]
          );

        DefaultChoiceScopeFilter::filter_scopes(&choices_map, &mut template_variables);

        assert_eq!(expected_template_variables, template_variables)
      }
    }

    // TODO: Add test missing choices and scopes
}
