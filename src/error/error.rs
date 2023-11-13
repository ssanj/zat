use std::{fmt::{self, write}, format as s, unimplemented, todo};

use ansi_term::Color::Yellow;
use clap::error;

pub type ZatResult<A> = Result<A, ZatError>;
pub type ZatAction = Result<(), ZatError>;


#[derive(Debug, PartialEq, Clone)]
pub enum ZatError {
  UserConfigError(UserConfigErrorReason),
  VariableFileError(VariableFileErrorReason),
  TemplateProcessingError(TemplateProcessingErrorReason),
  PostProcessingError(PostProcessingErrorReason),
}

#[derive(Debug, PartialEq, Clone)]
pub enum UserConfigErrorReason {
  TemplateDirDoesNotExist(String, String),
  TemplateFilesDirDoesNotExist(String, String),
  TargetDirectoryShouldNotExist(String, String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableFileErrorReason {
  VariableFileNotFound(String, String),
  VariableOpenError(String, String),
  VariableReadError(String, String),
  VariableDecodeError(String, String),
}


#[derive(Debug, PartialEq, Clone)]
pub enum TemplateProcessingErrorReason {
  NoFilesToProcessError(String, String),
  ReadingFileError(ReasonFileErrorReason),
  WritingFileError(String, Option<String>, String),
  DirectoryCreationError(String, Option<String>, String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReasonFileErrorReason {
  ReadingError(String, Option<String>, String),
  UnsupportedContentError(String, Option<String>, String),
  PrefixError(String, Option<String>, String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PostProcessingErrorReason {
  ExecutionError(String, Option<String>, String),
  NonZeroStatusCode(String, String),
  ProcessInterrupted(String, String),
}

#[derive(Debug)]
struct ErrorFormat {
  error_type: String,
  error_reason: String,
  exception: Option<String>,
  remediation: Option<String>
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


impl From<&UserConfigErrorReason> for ErrorFormat {
  fn from(error: &UserConfigErrorReason) -> Self {

    let (error, fix) = match error {
        UserConfigErrorReason::TemplateDirDoesNotExist(error, fix) => (error, fix),
        UserConfigErrorReason::TemplateFilesDirDoesNotExist(error, fix) => (error, fix),
        UserConfigErrorReason::TargetDirectoryShouldNotExist(error, fix) => (error, fix),
    };

    ErrorFormat {
      error_type: "Got a configuration error".to_owned(),
      error_reason: error.to_owned(),
      exception: None,
      remediation: Some(fix.to_owned())
    }
  }
}

impl From<&VariableFileErrorReason> for ErrorFormat {
  fn from(error: &VariableFileErrorReason) -> Self {

    let (error, fix) = match error {
        VariableFileErrorReason::VariableFileNotFound(error, fix) => (error, fix),
        VariableFileErrorReason::VariableOpenError(error, fix) => (error, fix),
        VariableFileErrorReason::VariableReadError(error, fix) => (error, fix),
        VariableFileErrorReason::VariableDecodeError(error, fix) => (error, fix),
    };

    ErrorFormat {
      error_type: "Got a error processing variables".to_owned(),
      error_reason: error.to_owned(),
      exception: None,
      remediation: Some(fix.to_owned())
    }
  }
}

impl fmt::Display for VariableFileErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        VariableFileErrorReason::VariableFileNotFound(error, fix) => write!(f, "{}", ZatError::print_error_fix(&error, &fix)),
        VariableFileErrorReason::VariableOpenError(error, fix) =>  write!(f, "{}", ZatError::print_error_fix(&error, &fix)),
        VariableFileErrorReason::VariableReadError(error, fix) => write!(f, "{}", ZatError::print_error_fix(&error, &fix)),
        VariableFileErrorReason::VariableDecodeError(error, fix) => write!(f, "{}", ZatError::print_error_fix(&error, &fix)),
      }
    }
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

  fn print_formatted_error<E>(err: E) -> String
    where E: Into<ErrorFormat>
  {
    let error = err.into();
    let heading_indent = "  ";
    let content_indent = "    ";

    let error_reason_heading = ZatError::heading(error.error_type.as_str());
    let error_reason = error.error_reason;

    let error_section = s!("{}{}\n{}{}", heading_indent, error_reason_heading, content_indent, error_reason);

    let exception_section = match error.exception {
        Some(exception) => {
          let exception_heading = ZatError::heading("Exception");
          s!("{}{}\n{}{}", heading_indent, exception_heading, content_indent, exception)
        },
        None => "".to_owned(),
    };

    let remediation_section = match error.remediation {
        Some(remediation) => {
          let possible_fix_heading = ZatError::heading("Possible fix");
          s!("{}{}\n{}{}", heading_indent, possible_fix_heading, content_indent, remediation)
        },
        None => "".to_owned(),
    };

    s!("\n\n{}\n{}\n{}", error_section, exception_section, remediation_section)
  }

  fn heading(heading: &str) -> String {
    s!("{}:", Yellow.paint(heading))
  }

  // -------------------------------------------------------------------------------------------------------------------
  // UserConfigError
  // -------------------------------------------------------------------------------------------------------------------
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

//----------------------------------------------------------------------------------------------------------------------
// VariableFileError
//----------------------------------------------------------------------------------------------------------------------

  pub fn variable_file_does_not_exist(path: &str) -> ZatError {
    ZatError::VariableFileError(
      VariableFileErrorReason::VariableFileNotFound(
        s!("Variable file '{}' does not exist. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", path),
        s!("Please create the variable file '{}' with the required tokens. See `zat -h` for more details.", path)
      )
    )
  }

  pub fn variable_file_cant_be_opened(path: &str, reason: &str) -> ZatError {
    ZatError::VariableFileError(
      VariableFileErrorReason::VariableOpenError(
        s!("Variable file '{}' could not be opened due to this error: {}. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", path, reason),
        s!("Make sure Zat can open and read the variable file '{}' and has the required file permissions.", path)
      )
    )
  }

  pub fn variable_file_cant_be_read(path: &str, reason: &str) -> ZatError {
    ZatError::VariableFileError(
      VariableFileErrorReason::VariableReadError(
        s!("Variable file '{}' could not be read due to this error: {}. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", path, reason),
        s!("Make sure Zat can open and read the variable file '{}' and has the required file permissions.", path)
      )
    )
  }

  pub fn variable_file_cant_be_decoded(path: &str, reason: &str) -> ZatError {
    ZatError::VariableFileError(
      VariableFileErrorReason::VariableDecodeError(
        s!("Variable file '{}' could not decoded. It failed decoding with this error: {}. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", path, reason),
        s!("Make the variable file '{}' is a valid JSON file.", path)
      )
    )
  }

  // -------------------------------------------------------------------------------------------------------------------
  // TemplateProcessingError
  // -------------------------------------------------------------------------------------------------------------------

  pub fn no_template_files_to_process(path: &str) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::NoFilesToProcessError(
        s!("There are no template files to process in the template directory '{}'.", path),
        s!("Create at least one file in the template directory '{}' for processing.", path))
    )
  }


  pub fn could_not_read_template_file(path: &str, error: String) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::ReadingFileError(
        ReasonFileErrorReason::ReadingError(
          s!("Could not read template file '{}'.", path),
          Some(error),
          s!("Ensure the template file '{}' exists and has the necessary permissions for reading.", path)
        )
      )
    )
  }

