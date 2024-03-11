use super::{ChoiceRunner, ChoiceError, SelectedChoices};
use crate::error::{ZatError, ZatResult};
use crate::templates::{Choice, TemplateVariables, TemplateVariable};
use std::{format as s, io::Read, println as p};
use ansi_term::Color::{Yellow, Red};
use ansi_term::Style;
use std::io::stdin;

pub struct DefaultChoiceRunner;

impl ChoiceRunner for DefaultChoiceRunner {
  fn run_choices(templates: TemplateVariables) -> ZatResult<SelectedChoices> {
    let choices: Vec<TemplateVariable> =
      templates
        .tokens
        .into_iter()
        .filter(|v| {
          v.choice.is_empty()
        })
        .collect::<Vec<_>>();

    let choice_refs =
      choices
        .iter()
        .map(|v| (v.prompt.as_str(), v.choice.iter().collect::<Vec<_>>()) )
        .collect::<Vec<_>>();


    let user_result: Vec<Choice> =
      choice_refs
        .into_iter()
        .map(|(pr, ch)| Self::get_choice(pr, &ch).cloned())
        .collect::<ZatResult<Vec<_>>>()?;


    Ok(SelectedChoices::new(user_result))
  }
}


impl DefaultChoiceRunner {
  fn print_menu<'a>(prompt: &str, items: &'a [&'a Choice]) -> Result<&'a Choice, ChoiceError> {
    p!("{}", Yellow.paint(prompt));

    let it =
      items
        .iter()
        .enumerate()
        .map(|(n, v)| s!("  {} {} {}", n + 1, v.display, v.description))
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


  fn get_choice<'a>(prompt: &str, items: &'a [&'a Choice]) -> ZatResult<&'a Choice> {
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

    result.map_err(|e| ZatError::generic_error("Could not get successful result from choice. ERROR_ID: 1000", e.to_string()))
  }
}
