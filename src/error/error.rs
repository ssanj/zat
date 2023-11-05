use std::{fmt::{self, write}, format as s};

use ansi_term::Color::Yellow;

pub type ZatResult<A> = Result<A, ZatError>;
pub type ZatAction = Result<(), ZatError>;


#[derive(Debug, PartialEq)]
pub enum ZatError {
  UserConfigError(UserConfigErrorReason),
  VariableFileNotFound(String),
  VariableOpenError(String),
  VariableReadError(String),
  VariableDecodeError(String),
  ReadingFileError(String),
  WritingFileError(String),
  NoFilesToProcessError(String),
  ProcessingErrors(Vec<ZatError>),
  PostProcessingError(String),
}

impl ZatError {
  fn print_error_fix(error: &str, fix: &str) -> String {
    let indent = "    ";
    let heading_indent = "  ";
    let heading = Yellow.paint("Possible fix:");
    let error_indent = s!("{}", indent);
    let fix_indent = s!("{}", indent);

    s!("{}{}\n\n{}{}\n{}{}", error_indent, error, heading_indent, heading, fix_indent, fix)
  }

  fn print_error<D: fmt::Display>(prefix: &str, error: D) -> String {
    let indent = "  ";
    s!("\n\n{}{}\n{}", indent, Yellow.paint(prefix), error)
  }
}

#[derive(Debug, PartialEq)]
pub enum UserConfigErrorReason {
  TemplateDirDoesNotExist(String, String),
  TemplateFilesDirDoesNotExist(String, String),
  TargetDirectoryShouldNotExist(String, String),
}

impl fmt::Display for UserConfigErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        UserConfigErrorReason::TemplateDirDoesNotExist(error, fix) => write!(f, "{}", ZatError::print_error_fix(&error, &fix)),
        UserConfigErrorReason::TemplateFilesDirDoesNotExist(error, fix) =>  write!(f, "{}", ZatError::print_error_fix(&error, &fix)),
        UserConfigErrorReason::TargetDirectoryShouldNotExist(error, fix) => write!(f, "{}", ZatError::print_error_fix(&error, &fix)),
      }
    }
}

impl ZatError {
  pub fn template_dir_does_not_exist(path: &str) -> ZatError {
    ZatError::UserConfigError(
      UserConfigErrorReason::TemplateDirDoesNotExist(
        s!("The Zat template directory '{}' does not exist. It should exist so Zat can read the templates configuration.", path),
        s!("Please create the Zat template directory '{}' with the Zat template folder structure. See `zat -h` for more.", path)
      )
    )
  }

  pub fn template_files_dir_does_not_exist(path: &str) -> ZatError {
    ZatError::UserConfigError(
      UserConfigErrorReason::TemplateFilesDirDoesNotExist(
        s!("The Zat template files directory '{}' does not exist. It should exist so Zat can read the template files.", path),
        s!("Please create the Zat template files directory '{}' with the necessary template files. See `zat -h` for more details.", path)
      )
    )
  }

  pub fn target_dir_should_not_exist(path: &str) -> ZatError {
    ZatError::UserConfigError(
      UserConfigErrorReason::TargetDirectoryShouldNotExist(
        s!("The target directory '{}' should not exist. It will be created when Zat processes the template files.", path),
        s!("Please supply an empty directory for the target.")
      )
    )
  }
}


impl std::fmt::Display for ZatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let string_rep = match self {
        ZatError::UserConfigError(error)        => ZatError::print_error("Got a configuration error:", error),
        ZatError::VariableFileNotFound(error)   => s!("Could not find variables file:\n    {}", error),
        ZatError::VariableOpenError(error)      => s!("Got an error opening variable file:\n    {}", error),
        ZatError::VariableReadError(error)      => s!("Got an error reading variable file:\n    {}", error),
        ZatError::VariableDecodeError(error)    => s!("Got an error decoding variable file:\n    {}", error),
        ZatError::ReadingFileError(error)       => s!("Could not read template file:\n    {}", error),
        ZatError::WritingFileError(error)       => s!("Could not write destination file:\n    {}", error),
        ZatError::NoFilesToProcessError(path)   => s!("Could not find any files to process at {}.", path),
        ZatError::PostProcessingError(error)    => s!("There was an error running the post processor {}.", error),
        ZatError::ProcessingErrors(errors)      => {
          let error_str =
            errors
              .into_iter()
              .map(|e| format!("{}", e))
              .collect::<Vec<_>>()
              .join("\n    - ");

          format!("Got an error processing template files: {}", error_str)
        },
      };

      write!(f, "{}", string_rep)
    }
}
