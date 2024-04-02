use crate::templates::{Choice, TemplateVariable};
use crate::error::ZatResult;

pub trait ChoiceStyle {
  fn get_choice<'a>(variable: &TemplateVariable, items: &'a [&'a Choice]) -> ZatResult<&'a Choice>;
}
