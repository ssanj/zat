use assert_cmd::Command;
use file_differ::print_diff;
use tempfile::tempdir;
use predicates::prelude::*;
use format as s;

mod file_differ;

#[test]
fn error_message_on_missing_template_dir() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "no-template-dir";
  let source_directory = s!("./tests/errors/{}/source", test_directory);

  let error_parts =
    ErrorParts::new(
      "Got a configuration error".to_owned(),
      s!("The Zat template directory '{}' does not exist. It should exist so Zat can read the templates configuration.", source_directory),
      s!("Please create the Zat template directory '{}' with the Zat template folder structure. See `zat -h` for more.", source_directory),
    );

  let error_test_config = ErrorTestConfig::source_no_input_directory_not_exists(test_directory, error_parts);

  run_error_test(error_test_config)
}

#[test]
fn error_message_on_missing_template_files_dir() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "no-template-files-dir";
  let source_directory = s!("./tests/errors/{}/source", test_directory);

  let error_parts =
    ErrorParts::new(
      "Got a configuration error".to_owned(),
      s!("The Zat template files directory '{}/template' does not exist. It should exist so Zat can read the template files.", source_directory),
      s!("Please create the Zat template files directory '{}/template' with the necessary template files. See `zat -h` for more details.", source_directory),
    );

  let error_test_config = ErrorTestConfig::source_no_input_directory_not_exists(test_directory, error_parts);
  run_error_test(error_test_config)
}

#[test]
fn error_message_on_missing_variables_file() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "no-variables-file";
  let source_directory = s!("./tests/errors/{}/source", test_directory);

  let error_parts =
    ErrorParts::new(
      "Got a error processing variables".to_owned(),
      s!("Variable file '{}/.variables.zat-prompt' does not exist. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", source_directory),
      s!("Please create the variable file '{}/.variables.zat-prompt' with the required tokens. See `zat -h` for more details.", source_directory),
    );

  let error_test_config = ErrorTestConfig::source_no_input_directory_not_exists(test_directory, error_parts);
  run_error_test(error_test_config)
}

#[test]
fn error_message_on_non_json_variables_file() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "non-json-variables-file";
  let source_directory = s!("./tests/errors/{}/source", test_directory);

  let error_parts =
    ErrorParts::new(
      "Got a error processing variables".to_owned(),
      s!("Variable file '{}/.variables.zat-prompt' could not be decoded as JSON into the expected format. It failed decoding with this error: invalid type: integer `123`, expected a sequence at line 1 column 3. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", source_directory),
      s!("Make the variable file '{}/.variables.zat-prompt' is a valid JSON file in the format required by Zat. See `zat -h` for more details on the format", source_directory),
    );

  let error_test_config = ErrorTestConfig::source_no_input_directory_not_exists(test_directory, error_parts);
  run_error_test(error_test_config)
}

#[test]
fn error_message_on_no_template_files() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "no-template-files";
  let source_directory = s!("./tests/errors/{}/source", test_directory);

  let error_parts =
    ErrorParts::new(
      "There was an error running the template".to_owned(),
      s!("There are no template files to process in the template directory '{}/template'.", source_directory),
      s!("Create at least one file in the template directory '{}/template' for processing.", source_directory),
    );

  let input = &["YouOnlyLiveOnce", "y"];
  let error_test_config = ErrorTestConfig::source_with_input_directory_not_exists(test_directory, input, error_parts);

  run_error_test(error_test_config)
}


#[test]
fn error_message_on_target_dir_exists() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "target-dir-exists";

  let target_directory = tempdir()?;
  let target_pathbuf = target_directory.into_path();
  let target_string = target_pathbuf.to_string_lossy().to_string();

  let error_parts =
    ErrorParts::new(
      "Got a configuration error".to_owned(),
      s!("The target directory '{}' should not exist. It will be created when Zat processes the template files.", &target_string),
      "Please supply an empty directory for the target.".to_owned(),
    );

  let error_test_config = ErrorTestConfig::source_with_target_directory_not_exists(test_directory, &target_string,error_parts);

 run_error_test(error_test_config)
}


#[test]
fn error_message_on_no_variables_defined() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "no-variables-defined";
  let source_directory = s!("./tests/errors/{}/source", test_directory);

  let error_parts =
    ErrorParts::new(
      "Got a error processing variables".to_owned(),
      s!("Variable file '{}/.variables.zat-prompt' does not define any variables. The purpose of Zat is to provide a templating tool to customise frequently used file structures. It does this by replacing variables defined in the file '{}/.variables.zat-prompt' on file and directory names of templates as well as within '.tmpl' files. If you want to simply copy a file structure use 'cp' instead.", source_directory, source_directory),
      s!("Please define at least one variable in the variable file '{}/.variables.zat-prompt'.", source_directory),
    );

  let error_test_config = ErrorTestConfig::run_template_without_input(test_directory, error_parts);

  run_error_test(error_test_config)
}

