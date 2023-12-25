use std::{println as p, eprintln as e};
use ansi_term::Colour::{Yellow, Red, Green};

pub struct Logger;

impl Logger {

  pub (crate) fn info(message: &str) {
    p!("\n{}", Yellow.paint(message))
  }

  pub (crate) fn info_str(message: &str) -> String {
    Yellow.paint(message).to_string()
  }

  pub (crate) fn success(message: &str) {
    p!("\n{}", Green.paint(message))
  }

  pub (crate) fn coloured(message: &str) {
    p!("\n{}", message)
  }

  pub (crate) fn warn(message: &str) {
    p!("\n{}", Red.paint(message))
  }

  pub (crate) fn error(message: &str, error: String) {
    e!("\n{}{}", Red.paint(message), error)
  }
}
