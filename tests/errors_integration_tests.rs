use assert_cmd::Command;
use file_differ::print_diff;
use tempfile::tempdir;
use predicates::prelude::*;
use format as s;
use std::println;

mod file_differ;

#[test]
fn error_message_on_missing_template_dir() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "no-template-dir";
  let source_directory = get_source_directory(test_directory);

  let error_parts =
    ErrorParts::new(
      "Got a configuration error".to_owned(),
      s!("The Zat repository directory '{}' does not exist. It should exist so Zat can read the template configuration.", source_directory),
      s!("Please create the Zat repository directory '{}' with the Zat folder structure. See `zat --help` for more.", source_directory),
    );

  let error_test_config = ErrorTestConfig::source_no_input_directory_not_exists(test_directory, error_parts);

  run_error_test(error_test_config)
}

#[test]
fn error_message_on_missing_template_files_dir() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "no-template-files-dir";
  let source_directory = get_source_directory(test_directory);

  let error_parts =
    ErrorParts::new(
      "Got a configuration error".to_owned(),
      s!("The Zat template files directory '{}/template' does not exist. It should exist so Zat can read the template files.", source_directory),
      s!("Please create the Zat template files directory '{}/template' with the necessary template files. See `zat --help` for more details.", source_directory),
    );

  let error_test_config = ErrorTestConfig::source_no_input_directory_not_exists(test_directory, error_parts);
  run_error_test(error_test_config)
}

#[test]
fn error_message_on_missing_variables_file() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "no-variables-file";
  let source_directory = get_source_directory(test_directory);

  let error_parts =
    ErrorParts::new(
      "Got an error processing variables".to_owned(),
      s!("Variable file '{}/.variables.zat-prompt' does not exist. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", source_directory),
      s!("Please create the variable file '{}/.variables.zat-prompt' with the required tokens. See `zat --help` for more details.", source_directory),
    );

  let error_test_config = ErrorTestConfig::source_no_input_directory_not_exists(test_directory, error_parts);
  run_error_test(error_test_config)
}

#[test]
fn error_message_on_non_json_variables_file() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "non-json-variables-file";
  let source_directory = get_source_directory(test_directory);

  let error_parts =
    ErrorParts::new(
      "Got an error processing variables".to_owned(),
      s!("Variable file '{}/.variables.zat-prompt' could not be decoded as JSON into the expected format. It failed decoding with this error: invalid type: integer `123`, expected a sequence at line 1 column 3. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", source_directory),
      s!("Make the variable file '{}/.variables.zat-prompt' is a valid JSON file in the format required by Zat. See `zat --help` for more details on the format", source_directory),
    );

  let error_test_config = ErrorTestConfig::source_no_input_directory_not_exists(test_directory, error_parts);
  run_error_test(error_test_config)
}

#[test]
fn error_message_on_no_template_files() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "no-template-files";
  let source_directory = get_source_directory(test_directory);

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
  let source_directory = get_source_directory(test_directory);

  let error_parts =
    ErrorParts::new(
      "Got an error processing variables".to_owned(),
      s!("Variable file '{}/.variables.zat-prompt' does not define any variables. The purpose of Zat is to provide a templating tool to customise frequently used file structures. It does this by replacing variables defined in the file '{}/.variables.zat-prompt' on file and directory names of templates as well as within '.tmpl' files. If you want to simply copy a file structure use 'cp' instead.", source_directory, source_directory),
      s!("Please define at least one variable in the variable file '{}/.variables.zat-prompt'.", source_directory),
    );

  let error_test_config = ErrorTestConfig::run_template_without_input(test_directory, error_parts);

  run_error_test(error_test_config)
}

#[test]
fn error_message_on_binary_template() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "binary-template-file";
  let source_directory = get_source_directory(test_directory);

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
  let source_directory = get_source_directory(test_directory);
  let temp_dir = tempdir()?;
  let target_dir_path = temp_dir.path().to_owned();
  let target_dir = target_dir_path.to_string_lossy().to_string();

  // delete temp dir so we can use it as the target dir (which should not exist)
  drop(temp_dir);

  let error_parts =
    ErrorParts::new(
      "There was an error running the post processor".to_owned(),
      s!("Shell hook '{}/shell-hook.zat-exec {}' failed with status code 1. The shell hook failed with a non-zero error code signifying an error.", source_directory, target_dir),
      s!("Please check the logs above for why the shell hook failed. Try running the shell hook file '{}/shell-hook.zat-exec' with argument '{}' manually to iterate on the error.", source_directory, target_dir),
    );


  let input = &["YouOnlyLiveOnce", "y"];
  let error_test_config = ErrorTestConfig::source_with_input_and_target_directory(test_directory, input, &target_dir, false, error_parts);

  run_error_test(error_test_config)
}


