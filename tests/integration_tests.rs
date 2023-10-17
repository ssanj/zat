use assert_cmd::Command;
use tempfile::tempdir;

use crate::file_differ::print_changes;
use predicates::prelude::*;

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
  let mut cmd = Command::cargo_bin("zat").unwrap();
  let working_directory = tempdir()?;
  let target_directory = working_directory.into_path().join("simple-template").to_string_lossy().to_string();
  let expected_target_directory = "./tests/examples/simple/destination";
  println!("target directory: {}", &target_directory);

  cmd
    .arg("--template-dir")
    .arg("./tests/examples/simple/source")
    .arg("--target-dir")
    .arg(&target_directory)
    .write_stdin(stdin(&["YouOnlyLiveOnce", "", "y"]))
    .assert()
    .success();

  assert!(std::path::Path::new(&target_directory).exists());

  print_changes(&expected_target_directory, &target_directory);

  assert!(!dir_diff::is_different(&target_directory, expected_target_directory).unwrap());

  Ok(())
}

#[test]
fn runs_a_simple_template_with_shell_hook() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();
  let working_directory = tempdir()?;
  let target_directory = working_directory.into_path().join("simple-with-shell-hook").to_string_lossy().to_string();
  let expected_target_directory = "./tests/examples/simple-with-shell-hook/destination";
  println!("target directory: {}", &target_directory);

  let std_out_contains = |expected:&str| {
    let owned_expected = expected.to_owned();
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      output.contains(&owned_expected)
    })
  };

  let args_string = format!("shell hook received args: {}", target_directory);
  cmd
    .arg("--template-dir")
    .arg("./tests/examples/simple-with-shell-hook/source")
    .arg("--target-dir")
    .arg(&target_directory)
    .write_stdin(stdin(&["Something Cool", "", "y"]))
    .assert()
    .success()
    .stdout(std_out_contains(&args_string))
    .stdout(std_out_contains("running shell hook"));

  assert!(std::path::Path::new(&target_directory).exists());
  assert!(std::path::Path::new(&target_directory).join("created-by-shell-hook").exists());

  print_changes(&expected_target_directory, &target_directory);

  assert!(!dir_diff::is_different(&target_directory, expected_target_directory).unwrap());

  Ok(())
}

#[test]
fn runs_a_sublime_plugin_template() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("zat").unwrap();
  let working_directory = tempdir()?;
  let target_directory = working_directory.into_path().join("sublime-plugin-template").to_string_lossy().to_string();
  let expected_target_directory = "./tests/examples/sublime-plugin/destination";
  let template_directory = "./tests/examples/sublime-plugin/source";
  println!("target directory: {}", &target_directory);

  cmd
    .arg("--template-dir")
    .arg(&template_directory)
    .arg("--target-dir")
    .arg(&target_directory)
    .write_stdin(stdin(&["HelloWorld", "Says Hello", "y"]))
    .assert()
    .success();

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
