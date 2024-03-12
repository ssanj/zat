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
            // 1. The value of the choice key and value match does not match the scope key and value
            // 2. There is no matching choice key
            choices
              .get(&key)
              .filter(|v| v.value.value.as_str() == not_value )
              .is_none()
          },
          Scope::ExcludeChoiceScope(ExcludeChoice { not_choice }) => {
            let key = UserChoiceKey::new(not_choice.to_owned());

            // We want to include the scope if
            // 1. The value of the choice key does not match the scope key
            // 2. There is no matching choice key
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

}