  pub fn template_file_content_is_unsupported(path: &str, error: String) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::ReadingFileError(
        ReasonFileErrorReason::UnsupportedContentError(
          s!("Could not decode template file '{}' content to a string. Only text file templates are supported.", path),
          Some(error),
          s!("Ensure the template file '{}' is a text file and is not corrupted.", path)
        )
      )
    )
  }

  pub fn could_not_determine_base_path_of_template_file(base_path: &str, path: &str, error: String) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::ReadingFileError(
        ReasonFileErrorReason::PrefixError(
          s!("Could not find base path {} in template file '{}'. The base path is needed to find the relative path at the output.", base_path, path),
          Some(error),
          s!("Ensure the template file '{}' is a text file and is not corrupted.", path)
        )
      )
    )
  }

  pub fn could_not_write_output_file(path: &str, error: String) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::WritingFileError(
        s!("Could not write output file '{}'.", path),
        Some(error),
        s!("Ensure the output file '{}' has the necessary permissions to be written and is a valid file name.", path)
      )
    )
  }

  pub fn could_not_create_output_file_directory(path: &str, error: String) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::DirectoryCreationError(
        s!("Could not create output directory '{}'.", path),
        Some(error),
        s!("Ensure the output directory '{}' has the necessary permissions to be created and has a valid directory name.", path)
      )
    )
  }


  // -------------------------------------------------------------------------------------------------------------------
  // PostProcessingError
  // -------------------------------------------------------------------------------------------------------------------

  pub fn post_processing_hook_failed(path: &str, error: String) -> ZatError {
    ZatError::PostProcessingError(
      PostProcessingErrorReason::ExecutionError(
        s!("Shell hook `{}` failed with an error.", path),
        Some(error),
        s!("Please ensure the shell hook file {} exists and is executable.", path))
    )
  }

  pub fn post_processing_hook_completed_with_non_zero_status(path: &str, status: i32) -> ZatError {
    ZatError::PostProcessingError(
      PostProcessingErrorReason::NonZeroStatusCode(
        s!("Shell hook `{}` failed with status code {}. The shell hook failed with a non-zero error code signifying an error.", path, status),
        s!("Please check the logs above for why the shell hook failed. Try running the shell hook file `{}` manually by itself on the output to iterate on the error", path))
    )
  }

  pub fn post_processing_hook_was_shutdown(path: &str) -> ZatError {
    ZatError::PostProcessingError(
      PostProcessingErrorReason::ProcessInterrupted(
        s!("Shell hook `{}` was shutdown. Some other process killed the shell hook process.", path),
        s!("Try running the shell hook file `{}` manually on the output.", path))
    )
  }
}


impl std::fmt::Display for ZatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let string_rep = match self {
        ZatError::UserConfigError(error)        => ZatError::print_formatted_error(error),
        ZatError::VariableFileError(error)      => ZatError::print_formatted_error(error),
        ZatError::TemplateProcessingError(_)    => s!("There was an error running the template {}.", "TODO: Remove"),
        ZatError::PostProcessingError(error)    => s!("There was an error running the post processor {}.", "TODO: Remove"),
      };

      write!(f, "{}", string_rep)
    }
}
