use assert_cmd::Command;
use file_differ::print_diff;
use tempfile::tempdir;
use predicates::prelude::*;
use format as s;

mod file_differ;

#[derive(Clone)]
struct ErrorParts(String, String, String);

#[test]
fn error_message_on_missing_template_dir() -> Result<(), Box<dyn std::error::Error>> {
  let source_directory = "./tests/errors/no-template-dir/source";

  let error =
    ErrorParts(
      "Got a configuration error".to_owned(),
      s!("The Zat template directory '{}' does not exist. It should exist so Zat can read the templates configuration.", source_directory),
      s!("Please create the Zat template directory '{}' with the Zat template folder structure. See `zat -h` for more.", source_directory),
    );

  assert_source_dir_error(error, source_directory)
}

#[test]
fn error_message_on_missing_template_files_dir() -> Result<(), Box<dyn std::error::Error>> {
  let source_directory = "./tests/errors/no-template-files-dir/source";

  let error =
    ErrorParts(
      "Got a configuration error".to_owned(),
      s!("The Zat template files directory '{}/template' does not exist. It should exist so Zat can read the template files.", source_directory),
      s!("Please create the Zat template files directory '{}/template' with the necessary template files. See `zat -h` for more details.", source_directory),
    );

  assert_source_dir_error(error, source_directory)
}

/// Asserts error output from the source directory and ensure the target directory has not been created
fn assert_source_dir_error(error: ErrorParts, source_directory: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();
  let working_directory = tempdir()?;
  let target_directory = working_directory.into_path().join("errors-no-template-dir").to_string_lossy().to_string();

  let std_err_contains = |error: ErrorParts| {
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      let lines: Vec<&str> = output.split('\n').collect();
      assert_error_message(&lines, error.clone())
    })
  };

  cmd
    .arg("--template-dir")
    .arg(source_directory)
    .arg("--target-dir")
    .arg(&target_directory)
    .assert()
    .failure()
    .stderr(std_err_contains(error));

  assert!(!std::path::Path::new(&target_directory).exists());
  println!("Targer dir {} should not have been created", &target_directory);

  Ok(())
}

#[test]
fn error_message_on_missing_target_dir() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();
  let target_directory = tempdir()?;
  let target_pathbuf = target_directory.into_path();
  let target_string = target_pathbuf.to_string_lossy().to_string();

  let std_err_contains = |error: ErrorParts| {
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      let lines: Vec<&str> = output.split('\n').collect();
      assert_error_message(&lines, error.clone())
    })
  };

  let error =
    ErrorParts(
      "Got a configuration error".to_owned(),
      s!("The target directory '{}' should not exist. It will be created when Zat processes the template files.", &target_string),
      "Please supply an empty directory for the target.".to_owned(),
    );

  cmd
    .arg("--template-dir")
    .arg("./tests/errors/no-target-dir/source")
    .arg("--target-dir")
    .arg(&target_string)
    .assert()
    .failure()
    .stderr(std_err_contains(error));

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
fn assert_error_message(lines: &[&str], error_parts: ErrorParts) -> bool {

  let error_colour = ansi_term::Color::Red;
  let heading_colour = ansi_term::Color::Yellow;
  let heading_indent = "  ";
  let content_indent = "    ";
  let ErrorParts(error_type, error, fix) = error_parts;

  let num_lines = lines.len();

  println!("Received lines:");
  for (index, line) in lines.iter().enumerate() {
    println!("{}. {}", index, line);
  }


  assert_eq!(num_lines, 8, "expected 8 lines but got {}", num_lines);

  let line0 = assert_line(0, lines[0], error_colour.paint("Zat failed an with error.").to_string().as_str());
  let line1 = assert_line(1, lines[1], "",);
  let line2 = assert_line(2, lines[2], s!("{}{}:", heading_indent, heading_colour.paint(error_type).to_string()).as_str());
  let line3 = assert_line(3, lines[3], s!("{}{}", content_indent, error).as_str());
  let line4 = assert_line(4, lines[4], "");
  let line5 = assert_line(5, lines[5], s!("{}{}:", heading_indent, heading_colour.paint("Possible fix").to_string()).as_str());
  let line6 = assert_line(6, lines[6], s!("{}{}", content_indent, fix).as_str());
  let line7 = assert_line(7, lines[7], "");

  line0 &&
  line1 &&
  line2 &&
  line3 &&
  line4 &&
  line5 &&
  line6 &&
  line7
}

fn assert_line(number: u8, actual: &str, expected: &str) -> bool {
  if actual != expected {
    print_diff(actual, expected);
    false
  } else {
    true
  }
}


fn stdin(responses: &[&str]) -> String {
  let delimited =
    responses
      .join("\n");

  format!("{}\n", delimited) // add the extra newline for complete the final answer
}
