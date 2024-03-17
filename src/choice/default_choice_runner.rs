use super::{ChoiceRunner, ChoiceError, SelectedChoices};
use crate::error::{ZatError, ZatResult};
use crate::templates::{Choice, TemplateVariable, TemplateVariables, UserChoiceKey, UserChoiceValue};
use std::collections::HashMap;
use std::{format as s, io::Read, println as p};
use ansi_term::Color::{Yellow, Red};
use ansi_term::Style;
use std::io::stdin;

pub struct DefaultChoiceRunner;

impl ChoiceRunner for DefaultChoiceRunner {
  fn run_choices(templates: TemplateVariables) -> ZatResult<SelectedChoices> {

    let (choice_variables, other_variables): (Vec<TemplateVariable>, Vec<TemplateVariable>) =
      templates
        .tokens
        .into_iter()
        .partition(|v| !v.choice.is_empty());

    let choice_refs: Vec<(&TemplateVariable, Vec<&Choice>)> =
      choice_variables
        .iter()
        .map(|v| (v, v.choice.iter().collect::<Vec<_>>()))
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


  fn get_choice<'a>(variable: &TemplateVariable, items: &'a [&'a Choice]) -> ZatResult<(&'a Choice)> {
    let mut result = Self::print_menu(variable.prompt.as_str(), items);
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
      result = Self::print_menu(variable.prompt.as_str(), items);
    }

    result
      .map_err(|e| ZatError::generic_error("Could not get successful result from choice. ERROR_ID: 1000", e.to_string()))
  }
}
