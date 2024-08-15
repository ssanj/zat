use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use std::format as s;
use crate::templates::Choice;
use crate::error::{ZatResult, ZatError};
use super::ChoiceStyle;

pub struct SelectionChoiceStyle;

impl ChoiceStyle<Choice> for SelectionChoiceStyle {

  fn get_choice<'a>(prompt: &str, items: &'a [&'a Choice]) -> ZatResult<&'a Choice> {

    let selections =
      items
        .iter()
        .map(|v| s!("{}", v))
        .collect::<Vec<_>>();

    FuzzySelect::with_theme(&ColorfulTheme::default())
      .with_prompt(prompt)
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

