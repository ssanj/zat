use std::fmt::Display;

use crate::error::ZatResult;

pub trait ChoiceStyle<T: Display> {
  fn get_choice<'a>(prompt: &str, items: &'a [&'a T]) -> ZatResult<&'a T>;
}
