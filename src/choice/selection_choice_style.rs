use dialoguer::console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use std::{fmt::Display, format as s};
use crate::error::{ZatResult, ZatError};
use super::ChoiceStyle;

pub struct SelectionChoiceStyle;

impl <T: Display> ChoiceStyle<T> for SelectionChoiceStyle {

  fn get_choice<'a>(prompt: &str, items: &'a [&'a T]) -> ZatResult<&'a T>
  {

    let selections =
      items
        .iter()
        .map(|v| s!("{}", v))
        .collect::<Vec<_>>();

    let theme =
      ColorfulTheme {
        active_item_style: Style::from_dotted_str("green.on_237.bold"),
        ..Default::default()
      };

    FuzzySelect::with_theme(&theme)
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

