use ansi_term::Color::{Yellow, Green};
use std::{format as s};
use super::Lines;

pub struct Printer;

impl Printer {
  pub fn print_verbose<L: Lines>(category: &str, values: &L) {
    let heading_indent = "";
    let content_indent = "  ";

    let heading_content = Self::heading(category);
    println!("\n{}{}", heading_indent, heading_content);
    for line in  values.lines() {
      println!("{}{}", Green.paint(content_indent), line)
    }
  }

  fn heading(heading: &str) -> String {
    s!("{}:", Yellow.paint(heading))
  }
}
