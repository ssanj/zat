use std::println;

use assert_cmd::Command;
use tempfile::tempdir;

use crate::file_differ::print_changes;
use predicates::prelude::*;
use format as s;

mod file_differ;

#[test]
fn error_message_on_missing_template_dir() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();
  let working_directory = tempdir()?;
  let target_directory = working_directory.into_path().join("errors-no-template_dir").to_string_lossy().to_string();

  let std_err_contains = |_expected:&str| {
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      let lines: Vec<&str> = output.split('\n').collect();
      assert_error_message(&lines)
    })
  };


  cmd
    .arg("--template-dir")
    .arg("./tests/errors/no-template-dir/source")
    .arg("--target-dir")
    .arg(&target_directory)
    .assert()
    .failure()
    .stderr(std_err_contains("blee"));

  assert!(!std::path::Path::new(&target_directory).exists());
  println!("Targer dir {} should not have been created", &target_directory);

  // print_changes(&expected_target_directory, &target_directory);

  // assert!(!dir_diff::is_different(&target_directory, expected_target_directory).unwrap());

  Ok(())
}

/// Assert each line of stderror.
///
/// line0: "Zat failed an with error."
/// line1: Blank
/// line2: <Error Category>:
/// line3: <Error>
/// line4: Blank
/// line5: "Possible fix:"
/// line6: <Fix>
/// line7: Blank
///, error_type: &str, error: &str, fix: &str
fn assert_error_message(lines: &[&str]) -> bool {
  // use ansi_term::Color::Red;
  // use ansi_term::Color::Yellow;
  let error_colour = ansi_term::Color::Red;
  let heading_colour = ansi_term::Color::Yellow;
  let heading_indent = "  ";
  let content_indent = "    ";

  let num_lines = lines.len();

  println!("Received lines:");
  for (index, line) in lines.iter().enumerate() {
    println!("{}. {}", index, line);
  }

  assert_eq!(num_lines, 8, "expected 8 lines but got {}", num_lines);
  assert_eq!(lines[0], error_colour.paint("Zat failed an with error.").to_string(), "line0 is different");
  assert_eq!(lines[1], "", "line1 is different");
  // assert_eq!(lines[2], "");
  assert_eq!(lines[4], "", "line4 is different");
  assert_eq!(lines[5], s!("{}{}", heading_indent, heading_colour.paint("Possible fix").to_string()), "line5 is different");
  assert_eq!(lines[7], "", "line7 is different");

  true
}


fn stdin(responses: &[&str]) -> String {
  let delimited =
    responses
      .join("\n");

  format!("{}\n", delimited) // add the extra newline for complete the final answer
}
