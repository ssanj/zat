use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use std::format as s;
use crate::templates::{Choice, TemplateVariable};
use crate::error::{ZatResult, ZatError};
use super::ChoiceStyle;

pub struct SelectionChoiceStyle;

impl ChoiceStyle for SelectionChoiceStyle {

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

