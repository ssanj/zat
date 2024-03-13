use std::collections::HashMap;
use super::ChoiceScopeFilter;
use crate::templates::{scope::{ExcludeChoice, ExcludeChoiceValue, IncludeChoice}, IncludeChoiceValue, Scope, TemplateVariables, UserChoiceKey, UserChoiceValue};

pub struct DefaultChoiceScopeFilter;

impl ChoiceScopeFilter for DefaultChoiceScopeFilter {

    // TODO: Test all combinations through this function.
    fn filter_scopes(choices: &HashMap<UserChoiceKey, UserChoiceValue>, variables: &mut TemplateVariables) {

       //filter variables that have don't have a scope or where the scope matches one of the choices
      variables
        .tokens
        .retain(|v| {
          match &v.scope {
            Some(scopes) => Self::filter_scopes_by_choices(choices, scopes),
            None => true // No scopes so include everything
          }
      });
    }
}

impl DefaultChoiceScopeFilter {

  fn filter_scopes_by_choices(choices: &HashMap<UserChoiceKey, UserChoiceValue>, scopes: &[Scope]) -> bool {
      scopes
        .into_iter()
        .any(|scope| Self::is_scope_included(choices, scope))
  }

  fn is_scope_included(choices: &HashMap<UserChoiceKey, UserChoiceValue>, scope: &Scope) -> bool {
      if choices.is_empty() {
        return true
      } else {
        match scope {
          Scope::IncludeChoiceValueScope(IncludeChoiceValue { choice, value }) => {
            println!("IncludeChoiceValueScope");
            let key = UserChoiceKey::new(choice.to_owned());
            choices
              .get(&key)
              .filter(|v| v.value.value.as_str() == value )
              .is_some()
          },
          Scope::IncludeChoiceScope(IncludeChoice { choice }) => {
            let key = UserChoiceKey::new(choice.to_owned());
            println!("IncludeChoiceScope: key: {}, choices: {:?}", &key.value, &choices);
            choices
              .get(&key)
              .is_some()
          },
          Scope::ExcludeChoiceValueScope(ExcludeChoiceValue { choice, not_value }) => {
            println!("ExcludeChoiceValueScope");
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
            println!("ExcludeChoiceScope");
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
    use pretty_assertions::assert_eq;
    use std::format as s;

    #[test]
    fn is_scope_included_without_choices_should_always_return_true() {
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
      fn is_scope_included_with_a_matching_choice_returns_true() {
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
      fn is_scope_included_with_a_mismatched_choice_value_returns_false() {
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
      fn is_scope_included_with_a_mismatched_choice_key_returns_false() {
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
      fn is_scope_included_with_a_matching_choice_returns_true() {
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
      fn is_scope_included_with_a_mismatched_choice_value_returns_false() {
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
      fn is_scope_included_with_a_excluded_choice_value_returns_false() {
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
      fn is_scope_included_with_a_mismatched_choice_value_returns_true() {
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
      fn is_scope_included_with_a_missing_choice_key_returns_true() {
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
      fn is_scope_included_with_a_excluded_choice_returns_false() {
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
      fn is_scope_included_with_a_missing_choice_returns_true() {
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

    #[test]
    fn filter_scopes_without_choices_returns_all_variables() {
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

      let tokens =
        vec![
          variable_1,
          variable_2,
          variable_3,
          variable_4,
          variable_5,
        ];

      let mut template_variables =
        TemplateVariables::new(tokens);

      let expected_template_variables =
        template_variables.clone();

      DefaultChoiceScopeFilter::filter_scopes(&no_choices, &mut template_variables);

      assert_eq!(expected_template_variables, template_variables)
    }


    #[test]
    fn filter_scopes_without_scopes_returns_all_variables() {
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
    fn filter_scopes_without_matching_scope_returns_matching_variables() {
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
}
