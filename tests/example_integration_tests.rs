use assert_cmd::Command;
use tempfile::tempdir;

use crate::file_differ::print_changes;
use crate::file_differ::list_directory_contents;
use predicates::prelude::*;
use std::{format as s, println as p};
use std::path::Path;
use ansi_term::Color::Red;

mod file_differ;

#[test]
fn returns_version() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();

  let version = env!("CARGO_PKG_VERSION");
  let expected_version_string = s!("zat {}\n", version);

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


// To test if a folder that matches one of the default ignores (.variables.zat-prompt, .git) fails.
// It should only use ignores "within" a the repository's template directory, not "on"  the repository directory path.
#[test]
fn runs_a_template_with_github_in_folder_name() -> Result<(), Box<dyn std::error::Error>> {
  let example_test_config =
    ExampleTestConfig::with_input(
      "with.github.com.in.folder.name",
      &["YouOnlyLiveOnce", "", "y"]
    );

  assert_run_example(example_test_config)
}


#[test]
fn runs_a_simple_template_with_shell_hook() -> Result<(), Box<dyn std::error::Error>> {
  let working_directory = tempdir()?;
  let target_directory = working_directory.into_path().join("example-simple-with-shell-hook");

  let args_string = s!("shell hook received args: {}", target_directory.to_string_lossy());
  let shell_output = "running shell hook";
  let shell_assertions = [shell_output, &args_string];

  let example_test_config =
    ExampleTestConfig::with_expected_output_and_files(
      "simple-with-shell-hook",
      &["Something Cool", "", "y"],
      AssertionType::Contains(&shell_assertions),
      target_directory.as_path(),
    );

  assert_run_example(example_test_config)
}


#[test]
fn runs_a_simple_template_with_plugin() -> Result<(), Box<dyn std::error::Error>> {
  let example_test_config =
    ExampleTestConfig::with_input(
      "simple-with-plugin",
      &["YouOnlyLiveOnce", "", "y"]
    );

  assert_run_example(example_test_config)
}


#[test]
fn runs_a_simple_template_with_choices() -> Result<(), Box<dyn std::error::Error>> {
  let example_test_config =
    ExampleTestConfig::with_input(
      "simple-with-choices",
      &["2", "YouOnlyLiveOnce", "y"]
    );

  assert_run_example(example_test_config)
}


#[test]
fn runs_a_simple_template_with_scopes() -> Result<(), Box<dyn std::error::Error>> {
  let example_test_config =
    ExampleTestConfig::with_input(
      "simple-with-scopes",
      &["2", "YouOnlyLiveOnce", "Your summary", "y"]
    );

  assert_run_example(example_test_config)
}


#[test]
fn runs_template_with_ignores() -> Result<(), Box<dyn std::error::Error>> {
  let working_directory = tempdir()?;
  let target_directory = working_directory.into_path().join("example-template-with-ignores");

  let ignores = ["^do-not-include.txt", "^wip/.*.txt"];
  let output = ["Zat completed successfully"];

  let unexpected_files =
    [
      Path::new("do-not-include.txt"),
      &Path::new("wip").join("sample.txt"),
      &Path::new("wip").join("scratch.txt")
    ];

  let example_test_config =
    ExampleTestConfig::with_ignores_and_expected_output_and_files(
      "template-with-ignores",
      &["YouOnlyLiveOnce", "", "y"],
      AssertionType::Contains(&output),
      target_directory.as_path(),
      &ignores,
      &unexpected_files
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


#[test]
fn runs_the_bootstrap_template() -> Result<(), Box<dyn std::error::Error>> {

  let variable_file = Path::new(".variables.zat-prompt");
  let readme_file = Path::new("template").join("README.md.tmpl");
  let config_file = Path::new("template").join("$project__underscore$_config.conf");

  let files_that_should_exist =
    [
      variable_file,
      readme_file.as_path(),
      config_file.as_path()
    ];

    let working_directory_path = tempdir()?.into_path();
    let repository_directory =
      &working_directory_path.join("example-bootstrap-dir").to_string_lossy().to_string();

  let output_message_1 = s!("Zat created a bootstrap repository at `{}`.", &repository_directory);

  let output_message_2 = s!("Process the bootstrap repository with: `zat process --repository-dir {} --target-dir <YOUR_TARGET_DIRECTORY>`", &repository_directory);


  let output_messages =
    [
      output_message_1.as_str(),
      output_message_2.as_str(),
      "Zat completed successfully.",
    ];

  let bootstrap_test_config =
    BootstrapExampleTestConfig::new(repository_directory, &files_that_should_exist, &output_messages);

  assert_run_bootstrap_example(bootstrap_test_config)
}

//----------------------------------------------------------------------------------------------------------------------
// Helper classes
//----------------------------------------------------------------------------------------------------------------------

#[allow(dead_code)]
enum AssertionType<'a> {
  Contains(&'a[&'a str]),
}

struct ExampleTestConfig<'a> {
  test_directory: &'a str,
  maybe_input: Option<&'a[&'a str]>,
  maybe_target_directory: Option<&'a Path>,
  maybe_stdout_assertions: Option<AssertionType<'a>>,
  files_that_should_not_exist: &'a[&'a Path],
  ignores: &'a[&'a str]
}

