use ansi_term::Color::{Red, RGB, self};
use std::{format as s, println as p};
use super::Lines;

pub struct Printer;

impl Printer {

  const HEADING_INDENT: &'static str = "";
  const CONTENT_INDENT: &'static str = "  ";
  const GRAY: Color = RGB(82, 97, 111);

  pub fn print_verbose<L: Lines>(category: &str, values: &L) {
    Self::print_verbose_strings(category, values.lines())
  }

  pub fn print_verbose_strings(category: &str, values: Vec<String>) {
    let heading_content = Self::heading(category);
    p!("\n{}{}", Printer::HEADING_INDENT, heading_content);
    for line in values {
      p!("{}{}",Printer::CONTENT_INDENT,  Printer::GRAY.paint(line))
    }
  }

  pub fn heading(heading: &str) -> String {
    s!("{}:", Red.paint(heading))
  }

  pub fn heading_only(heading: &str) {
    let heading_content = Self::heading(heading);
    p!("\n{}{}", Printer::HEADING_INDENT, heading_content)
  }

  pub fn content_only(content: &str) {
    p!("{}{}", Printer::CONTENT_INDENT, Printer::GRAY.paint(content));
  }
}
