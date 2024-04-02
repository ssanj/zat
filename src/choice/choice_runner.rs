use crate::{config::UserConfig, templates::TemplateVariables};
use super::SelectedChoices;
use crate::error::ZatResult;

/// Asks the user for to select the choice values and returns the result
pub trait ChoiceRunner {
  fn run_choices(templates: TemplateVariables, user_config: &UserConfig) -> ZatResult<SelectedChoices>;
}
