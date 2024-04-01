use super::{ChoiceRunner, SelectedChoices};
use crate::error::{ZatError, ZatResult};
use crate::templates::{Choice, TemplateVariable, TemplateVariables, UserChoiceKey, UserChoiceValue};
use std::collections::HashMap;
use std::format as s;
use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;

pub struct DefaultChoiceRunner;

impl ChoiceRunner for DefaultChoiceRunner {
  fn run_choices(templates: TemplateVariables) -> ZatResult<SelectedChoices> {

    let (choice_variables, other_variables): (Vec<TemplateVariable>, Vec<TemplateVariable>) =
      templates
        .tokens
        .into_iter()
        .partition(|v| !v.choices.is_empty());

    let choice_refs: Vec<(&TemplateVariable, Vec<&Choice>)> =
      choice_variables
        .iter()
        .map(|v| (v, v.choices.iter().collect::<Vec<_>>()))
        .collect::<Vec<_>>();


    // Ask user to select a single choice
    let user_choices: Vec<(&TemplateVariable, Choice)> =
      choice_refs
        .into_iter()
        .map(|(v, ch)| {
          Self::get_choice(v, &ch)
            .cloned()
            .map(|c| (v, c))
        })
        .collect::<ZatResult<Vec<(&TemplateVariable, Choice)>>>()?;


    let choices =
      user_choices
        .into_iter()
        .map(|(variable, choice)| {
          (UserChoiceKey::from(variable.variable_name.as_str()), UserChoiceValue::new(choice))
        })
        .collect::<HashMap<UserChoiceKey, UserChoiceValue>>();

    Ok(SelectedChoices::new(choices, other_variables))
  }
}


impl DefaultChoiceRunner {

  fn get_choice<'a>(variable: &TemplateVariable, items: &'a [&'a Choice]) -> ZatResult<&'a Choice> {

    let selections =
      items
        .iter()
        .map(|v| s!("{} - {}", v.display, v.description))
        .collect::<Vec<_>>();

    FuzzySelect::with_theme(&ColorfulTheme::default())
      .with_prompt(variable.prompt.as_str())
      .default(0)
      .items(&selections)
      .interact()
      .map_err(|e| ZatError::generic_error("Could not get successful result from choice. ERROR_ID: 1000", e.to_string()))
      .and_then(|index| {
        let err = || ZatError::generic_error("Could not get successful result from choice. ERROR_ID: 1001", "Invalid selection index: {index}".to_owned());
        items
          .get(index)
          .cloned()
          .ok_or_else(err)
        })
  }

}