#[test]
fn error_message_on_binary_template() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "binary-template-file";
  let source_directory = s!("./tests/errors/{}/source", test_directory);

  let error_parts =
    ErrorParts::with_exception(
      "There was an error running the template".to_owned(),
      s!("Could not decode ReasonFileErrorReason::template file '{}/template/one.zip.tmpl' content to a string. Only text file templates are supported.", source_directory),
      "invalid utf-8 sequence of 1 bytes from index 14".to_owned(),
      s!("Ensure the template file '{}/template/one.zip.tmpl' is a text file and is not corrupted.", source_directory),
    );

  let input = &["YouOnlyLiveOnce", "y"];
  let error_test_config = ErrorTestConfig::run_template(test_directory, input, error_parts);

  run_error_test(error_test_config)
}

#[test]
fn error_message_on_shell_hook_returning_non_zero_result() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "non-zero-post-processing-shell-hook-result";
  let source_directory = s!("./tests/errors/{}/source", test_directory);

  let error_parts =
    ErrorParts::new(
      "There was an error running the post processor".to_owned(),
      s!("Shell hook `{}/shell-hook.zat-exec` failed with status code 1. The shell hook failed with a non-zero error code signifying an error.", source_directory),
      s!("Please check the logs above for why the shell hook failed. Try running the shell hook file `{}/shell-hook.zat-exec` manually by itself on the output to iterate on the error.", source_directory),
    );

  let input = &["YouOnlyLiveOnce", "y"];
  let error_test_config = ErrorTestConfig::run_template(test_directory, input, error_parts);

  run_error_test(error_test_config)
}

#[test]
fn error_message_on_shell_hook_not_executable() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "post-processing-shell-hook-not-executable";
  let source_directory = s!("./tests/errors/{}/source", test_directory);

  let error_parts =
    ErrorParts::with_exception(
      "There was an error running the post processor".to_owned(),
      s!("Shell hook `{}/shell-hook.zat-exec` failed with an error.", source_directory),
      "Permission denied (os error 13)".to_owned(),
      s!("Please ensure the shell hook file `{}/shell-hook.zat-exec` exists and is executable.", source_directory),
    );

  let input = &["YouOnlyLiveOnce", "y"];
  let error_test_config = ErrorTestConfig::run_template(test_directory, input, error_parts);

  run_error_test(error_test_config)
}

//----------------------------------------------------------------------------------------------------------------------
// Helper Classes
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone)]
struct ErrorParts(String, String, Option<String>, String);


struct ErrorTestConfig<'a> {
  test_directory: &'a str,
  maybe_input: Option<&'a[&'a str]>,
  maybe_target_directory: Option<&'a str>,
  target_directory_should_exist: bool,
  error_parts: ErrorParts
}

impl ErrorParts {
  fn new(error_type: String, error: String, fix: String) -> Self {
    ErrorParts(
      error_type,
      error,
      None,
      fix
    )
  }

  fn with_exception(error_type: String, error: String, exception: String, fix: String) -> Self {
    ErrorParts(
      error_type,
      error,
      Some(exception),
      fix
    )
  }
}

impl <'a> ErrorTestConfig<'a> {

  /// Source error test, without input and without a target directory getting created.
  fn source_no_input_directory_not_exists(test_directory: &'a str, error_parts: ErrorParts) -> Self {

  let maybe_input = None;
  let maybe_target_directory = None;
  let target_directory_should_exist = false;

    Self {
      test_directory,
      maybe_input,
      maybe_target_directory,
      target_directory_should_exist,
      error_parts
    }
  }

