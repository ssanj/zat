use ansi_term::Color::{Yellow, Green};
use std::{format as s};
use super::Lines;

pub struct Printer;

impl Printer {
  pub fn print_verbose<L: Lines>(category: &str, values: &L) {
    Self::print_verbose_strings(category, values.lines())
  }

  pub fn print_verbose_strings(category: &str, values: Vec<String>) {
    let heading_indent = "";
    let content_indent = "  ";

    let heading_content = Self::heading(category);
    println!("\n{}{}", heading_indent, heading_content);
    for line in  values {
      println!("{}{}", Green.paint(content_indent), line)
    }
  }

  fn heading(heading: &str) -> String {
    s!("{}:", Yellow.paint(heading))
  }
}