impl <'a> ExampleTestConfig<'a> {
  fn with_input(test_directory: &'a str, input: &'a[&'a str]) -> Self {

    let maybe_input = Some(input);
    let maybe_stdout_assertions = None;
    let maybe_target_directory = None;
    let files_that_should_not_exist = &[];
    let ignores = &[];

    Self {
      test_directory,
      maybe_input,
      maybe_target_directory,
      maybe_stdout_assertions,
      files_that_should_not_exist,
      ignores
    }
  }

  fn with_expected_output_and_files(test_directory: &'a str, input: &'a[&'a str], expected_output: AssertionType<'a>, target_dir: &'a Path) -> Self {

    let maybe_input = Some(input);
    let maybe_target_directory = Some(target_dir);
    let maybe_stdout_assertions = Some(expected_output);
    let files_that_should_not_exist = &[];
    let ignores = &[];

    Self {
      test_directory,
      maybe_input,
      maybe_target_directory,
      maybe_stdout_assertions,
      files_that_should_not_exist,
      ignores
    }
  }

  fn with_ignores_and_expected_output_and_files(test_directory: &'a str, input: &'a[&'a str], expected_output: AssertionType<'a>, target_dir: &'a Path, ignores: &'a[&'a str], files_that_should_not_exist: &'a[&'a Path]) -> Self {

    let maybe_input = Some(input);
    let maybe_target_directory = Some(target_dir);
    let maybe_stdout_assertions = Some(expected_output);

    Self {
      test_directory,
      maybe_input,
      maybe_target_directory,
      maybe_stdout_assertions,
      files_that_should_not_exist,
      ignores
    }
  }
}

struct BootstrapExampleTestConfig<'a> {
  repository_directory: &'a str,
  files_that_should_exist: &'a[&'a Path],
  maybe_stdout_assertions: Option<AssertionType<'a>>,
}

impl <'a> BootstrapExampleTestConfig<'a> {

  fn new(repository: &'a str, files_that_should_exist: &'a [&'a Path], output_messages: &'a[&'a str]) -> Self {
    BootstrapExampleTestConfig {
      repository_directory: repository,
      files_that_should_exist,
      maybe_stdout_assertions: Some(AssertionType::Contains(output_messages))
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

  p!("target directory: {}", &target_directory);

  let std_out_contains = |expected:&str| {
    let owned_expected = expected.to_owned();
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      p!("Could not validate stdout contains: {}", &owned_expected);
      output.contains(&owned_expected)
    })
  };


  assert!(Path::new(&source_directory).exists(), "{}", Red.paint(s!("Source directory `{}` does not exist: ", &source_directory)));

  cmd
    .arg("process")
    .arg("--repository-dir")
    .arg(&source_directory)
    .arg("--target-dir")
    .arg(&target_directory);

    for ig in example_config.ignores {
      cmd
        .arg("--ignores")
        .arg(ig);
    }

    if let Some(input) = example_config.maybe_input {
      cmd.write_stdin(stdin(input));
    }

  let mut output =
    cmd
      .assert()
      .success();

  if let Some(AssertionType::Contains(contents)) = example_config.maybe_stdout_assertions {
    for content in contents {
      output = output.stdout(std_out_contains(content));
    }
  }

  assert!(Path::new(&target_directory).exists(), "{}", Red.paint(s!("target directory `{}` does not exist", &target_directory)));

  // Files that should not exist; might be ignore or deleted via a shell hook
  for unexpected_file in example_config.files_that_should_not_exist {
    let fullpath = Path::new(&target_directory).join(unexpected_file);
    assert!(!fullpath.exists(), "{}", Red.paint(s!("Unexpected file `{}` exists", fullpath.to_string_lossy())));
  }

  println!("Target Directory {} contents", &target_directory);
  println!();
  for f in list_directory_contents(&target_directory) {
    println!("{}", f);
  }

  println!("Expected Target {} Directory contents", &expected_target_directory);
  println!();
  for f in list_directory_contents(&expected_target_directory) {
    println!("{}", f);
  }

  print_changes(&expected_target_directory, &target_directory);

  assert!(!dir_diff::is_different(&target_directory, expected_target_directory).unwrap());

  Ok(())
}

fn assert_run_bootstrap_example(bootstrap_example_config: BootstrapExampleTestConfig) -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();

  let working_directory_path = bootstrap_example_config.repository_directory;
  p!("repository directory: {}", &working_directory_path);

  let std_out_contains = |expected:&str| {
    let owned_expected = expected.to_owned();
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      p!("Could not validate stdout contains: {}", &owned_expected);
      output.contains(&owned_expected)
    })
  };

  cmd
    .arg("bootstrap")
    .arg("--repository-dir")
    .arg(working_directory_path);

  let mut output =
    cmd
      .assert()
      .success();


  for expected_file in bootstrap_example_config.files_that_should_exist {
    let file = Path::new(&working_directory_path).join(expected_file);
    assert!(file.exists(), "{}", Red.paint(s!("Expected file `{}` does not exist: ", file.to_string_lossy())));
  }

  if let Some(AssertionType::Contains(contents)) = bootstrap_example_config.maybe_stdout_assertions {
    for content in contents {
      output = output.stdout(std_out_contains(content));
    }
  }

  Ok(())
}


fn stdin(responses: &[&str]) -> String {
  let delimited =
    responses
      .join("\n");

  s!("{}\n", delimited) // add the extra newline for complete the final answer
}
