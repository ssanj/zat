use std::{fmt::{self, write}, format as s, unimplemented};

use ansi_term::Color::Yellow;

pub type ZatResult<A> = Result<A, ZatError>;
pub type ZatAction = Result<(), ZatError>;


#[derive(Debug, PartialEq)]
pub enum ZatError {
  UserConfigError(UserConfigErrorReason),
  VariableFileError(VariableFileErrorReason),
  // ReadingFileError(String),
  // WritingFileError(String),
  // DirectoryCreationError(String),
  // NoFilesToProcessError(String),
  ProcessingErrors(Vec<ZatError>),
  TemplateProcessingError(TemplateProcessingErrorReason),
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
pub enum TemplateProcessingErrorReason {
  NoFilesToProcessError(String, String),
  SingleProcessError(TemplateItemErrorReason),
  MultipleProcessingErrors(Vec<TemplateItemErrorReason>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TemplateItemErrorReason {
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

#[derive(Debug, PartialEq)]
pub enum VariableFileErrorReason {
  VariableFileNotFound(String, String),
  VariableOpenError(String, String),
  VariableReadError(String, String),
  VariableDecodeError(String, String),
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

  // See TemplateItemErrorReason section for constructors
  pub fn multiple_template_processing_errors(errors: &[TemplateItemErrorReason]) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::MultipleProcessingErrors(
        Vec::from(errors)
      )
    )
  }


  pub fn could_not_read_template_file(path: &str, error: String) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::SingleProcessError(
        ZatError::item_error_could_not_read_template_file(path, error)
      )
    )
  }

  pub fn template_file_content_is_unsupported(path: &str, error: String) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::SingleProcessError(
        ZatError::item_error_template_file_content_is_unsupported(path, error)
      )
    )
  }

  pub fn could_not_determine_base_path_of_template_file(base_path: &str, path: &str, error: String) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::SingleProcessError(
        ZatError::item_error_could_not_determine_base_path_of_template_file(base_path, path, error)
      )
    )
  }

  pub fn could_not_write_output_file(path: &str, error: String) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::SingleProcessError(
        ZatError::item_error_could_not_write_output_file(path, error)
      )
    )
  }

  pub fn could_not_create_output_file_directory(path: &str, error: String) -> ZatError {
    ZatError::TemplateProcessingError(
      TemplateProcessingErrorReason::SingleProcessError(
        ZatError::item_error_could_not_create_output_file_directory(path, error)
      )
    )
  }

  // -------------------------------------------------------------------------------------------------------------------
  // TemplateItemErrorReason
  // -------------------------------------------------------------------------------------------------------------------

  pub fn item_error_could_not_read_template_file(path: &str, error: String) -> TemplateItemErrorReason {
    TemplateItemErrorReason::ReadingFileError(
      ReasonFileErrorReason::ReadingError(
        s!("Could not read template file '{}'.", path),
        Some(error),
        s!("Ensure the template file '{}' exists and has the necessary permissions for reading.", path)
      )
    )
  }

  pub fn item_error_template_file_content_is_unsupported(path: &str, error: String) -> TemplateItemErrorReason {
    TemplateItemErrorReason::ReadingFileError(
      ReasonFileErrorReason::UnsupportedContentError(
        s!("Could not decode template file '{}' content to a string. Only text file templates are supported.", path),
        Some(error),
        s!("Ensure the template file '{}' is a text file and is not corrupted.", path)
      )
    )
  }

  pub fn item_error_could_not_determine_base_path_of_template_file(base_path: &str, path: &str, error: String) -> TemplateItemErrorReason {
    TemplateItemErrorReason::ReadingFileError(
      ReasonFileErrorReason::PrefixError(
        s!("Could not find base path {} in template file '{}'. The base path is needed to find the relative path at the output.", base_path, path),
        Some(error),
        s!("Ensure the template file '{}' is a text file and is not corrupted.", path)
      )
    )
  }

  pub fn item_error_could_not_write_output_file(path: &str, error: String) -> TemplateItemErrorReason {
    TemplateItemErrorReason::WritingFileError(
      s!("Could not write output file '{}'.", path),
      Some(error),
      s!("Ensure the output file '{}' has the necessary permissions to be written and is a valid file name.", path)
    )
  }

  pub fn item_error_could_not_create_output_file_directory(path: &str, error: String) -> TemplateItemErrorReason {
    TemplateItemErrorReason::DirectoryCreationError(
      s!("Could not create output directory '{}'.", path),
      Some(error),
      s!("Ensure the output directory '{}' has the necessary permissions to be created and has a valid directory name.", path)
    )
  }

}


impl std::fmt::Display for ZatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let string_rep = match self {
        ZatError::UserConfigError(error)        => ZatError::print_error("Got a configuration error:", error),
        ZatError::VariableFileError(error)      => ZatError::print_error("Got a error processing variables:", error),
        // ZatError::ReadingFileError(error)       => s!("Could not read template file:\n    {}", error),
        // ZatError::WritingFileError(error)       => s!("Could not write destination file:\n    {}", error),
        // ZatError::DirectoryCreationError(error) => s!("Could not create directory:\n    {}", error),
        // ZatError::NoFilesToProcessError(path)   => s!("Could not find any files to process at {}.", path),
        ZatError::TemplateProcessingError(_)    => s!("There was an error running the template {}.", "TODO: Remove"),
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