  /// Source error test, with input and without a target directory getting created.
  fn source_with_input_directory_not_exists(test_directory: &'a str, input: &'a[&'a str], error_parts: ErrorParts) -> Self {
    let maybe_input = Some(input);
    let maybe_target_directory = None;
    let target_directory_should_exist = false;

      Self {
        test_directory,
        maybe_input,
        maybe_target_directory,
        target_directory_should_exist,
        error_parts
      }
  }

  /// Source error test, ensuring a specific target directory does not exist
  fn source_with_target_directory_not_exists(test_directory: &'a str, target_directory: &'a str, error_parts: ErrorParts) -> Self {
    let maybe_input = None;
    let maybe_target_directory = Some(target_directory);
    let target_directory_should_exist = false;

      Self {
        test_directory,
        maybe_input,
        maybe_target_directory,
        target_directory_should_exist,
        error_parts
      }
  }

  /// Source error test, with input and with a specific a target directory, which may or may not exist.
  #[allow(dead_code)]
  fn source_with_input_and_target_directory(test_directory: &'a str, input: &'a[&'a str], target_directory: &'a str, target_directory_should_exist: bool, error_parts: ErrorParts) -> Self {
    let maybe_input = Some(input);
    let maybe_target_directory = Some(target_directory);

      Self {
        test_directory,
        maybe_input,
        maybe_target_directory,
        target_directory_should_exist,
        error_parts
      }
  }

  /// Source error test, with input and where the target directory should exist.
  fn run_template(test_directory: &'a str, input: &'a[&'a str], error_parts: ErrorParts) -> Self {
    let maybe_input = Some(input);
    let maybe_target_directory = None;
    let target_directory_should_exist = true;

      Self {
        test_directory,
        maybe_input,
        maybe_target_directory,
        target_directory_should_exist,
        error_parts
      }
  }

  fn run_template_without_input(test_directory: &'a str, error_parts: ErrorParts) -> Self {
    let maybe_input = None;
    let maybe_target_directory = None;
    let target_directory_should_exist = false;

      Self {
        test_directory,
        maybe_input,
        maybe_target_directory,
        target_directory_should_exist,
        error_parts
      }
  }
}


//----------------------------------------------------------------------------------------------------------------------
// Helper functions
//----------------------------------------------------------------------------------------------------------------------

fn run_error_test(error_config: ErrorTestConfig<'_>) -> Result<(), Box<dyn std::error::Error>> {

  let source_directory = s!("./tests/errors/{}/source", error_config.test_directory);

  let mut cmd = Command::cargo_bin("zat").unwrap();
  let working_directory = tempdir()?;

  let target_directory = match error_config.maybe_target_directory {
    Some(target_dir) => target_dir.to_owned(),
    None => working_directory.into_path().join(s!("errors-{}", error_config.test_directory)).to_string_lossy().to_string(),
  };

  let error = error_config.error_parts;

  let std_err_contains = |error: ErrorParts| {
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      let lines: Vec<&str> = output.split('\n').collect();
      assert_error_message(&lines, error.clone())
    })
  };

  let command =
    cmd
      .arg("--template-dir")
      .arg(source_directory)
      .arg("--target-dir")
      .arg(&target_directory);

  if let Some(input) = error_config.maybe_input {
    command.write_stdin(stdin(input));
  }

  command
    .assert()
    .failure()
    .stderr(std_err_contains(error));

  if error_config.target_directory_should_exist {
    assert!(std::path::Path::new(&target_directory).exists());
    println!("Target dir {} should not have been created", &target_directory);
  }

  Ok(())
}

/// Assert each line of stderror.
///
/// Without an exception:
/// line0: "Zat failed an with error."
/// line1: Blank
/// line2: <Error Category>:
/// line3: <Error>
/// line4: Blank
/// line5: "Possible fix:"
/// line6: <Fix>
/// line7: Blank
///
/// With an exception:
/// line0: "Zat failed an with error."
/// line1: Blank
/// line2: <Error Category>:
/// line3: <Error>
/// line4: Blank
/// line5: Exception:
/// line6: <Exception Message>
/// line7: Blank
/// line8: "Possible fix:"
/// line9: <Fix>
/// line10: Blank
fn assert_error_message(lines: &[&str], error_parts: ErrorParts) -> bool {

  let error_colour = ansi_term::Color::Red;
  let heading_colour = ansi_term::Color::Yellow;
  let heading_indent = "  ";
  let content_indent = "    ";
  let ErrorParts(error_type, error, maybe_exception, fix) = error_parts;

  let num_lines = lines.len();

  println!("Received lines:");
  for (index, line) in lines.iter().enumerate() {
    println!("{}. {}", index, line);
  }

  if let Some(exception) = maybe_exception {
    assert_eq!(num_lines, 11, "expected 11 lines but got {}", num_lines);

    let line0 = assert_line(0, lines[0], error_colour.paint("Zat failed an with error.").to_string().as_str());
    let line1 = assert_line(1, lines[1], "",);

    let line2 = assert_line(2, lines[2], s!("{}{}:", heading_indent, heading_colour.paint(error_type).to_string()).as_str());
    let line3 = assert_line(3, lines[3], s!("{}{}", content_indent, error).as_str());
    let line4 = assert_line(4, lines[4], "");

    let line5 = assert_line(5, lines[5], s!("{}{}:", heading_indent, heading_colour.paint("Exception").to_string()).as_str());
    let line6 = assert_line(6, lines[6], s!("{}{}", content_indent, exception).as_str());
    let line7 = assert_line(7, lines[7], "");


    let line8 = assert_line(8, lines[8], s!("{}{}:", heading_indent, heading_colour.paint("Possible fix").to_string()).as_str());
    let line9 = assert_line(9, lines[9], s!("{}{}", content_indent, fix).as_str());
    let line10 = assert_line(10, lines[10], "");

    line0 &&
    line1 &&
    line2 &&
    line3 &&
    line4 &&
    line5 &&
    line6 &&
    line7 &&
    line8 &&
    line9 &&
    line10
  } else {
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

}

fn assert_line(
  _number: u8, actual: &str, expected: &str) -> bool {
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