#[test]
fn error_message_plugin_failure() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "plugin-failure";

  let error_parts =
    ErrorParts::with_exception(
      s!("There was an error running a plugin"),
      s!("Plugin 'failure.sh' returned the following error: Something went wrong."),
      s!("Born to fail"),
      s!("Run the success.sh plugin instead."),
    );

  let error_test_config = ErrorTestConfig::source_no_input_directory_not_exists(test_directory, error_parts);

  run_error_test(error_test_config)
}


#[test]
fn error_message_on_shell_hook_not_executable() -> Result<(), Box<dyn std::error::Error>> {
  let test_directory = "post-processing-shell-hook-not-executable";
  let source_directory = get_source_directory(test_directory);

  let error_parts =
    ErrorParts::with_exception(
      "There was an error running the post processor".to_owned(),
      s!("Shell hook '{}/shell-hook.zat-exec' failed with an error.", source_directory),
      "Permission denied (os error 13)".to_owned(),
      s!("Please ensure the shell hook file '{}/shell-hook.zat-exec' exists and is executable.", source_directory),
    );

  let input = &["YouOnlyLiveOnce", "y"];
  let error_test_config = ErrorTestConfig::run_template(test_directory, input, error_parts);

  run_error_test(error_test_config)
}

#[test]
fn error_message_on_bootstrap_repository_exists() -> Result<(), Box<dyn std::error::Error>> {
  let working_directory = tempdir()?;
  let repository_directory = working_directory.into_path().to_string_lossy().to_string();

  let error_parts =
    ErrorParts::new(
      "There was an error running the bootstrap process".to_owned(),
      s!("The repository directory '{}' should not exist. It will be created by the Zat bootstrap process.", repository_directory.as_str()),
      s!("Please supply an empty directory for the repository.")
    );

  let bootstrap_error_config = BootstrapErrorTestConfig::new(repository_directory.as_str(), error_parts);

  run_bootstrap_error_test(bootstrap_error_config)
}

#[test]
fn error_message_on_invalid_remote_url() -> Result<(), Box<dyn std::error::Error>> {
  let url = "this/is/not/a/url";
  let working_directory = tempdir()?;
  let repository_directory = working_directory.into_path().to_string_lossy().to_string();
  let error_parts =
    ErrorParts::with_exception(
      "There was an error running a remote processing command".to_owned(),
      s!("The remote repository URL supplied '{}' is invalid. Zat needs a valid URL to checkout this repository.", url),
      "relative URL without a base".to_owned(),
      "Please ensure the remote repository URL supplied is valid.".to_owned()
    );

  let process_remote_config = ErrorRemoteTestConfig::source_no_input_directory_not_exists(url, repository_directory.as_str(), error_parts);

  run_remote_error_test(process_remote_config)
}

#[test]
fn error_message_on_unsupported_hostname() -> Result<(), Box<dyn std::error::Error>> {
  let url = "data:text/plain, Stuff";
  let working_directory = tempdir()?;
  let repository_directory = working_directory.into_path().to_string_lossy().to_string();
  let error_parts =
    ErrorParts::new(
      "There was an error running a remote processing command".to_owned(),
      s!("The remote repository URL supplied '{}' has an invalid hostname. Zat needs the hostname to be a domain name or IP address. IPv6 addresses should be supplied within square braces.", url),
      "Please ensure the remote repository URL hostname is a domain or an IP address.".to_owned()
    );

  let process_remote_config = ErrorRemoteTestConfig::source_no_input_directory_not_exists(url, repository_directory.as_str(), error_parts);

  run_remote_error_test(process_remote_config)
}

#[test]
fn error_message_on_git_clone_status_failure() -> Result<(), Box<dyn std::error::Error>> {
  let url = "https://github.com/ssanj/does-not-exist";
  let working_directory = tempdir()?;
  let repository_directory = working_directory.into_path().to_string_lossy().to_string();
  let error_parts =
    ErrorParts::new(
      "There was an error running a remote processing command".to_owned(),
      s!("Zat could not could not clone remote repository '{}' because it returned an exit code of '128'.", url),
      s!("Please ensure the following command runs successfully external to Zat: 'GIT_TERMINAL_PROMPT=0 git clone {}'. It should also not require a password as Zat does not support private repositories that are not accessible through your Git user. Please also see the clone output for possible other issues.", url)
    );

  let process_remote_config = ErrorRemoteTestConfig::source_no_input_directory_not_exists(url, repository_directory.as_str(), error_parts);

  run_remote_error_test(process_remote_config)
}


//----------------------------------------------------------------------------------------------------------------------
// Helper Classes
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone)]
enum AssertionType {
  Equals(String),
  Contains(String),
}

