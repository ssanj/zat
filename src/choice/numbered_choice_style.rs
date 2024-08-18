use std::fmt::Display;
use std::io::{stdin, Read};
use std::{println as p, format as s};
use crate::error::{ZatResult, ZatError};
use ansi_term::Color::{Red, Yellow};
use ansi_term::Style;

use super::{ChoiceStyle, ChoiceError};

pub struct NumberedChoiceStyle;

impl <T: Display> ChoiceStyle<T> for NumberedChoiceStyle {

  fn get_choice<'a>(prompt: &str, items: &'a [&'a T]) -> ZatResult<&'a T> {
  // fn get_choice<'a>(prompt: &str, items: &'a [&'a Choice]) -> ZatResult<&'a Choice> {
    let mut result = Self::print_menu(prompt, items);
    while let Err(error) = result {
      let error_message = match error {
        ChoiceError::CouldNotReadInput(error) => s!("Could not read input: {error}"),
        ChoiceError::NotANumber(input) => s!("Selection has to be a number: {} is not a number.", input.trim()),
        ChoiceError::OutOfBounds(index) => s!("Selected index: {} is out of bounds. It should be between 1 - {}", index, items.len())
      };
      p!("{}", Red.paint(error_message));
      p!("press {} to continue", Style::new().underline().paint("ENTER"));
      let mut char_buf = [0;1];
      let _ = stdin().read(&mut char_buf);
      p!();
      p!();
      result = Self::print_menu(prompt, items);
    }

    result
      .map_err(|e| ZatError::generic_error("Could not get successful result from choice. ERROR_ID: 1000", e.to_string()))
  }
}

impl NumberedChoiceStyle {

  fn print_menu<'a, T: Display>(prompt: &str, items: &'a [&'a T]) -> Result<&'a T, ChoiceError> {
    p!("{}", Yellow.paint(prompt));

    let it =
      items
        .iter()
        .enumerate()
        .map(|(n, v)| s!("  {} {}", n + 1, v))
        .collect::<Vec<_>>();

    p!("{}", it.join("\n"));

    let mut buffer = String::new();
    stdin()
      .read_line(&mut buffer)
      .map_err(|e| ChoiceError::CouldNotReadInput(e.to_string()))
      .and({
        buffer
          .trim()
          .parse::<usize>()
          .map_err(|_| ChoiceError::NotANumber(buffer.clone()))
          .and_then(|n| {
            if n > 0 && n <= items.len() {
              Ok(
                items[n-1]
              )
            } else {
              Err(ChoiceError::OutOfBounds(n))
            }
          })
      })
  }

}
