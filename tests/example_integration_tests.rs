use assert_cmd::Command;
use tempfile::tempdir;

use crate::file_differ::print_changes;
use predicates::prelude::*;
use std::{format as s};
use std::path::Path;

mod file_differ;

#[test]
fn returns_version() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();

  let version = env!("CARGO_PKG_VERSION");
  let expected_version_string = format!("zat {}\n", version);

  cmd
    .arg("-V")
    .assert()
    .success()
    .stdout(expected_version_string);

  Ok(())
}


#[test]
fn runs_a_simple_template() -> Result<(), Box<dyn std::error::Error>> {
  let example_test_config =
    ExampleTestConfig::with_input(
      "simple",
      &["YouOnlyLiveOnce", "", "y"]
    );

  assert_run_example(example_test_config)
}


#[test]
fn runs_a_simple_template_with_shell_hook() -> Result<(), Box<dyn std::error::Error>> {
  let working_directory = tempdir()?;
  let target_directory = working_directory.into_path().join("example-simple-with-shell-hook");

  let args_string = format!("shell hook received args: {}", target_directory.to_string_lossy());
  let shell_output = "running shell hook";
  let shell_assertions = [shell_output, &args_string];

  let shell_created_dir = target_directory.join("created-by-shell-hook");
  let shell_created_dir_path = shell_created_dir.as_path();
  let expected_files = [shell_created_dir_path];

  let example_test_config =
    ExampleTestConfig::with_expected_output_and_files(
      "simple-with-shell-hook",
      &["Something Cool", "", "y"],
      AssertionType::Contains(&shell_assertions),
      target_directory.as_path(),
      &expected_files
    );

  assert_run_example(example_test_config)
}

#[test]
fn runs_a_sublime_plugin_template() -> Result<(), Box<dyn std::error::Error>> {
  let example_test_config =
    ExampleTestConfig::with_input(
      "sublime-plugin",
      &["HelloWorld", "Says Hello", "y"],
    );

  assert_run_example(example_test_config)
}


#[test]
fn runs_a_template_with_binary_files() -> Result<(), Box<dyn std::error::Error>> {
  let example_test_config =
    ExampleTestConfig::with_input(
      "binary-files",
      &["YouOnlyLiveOnce", "y"],
    );

  assert_run_example(example_test_config)
}

//----------------------------------------------------------------------------------------------------------------------
// Helper classes
//----------------------------------------------------------------------------------------------------------------------

enum AssertionType<'a> {
  Equals(&'a str),
  Contains(&'a[&'a str]),
}

struct ExampleTestConfig<'a> {
  test_directory: &'a str,
  maybe_input: Option<&'a[&'a str]>,
  maybe_target_directory: Option<&'a Path>,
  maybe_stdout_assertions: Option<AssertionType<'a>>,
  files_that_should_exist: &'a[&'a Path],
  files_that_should_not_exist: &'a[&'a Path],
}

impl <'a> ExampleTestConfig<'a> {
  fn with_input(test_directory: &'a str, input: &'a[&'a str]) -> Self {

    let maybe_input = Some(input);
    let maybe_stdout_assertions = None;
    let maybe_target_directory = None;
    let files_that_should_exist = &[];
    let files_that_should_not_exist = &[];

    Self {
      test_directory,
      maybe_input,
      maybe_target_directory,
      maybe_stdout_assertions,
      files_that_should_exist,
      files_that_should_not_exist,
    }
  }

  fn with_expected_output_and_files(test_directory: &'a str, input: &'a[&'a str], expected_output: AssertionType<'a>, target_dir: &'a Path, files_that_should_exist: &'a[&'a Path]) -> Self {

    let maybe_input = Some(input);
    let maybe_target_directory = Some(target_dir);
    let maybe_stdout_assertions = Some(expected_output);
    let files_that_should_not_exist = &[];

    Self {
      test_directory,
      maybe_input,
      maybe_target_directory,
      maybe_stdout_assertions,
      files_that_should_exist,
      files_that_should_not_exist,
    }
  }
}

//----------------------------------------------------------------------------------------------------------------------
// Helper functions
//----------------------------------------------------------------------------------------------------------------------

fn assert_run_example(example_config: ExampleTestConfig) -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();

  let source_directory = s!("./tests/examples/{}/source", example_config.test_directory);
  let expected_target_directory = s!("./tests/examples/{}/destination", example_config.test_directory);

  let target_directory = match example_config.maybe_target_directory {
    Some(td) => td.to_string_lossy().to_string(),
    None => {
      let working_directory = tempdir()?;
      working_directory.into_path().join(s!("example-{}", example_config.test_directory)).to_string_lossy().to_string()
    },
  };

  println!("target directory: {}", &target_directory);

  let std_out_contains = |expected:&str| {
    let owned_expected = expected.to_owned();
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      println!("Could not validate stdout contains: {}", &owned_expected);
      output.contains(&owned_expected)
    })
  };


  assert!(Path::new(&source_directory).exists(), "Source directory `{}` does not exist: ", &source_directory);

  cmd
    .arg("--template-dir")
    .arg(&source_directory)
    .arg("--target-dir")
    .arg(&target_directory);

    if let Some(input) = example_config.maybe_input {
      cmd.write_stdin(stdin(input));
    }

  let mut output =
    cmd
      .assert()
      .success();

  match example_config.maybe_stdout_assertions {
      Some(AssertionType::Equals(content)) => {
        println!("stdout did not equal: {}", &content);
        output.stdout(content.to_owned());
      },

      Some(AssertionType::Contains(contents)) => {
        for content in contents {
          output = output.stdout(std_out_contains(content));
        }
      },

      None => ()
  }

  assert!(Path::new(&target_directory).exists(), "target directory `{}` does not exist", &target_directory);
  for expected_file in example_config.files_that_should_exist {
    assert!(Path::new(expected_file).exists(), "Expected file `{}` does not exist: ", &expected_file.to_string_lossy());
  }

  for unexpected_file in example_config.files_that_should_not_exist {
    assert!(!Path::new(unexpected_file).exists(), "Unexpected file `{}` exists", &unexpected_file.to_string_lossy());
  }

  print_changes(&expected_target_directory, &target_directory);

  assert!(!dir_diff::is_different(&target_directory, expected_target_directory).unwrap());

  Ok(())
}


fn stdin(responses: &[&str]) -> String {
  let delimited =
    responses
      .join("\n");

  format!("{}\n", delimited) // add the extra newline for complete the final answer
}