impl AssertionType {
  fn map<F:FnOnce(String) -> String>(self, f: F) -> Self {
    match self {
      AssertionType::Equals(value) => AssertionType::Equals(f(value)),
      AssertionType::Contains(value) => AssertionType::Contains(f(value)),
    }
  }
}


#[derive(Clone)]
struct ErrorParts {
  error_type: AssertionType,
  error: AssertionType,
  maybe_exception: Option<AssertionType>,
  fix: AssertionType
}


struct ErrorTestConfig<'a> {
  test_directory: &'a str,
  maybe_input: Option<&'a[&'a str]>,
  maybe_target_directory: Option<&'a str>,
  target_directory_should_exist: bool,
  error_parts: ErrorParts
}


struct ErrorRemoteTestConfig<'a> {
  url: &'a str,
  test_directory: &'a str,
  maybe_target_directory: Option<&'a str>,
  error_parts: ErrorParts
}


struct BootstrapErrorTestConfig<'a> {
  repository_directory: &'a str,
  error_parts: ErrorParts
}


impl ErrorParts {
  fn new(error_type: String, error: String, fix: String) -> Self {
    ErrorParts {
      error_type: AssertionType::Equals(error_type),
      error: AssertionType::Equals(error),
      maybe_exception: None,
      fix: AssertionType::Equals(fix)
    }
  }

