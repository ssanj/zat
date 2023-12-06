use std::{println as p, eprintln as e};
use ansi_term::Color::{Yellow, Red};

pub struct Logger;

impl Logger {

  pub (crate) fn info(message: &str) {
    p!("\n{}", Yellow.paint(message))
  }

  pub (crate) fn warn(message: &str) {
    p!("\n{}", Red.paint(message))
  }

  pub (crate) fn error(message: &str, error: String) {
    e!("\n{}{}", Red.paint(message), error)
  }
}
