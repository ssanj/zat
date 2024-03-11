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
            Some(scopes) if choices.is_empty() => true, // No choices, so include everything
            Some(scopes) => Self::filter_by_scope(choices, scopes),
            None => true // No scopes so include everything
          }
      });
    }
}

impl DefaultChoiceScopeFilter {

  fn filter_by_scope(choices: &HashMap<UserChoiceKey, UserChoiceValue>, scopes: &[Scope]) -> bool {
      scopes
        .into_iter()
        .any(|scope| {
          match scope {
            Scope::IncludeChoiceValueScope(IncludeChoiceValue { choice, value }) => todo!(),
            Scope::IncludeChoiceScope(IncludeChoice { choice }) => todo!(),
            Scope::ExcludeChoiceValueScope(ExcludeChoiceValue { choice, not_value }) => todo!(),
            Scope::ExcludeChoiceScope(ExcludeChoice { not_choice }) => todo!(),
          }
        })
    }
}
