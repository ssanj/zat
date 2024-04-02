use super::{ChoiceRunner, SelectedChoices, ChoiceStyle, NumberedChoiceStyle, SelectionChoiceStyle};
use crate::config::UserConfig;
use crate::error::ZatResult;
use crate::templates::{Choice, TemplateVariable, TemplateVariables, UserChoiceKey, UserChoiceValue};
use std::collections::HashMap;


pub struct DefaultChoiceRunner;


impl ChoiceRunner for DefaultChoiceRunner {
  fn run_choices(templates: TemplateVariables, user_config: &UserConfig) -> ZatResult<SelectedChoices> {

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


    let choice_style = match user_config.menu_style {
        crate::config::user_config::MenuStyle::Numbered => <NumberedChoiceStyle as ChoiceStyle>::get_choice,
        crate::config::user_config::MenuStyle::Selection => <SelectionChoiceStyle as ChoiceStyle>::get_choice,
    };

    // Ask user to select a single choice
    let user_choices: Vec<(&TemplateVariable, Choice)> =
      choice_refs
        .into_iter()
        .map(|(v, ch)| {
          choice_style(v, &ch)
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