  fn with_exception(error_type: String, error: String, exception: String, fix: String) -> Self {
    ErrorParts {
      error_type: AssertionType::Equals(error_type),
      error: AssertionType::Equals(error),
      maybe_exception: Some(AssertionType::Equals(exception)),
      fix: AssertionType::Equals(fix)
    }
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
        error_parts,
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


impl <'a> BootstrapErrorTestConfig<'a> {

  fn new(repository_directory: &'a str, error_parts: ErrorParts) -> Self {
    Self {
      repository_directory,
      error_parts
    }
  }
}

impl <'a> ErrorRemoteTestConfig<'a> {
  /// Source error test, without input and without a target directory getting created.
  fn source_no_input_directory_not_exists(url: &'a str, test_directory: &'a str, error_parts: ErrorParts) -> Self {
    let maybe_target_directory = None;

      Self {
        url,
        test_directory,
        maybe_target_directory,
        error_parts
      }
  }

}

//----------------------------------------------------------------------------------------------------------------------
// Helper functions
//----------------------------------------------------------------------------------------------------------------------


fn get_source_directory(test_directory: &str) -> String {
  let current_directory = std::env::current_dir().expect("Could not get current directory");
  println!("current directory {}", current_directory.to_string_lossy());

  let source_directory = current_directory.join(s!("tests/errors/{}/source", test_directory));
  println!("source directory {}", source_directory.to_string_lossy());

  source_directory.to_string_lossy().to_string()
}

fn run_error_test(error_config: ErrorTestConfig<'_>) -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();
  let working_directory = tempdir()?;

  let source_directory = get_source_directory(error_config.test_directory);

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
      .arg("process")
      .arg("--repository-dir")
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

fn run_remote_error_test(error_remote_config: ErrorRemoteTestConfig<'_>) -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();
  let working_directory = tempdir()?;

  let target_directory = match error_remote_config.maybe_target_directory {
    Some(target_dir) => target_dir.to_owned(),
    None => working_directory.into_path().join(s!("errors-remote-{}", error_remote_config.test_directory)).to_string_lossy().to_string(),
  };

  let error = error_remote_config.error_parts;

  let std_err_contains = |error: ErrorParts| {
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      let lines: Vec<&str> = output.split('\n').collect();
      assert_error_message(&lines, error.clone())
    })
  };

  let url = error_remote_config.url;

  let command =
    cmd
      .arg("process-remote")
      .arg("--repository-url")
      .arg(url)
      .arg("--target-dir")
      .arg(&target_directory);

  command
    .assert()
    .failure()
    .stderr(std_err_contains(error));

  // if error_remote_config.target_directory_should_exist {
  //   assert!(std::path::Path::new(&target_directory).exists());
  //   println!("Target dir {} should not have been created", &target_directory);
  // }

  Ok(())
}

fn run_bootstrap_error_test(bootstrap_error_config: BootstrapErrorTestConfig<'_>) -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();
  let repository_directory =  bootstrap_error_config.repository_directory;
  let error = bootstrap_error_config.error_parts;

  let std_err_contains = |error: ErrorParts| {
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      let lines: Vec<&str> = output.split('\n').collect();
      assert_error_message(&lines, error.clone())
    })
  };

  let command =
    cmd
      .arg("bootstrap")
      .arg("--repository-dir")
      .arg(repository_directory);


  command
    .assert()
    .failure()
    .stderr(std_err_contains(error));

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
/// line0:
/// line1: "Zat failed an with error."
/// line2: Blank
/// line3: <Error Category>:
/// line4: <Error>
/// line5: Blank
/// line6: Exception:
/// line7: <Exception Message>
/// line8: Blank
/// line9: "Possible fix:"
/// line10: <Fix>
/// line11: Blank
fn assert_error_message(lines: &[&str], error_parts: ErrorParts) -> bool {

  let error_colour = ansi_term::Color::Red;
  let heading_colour = ansi_term::Color::Yellow;
  let heading_indent = "  ";
  let content_indent = "    ";
  let ErrorParts { error_type, error, maybe_exception, fix } = error_parts;

  let num_lines = lines.len();

  println!("Received lines:");
  for (index, line) in lines.iter().enumerate() {
    println!("{}. {}", index, line);
  }

  if let Some(exception) = maybe_exception {
    assert_eq!(num_lines, 12, "expected 12 lines but got {}", num_lines);

    let line0 = assert_line(0, lines[0], AssertionType::Equals("".to_owned()));
    let line1 = assert_line(1, lines[1], AssertionType::Equals(error_colour.paint("Zat failed an with error.").to_string()));

    let line2 = assert_line(2, lines[2], AssertionType::Equals("".to_owned()));
    let line3 = assert_line(3, lines[3], error_type.map(|et| s!("{}{}:", heading_indent, heading_colour.paint(et).to_string())));
    let line4 = assert_line(4, lines[4], error.map(|e| s!("{}{}", content_indent, e)));

    let line5 = assert_line(5, lines[5], AssertionType::Equals("".to_owned()));
    let line6 = assert_line(6, lines[6], AssertionType::Equals(s!("{}{}:", heading_indent, heading_colour.paint("Exception").to_string())));
    let line7 = assert_line(7, lines[7], exception.map(|e| s!("{}{}", content_indent, e)));

    let line8 = assert_line(8, lines[8], AssertionType::Equals("".to_owned()));
    let line9 = assert_line(9, lines[9], AssertionType::Equals(s!("{}{}:", heading_indent, heading_colour.paint("Possible fix").to_string())));
    let line10 = assert_line(10, lines[10], fix.map(|f| s!("{}{}", content_indent, f)));
    let line11 = assert_line(11, lines[11], AssertionType::Equals("".to_owned()));

    line0  &&
    line1  &&
    line2  &&
    line3  &&
    line4  &&
    line5  &&
    line6  &&
    line7  &&
    line8  &&
    line9  &&
    line10 &&
    line11
  } else {

    let first_line =
      if num_lines > 9 {
        // We have unexpected stdout lines.
        // Find the "first" line; the line before : "Zat failed an with error."
        lines
          .iter()
          .position(|line| line.contains("Zat failed an with error."))
          .map_or(0, |n| n - 1)
      } else {
        0
      };

    let final_lines = first_line + 9;

    println!("first line: {}, num lines: {}", first_line, num_lines);

    assert_eq!(num_lines, final_lines, "expected 9 lines but got {}", num_lines);

    let line0 = assert_line(0, lines[first_line], AssertionType::Equals("".to_owned()));
    let line1 = assert_line(1, lines[first_line + 1], AssertionType::Equals(error_colour.paint("Zat failed an with error.").to_string()));

    let line2 = assert_line(2, lines[first_line + 2], AssertionType::Equals("".to_owned()));
    let line3 = assert_line(3, lines[first_line + 3], error_type.map(|et| s!("{}{}:", heading_indent, heading_colour.paint(et).to_string())));
    let line4 = assert_line(4, lines[first_line + 4], error.map(|e| s!("{}{}", content_indent, e)));

    let line5 = assert_line(5, lines[first_line + 5], AssertionType::Equals("".to_owned()));
    let line6 = assert_line(6, lines[first_line + 6], AssertionType::Equals(s!("{}{}:", heading_indent, heading_colour.paint("Possible fix").to_string())));
    let line7 = assert_line(7, lines[first_line + 7], fix.map(|f| s!("{}{}", content_indent, f)));

    let line8 = assert_line(8, lines[first_line + 8], AssertionType::Equals("".to_owned()));

    line0 &&
    line1 &&
    line2 &&
    line3 &&
    line4 &&
    line5 &&
    line6 &&
    line7 &&
    line8
  }
}

fn assert_line(
  _number: u8, actual: &str, assertion_type: AssertionType) -> bool {
  match assertion_type {
    AssertionType::Equals(expected) => {
      if actual != expected {
        print_diff(actual, &expected);
        false
      } else {
        true
      }
    },
    AssertionType::Contains(expected) => {
      if !actual.contains(&expected) {
        print_diff(actual, &expected);
        false
      } else {
        true
      }
    },
}

}


fn stdin(responses: &[&str]) -> String {
  let delimited =
    responses
      .join("\n");

  format!("{}\n", delimited) // add the extra newline for complete the final answer
}
