
use format as s;
use url::Url;
use super::ErrorFormat;
use super::UserConfigErrorReason;
use super::VariableFileErrorReason;
use super::TemplateProcessingErrorReason;
use super::ReasonFileErrorReason;
use super::PostProcessingErrorReason;
use super::BootstrapCommandErrorReason;
use super::ProcessRemoteCommandErrorReason;
use super::PluginErrorReason;
use ansi_term::Color::Yellow;

pub type ZatResult<A> = Result<A, ZatError>;
pub type ZatAction = Result<(), ZatError>;

#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum ZatError {
  ProcessCommandError(ProcessCommandErrorReason),
  BootstrapCommandError(BootstrapCommandErrorReason),
  ProcessRemoteCommandError(ProcessRemoteCommandErrorReason),
  PluginError(PluginErrorReason)

}

#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum ProcessCommandErrorReason {
  UserConfigError(UserConfigErrorReason),
  VariableFileError(VariableFileErrorReason),
  TemplateProcessingError(TemplateProcessingErrorReason),
  PostProcessingError(PostProcessingErrorReason),
}


impl ZatError {

  fn print_formatted_error<E>(error_type: &str, err: E) -> String
    where E: Into<ErrorFormat>
  {
    let error = err.into();
    let heading_indent = "  ";
    let content_indent = "    ";

    let error_reason_heading = ZatError::heading(error_type);
    let error_reason = error.error_reason;

    let error_section = s!("\n\n{}{}\n{}{}", heading_indent, error_reason_heading, content_indent, error_reason);

    let exception_section = match error.exception {
        Some(exception) => {
          let exception_heading = ZatError::heading("Exception");
          s!("\n\n{}{}\n{}{}", heading_indent, exception_heading, content_indent, exception)
        },
        None => "".to_owned(),
    };

    let remediation_section = match error.remediation {
        Some(remediation) => {
          let possible_fix_heading = ZatError::heading("Possible fix");
          s!("\n\n{}{}\n{}{}", heading_indent, possible_fix_heading, content_indent, remediation)
        },
        None => "".to_owned(),
    };

    s!("{}{}{}", error_section, exception_section, remediation_section)
  }

  fn heading(heading: &str) -> String {
    s!("{}:", Yellow.paint(heading))
  }
}

impl ZatError {


  // -------------------------------------------------------------------------------------------------------------------
  // UserConfigError
  // -------------------------------------------------------------------------------------------------------------------
  pub fn template_dir_does_not_exist(path: &str) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::UserConfigError(
        UserConfigErrorReason::RepositoryDirDoesNotExist(
          s!("The Zat repository directory '{}' does not exist. It should exist so Zat can read the template configuration.", path),
          s!("Please create the Zat repository directory '{}' with the Zat folder structure. See `zat --help` for more.", path)
        )
      )
    )
  }

  pub fn template_files_dir_does_not_exist(path: &str) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::UserConfigError(
        UserConfigErrorReason::TemplateFilesDirDoesNotExist(
          s!("The Zat template files directory '{}' does not exist. It should exist so Zat can read the template files.", path),
          s!("Please create the Zat template files directory '{}' with the necessary template files. See `zat --help` for more details.", path)
        )
      )
    )
  }

  pub fn target_dir_should_not_exist(path: &str) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::UserConfigError(
        UserConfigErrorReason::TargetDirectoryShouldNotExist(
          s!("The target directory '{}' should not exist. It will be created when Zat processes the template files.", path),
          "Please supply an empty directory for the target.".to_owned()
        )
      )
    )
  }

//----------------------------------------------------------------------------------------------------------------------
// VariableFileError
//----------------------------------------------------------------------------------------------------------------------

  pub fn variable_file_does_not_exist(path: &str) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::VariableFileError(
        VariableFileErrorReason::VariableFileNotFound(
          s!("Variable file '{}' does not exist. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", path),
          s!("Please create the variable file '{}' with the required tokens. See `zat --help` for more details.", path)
        )
      )
    )
  }

  pub fn variable_file_has_no_variables_defined(path: &str) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::VariableFileError(
        VariableFileErrorReason::VariableFileHasNoVariableDefinitions(
          s!("Variable file '{}' does not define any variables. The purpose of Zat is to provide a templating tool to customise frequently used file structures. It does this by replacing variables defined in the file '{}' on file and directory names of templates as well as within '.tmpl' files. If you want to simply copy a file structure use 'cp' instead.", path, path),
          s!("Please define at least one variable in the variable file '{}'.", path)
        )
      )
    )
  }

  pub fn variable_file_cant_be_opened(path: &str, reason: &str) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::VariableFileError(
        VariableFileErrorReason::VariableOpenError(
          s!("Variable file '{}' could not be opened due to this error: {}. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", path, reason),
          s!("Make sure Zat can open and read the variable file '{}' and has the required file permissions.", path)
        )
      )
    )
  }

  pub fn variable_file_cant_be_read(path: &str, reason: &str) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::VariableFileError(
        VariableFileErrorReason::VariableReadError(
          s!("Variable file '{}' could not be read due to this error: {}. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", path, reason),
          s!("Make sure Zat can open and read the variable file '{}' and has the required file permissions.", path)
        )
      )
    )
  }

  pub fn variable_file_cant_be_decoded(path: &str, reason: &str) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::VariableFileError(
        VariableFileErrorReason::VariableDecodeError(
          s!("Variable file '{}' could not be decoded as JSON into the expected format. It failed decoding with this error: {}. Zat uses this file to retrieve tokens that will be replaced when rendering the templates.", path, reason),
          s!("Make the variable file '{}' is a valid JSON file in the format required by Zat. See `zat --help` for more details on the format", path)
        )
      )
    )
  }

  // -------------------------------------------------------------------------------------------------------------------
  // TemplateProcessingError
  // -------------------------------------------------------------------------------------------------------------------

  pub fn no_template_files_to_process(path: &str) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::TemplateProcessingError(
        TemplateProcessingErrorReason::NoFilesToProcessError(
          s!("There are no template files to process in the template directory '{}'.", path),
          s!("Create at least one file in the template directory '{}' for processing.", path))
      )
    )
  }


  pub fn could_not_read_template_file(path: &str, error: String) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::TemplateProcessingError(
        TemplateProcessingErrorReason::ReadingFileError(
          ReasonFileErrorReason::ReadingError(
            s!("Could not read template file '{}'.", path),
            error,
            s!("Ensure the template file '{}' exists and has the necessary permissions for reading.", path)
          )
        )
      )
    )
  }

  pub fn template_file_content_is_unsupported(path: &str, error: String) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::TemplateProcessingError(
        TemplateProcessingErrorReason::ReadingFileError(
          ReasonFileErrorReason::UnsupportedContentError(
            s!("Could not decode ReasonFileErrorReason::template file '{}' content to a string. Only text file templates are supported.", path),
            error,
            s!("Ensure the template file '{}' is a text file and is not corrupted.", path)
          )
        )
      )
    )
  }

  pub fn could_not_determine_base_path_of_template_file(base_path: &str, path: &str, error: String) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::TemplateProcessingError(
        TemplateProcessingErrorReason::ReadingFileError(
          ReasonFileErrorReason::PrefixError(
            s!("Could not find base path {} in template file '{}'. The base path is needed to find the relative path at the output.", base_path, path),
            error,
            s!("Ensure the template file '{}' is a text file and is not corrupted.", path)
          )
        )
      )
    )
  }

  pub fn could_not_write_output_file(path: &str, error: String) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::TemplateProcessingError(
        TemplateProcessingErrorReason::WritingFileError(
          s!("Could not write output file '{}'.", path),
          error,
          s!("Ensure the output file '{}' has the necessary permissions to be written and is a valid file name.", path)
        )
      )
    )
  }

  pub fn could_not_create_output_file_directory(path: &str, error: String) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::TemplateProcessingError(
        TemplateProcessingErrorReason::DirectoryCreationError(
          s!("Could not create output directory '{}'.", path),
          error,
          s!("Ensure the output directory '{}' has the necessary permissions to be created and has a valid directory name.", path)
        )
      )
    )
  }


  // -------------------------------------------------------------------------------------------------------------------
  // PostProcessingError
  // -------------------------------------------------------------------------------------------------------------------

  pub fn post_processing_hook_failed(path: &str, error: String) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::PostProcessingError(
        PostProcessingErrorReason::ExecutionError(
          s!("Shell hook '{}' failed with an error.", path),
          error,
          s!("Please ensure the shell hook file '{}' exists and is executable.", path))
      )
    )
  }

  pub fn post_processing_hook_completed_with_non_zero_status(path: &str, arg: &str, status: i32) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::PostProcessingError(
        PostProcessingErrorReason::NonZeroStatusCode(
          s!("Shell hook '{} {}' failed with status code {}. The shell hook failed with a non-zero error code signifying an error.", path, arg, status),
          s!("Please check the logs above for why the shell hook failed. Try running the shell hook file '{}' with argument '{}' manually to iterate on the error.", path, arg))
      )
    )
  }

  pub fn post_processing_hook_was_shutdown(path: &str) -> ZatError {
    ZatError::ProcessCommandError(
      ProcessCommandErrorReason::PostProcessingError(
        PostProcessingErrorReason::ProcessInterrupted(
          s!("Shell hook '{}' was shutdown. Some other process killed the shell hook process.", path),
          s!("Try running the shell hook file '{}' manually on the output.", path))
      )
    )
  }

  // -------------------------------------------------------------------------------------------------------------------
  // BootstrapError
  // -------------------------------------------------------------------------------------------------------------------
  pub fn bootstrap_repository_dir_should_not_exist(path: &str) -> ZatError {
    ZatError::BootstrapCommandError(
      BootstrapCommandErrorReason::RepositoryDirectoryShouldNotExist(
          s!("The repository directory '{}' should not exist. It will be created by the Zat bootstrap process.", path),
          "Please supply an empty directory for the repository.".to_owned()
      )
    )
  }

  pub fn could_not_create_bootstrap_repository(e: std::io::Error, path: &str) -> ZatError {
    ZatError::BootstrapCommandError(
      BootstrapCommandErrorReason::CouldNotCreateRepositoryDirectory(
          s!("The repository directory '{}' could not be created.", path),
          e.to_string(),
          s!("Please ensure the path supplied for the repository directory '{}' is writable by the current user", path)
      )
    )
  }

  pub fn could_not_create_bootstrap_file(e: std::io::Error, path: &str) -> ZatError {
    ZatError::BootstrapCommandError(
      BootstrapCommandErrorReason::CouldNotCreateFile(
          s!("The bootstrap file '{}' could not be created.", path),
          e.to_string(),
          s!("Please ensure the file '{}' is writable by the current user", path)
      )
    )
  }


  // -------------------------------------------------------------------------------------------------------------------
  // Process Remote Errors
  // -------------------------------------------------------------------------------------------------------------------

  pub fn could_not_create_checkout_directory(error: String) -> ZatError {
    ZatError::ProcessRemoteCommandError(
      ProcessRemoteCommandErrorReason::CouldNotCreateCheckoutDirectory(
        "Zat could not create a folder under your system's temporary directory. Zat needs to create a temporary local folder to checkout the remote repository.".to_owned(),
        error,
        "Please ensure the Zat user has enough privileges to create a temporary directory and that you are not out of disk space".to_owned()
      )
    )
  }


  pub fn invalid_remote_repository_url(error: String, url: &str) -> ZatError {
    ZatError::ProcessRemoteCommandError(
      ProcessRemoteCommandErrorReason::RemoteRepositoryUrlIsInvalid(
        s!("The remote repository URL supplied '{}' is invalid. Zat needs a valid URL to checkout this repository.", url),
        error,
        "Please ensure the remote repository URL supplied is valid.".to_owned()
      )
    )
  }


  pub fn unsupported_hostname(url: &str) -> ZatError {
    ZatError::ProcessRemoteCommandError(
      ProcessRemoteCommandErrorReason::RemoteRepositoryUrlHostnameIsInvalid(
        s!("The remote repository URL supplied '{}' has an invalid hostname. Zat needs the hostname to be a domain name or IP address. IPv6 addresses should be supplied within square braces.", url),
        "Please ensure the remote repository URL hostname is a domain or an IP address.".to_owned()
      )
    )
  }


  pub fn could_not_create_checkout_directory_structure(error: String, path: &str, url: &Url) -> ZatError {
    ZatError::ProcessRemoteCommandError(
      ProcessRemoteCommandErrorReason::CouldNotCreateCheckoutDirectoryStructure(
        s!("Zat could not create the local checkout directory '{}'. Zat needs to create a local checkout directory for the remote repository '{}' before it clones it locally.", path, url),
        error,
        s!("Please ensure the Zat user has enough privileges to create a directory at '{}'.", path)
      )
    )
  }

  pub fn git_clone_error(error: String, program: &str, url: &str, path: &str) -> ZatError {
    ZatError::ProcessRemoteCommandError(
      ProcessRemoteCommandErrorReason::GitCloneFailed(
        s!("Zat could not could not clone remote repository '{}' to local path '{}'. \n\n    Zat ran the following command to clone the remote repository: \n    '{}'", url, path, program),
        error,
        s!("Please ensure you have Git installed and it's accessible on the PATH used by Zat. Please also ensure Zat has enough privileges to create a directory at '{}'.", path)
      )
    )
  }

  pub fn git_clone_status_error(error_code: Option<i32>, program: &str, url: &str) -> ZatError {
    let code = error_code.map_or_else(|| "Unknown".to_owned(), |ec| ec.to_string());
    ZatError::ProcessRemoteCommandError(
      ProcessRemoteCommandErrorReason::GitCloneStatusError(
        s!("Zat could not could not clone remote repository '{}' because it returned an exit code of '{}'.", url, code),
        s!("Please ensure the following command runs successfully external to Zat: '{}'. It should also not require a password as Zat does not support private repositories that are not accessible through your Git user. Please also see the clone output for possible other issues.", program)
      )
    )
  }


  // -------------------------------------------------------------------------------------------------------------------
  // Plugin Errors
  // -------------------------------------------------------------------------------------------------------------------

  pub fn could_not_run_plugin(plugin: &str, exception: String) -> ZatError {
    ZatError::PluginError(
      PluginErrorReason::CouldNotRunPlugin(
        plugin.to_owned(),
        "Plugin could not be run. Does it exist and is it executable?".to_owned(),
        exception,
        "Try running the plugin manually to fix the above error.".to_owned()
      )
    )
  }


  pub fn plugin_return_invalid_status_code(plugin: &str, opt_code: Option<&i32>) -> ZatError {
    ZatError::PluginError(
      PluginErrorReason::PluginReturnedInvalidExitCodeFailure(
        plugin.to_owned(),
        s!("Plugin failed with status code {}. The plugin failed with a non-zero error code signifying an error.",
          opt_code
            .map_or_else(|| "unknown".to_owned(), |ec| ec.to_string())
        ),
        "Try running the plugin manually to fix the above error.".to_owned()
      )
    )
  }


  pub fn could_not_decode_plugin_result_to_utf8(plugin: &str, exception: String) -> ZatError {
    ZatError::PluginError(
      PluginErrorReason::CouldNotDecodePluginOutputToUtf8(
        plugin.to_owned(),
        "The plugin return invalid UTF8 characters.".to_owned(),
        exception.to_owned(),
        "Try running the plugin manually to fix the above error.".to_owned()
      )
    )
  }


  pub fn could_not_decode_plugin_stderr_to_utf8(plugin: &str, exception: String) -> ZatError {
    ZatError::PluginError(
      PluginErrorReason::CouldNotDecodePluginStdErrToUtf8(
        plugin.to_owned(),
        "The plugin return invalid UTF8 characters to stderr.".to_owned(),
        exception.to_owned(),
        "Try running the plugin manually to fix the above error.".to_owned()
      )
    )
  }

  pub fn could_not_decode_plugin_result_to_json(plugin: &str, exception: String,result: &str, std_err: &str) -> ZatError {
    let error_message =
      if !std_err.is_empty() {
        s!(" The plugin returned the following error: {}", std_err)
      } else {
        "".to_owned()
      };

    ZatError::PluginError(
      PluginErrorReason::CouldNotDecodePluginResultToJson(
        plugin.to_owned(),
        s!("Could not decode result from plugin. The plugin returned: '{}'.{}", result, error_message),
        exception.to_owned(),
        "Try running the plugin manually verify the output format of the plugin adheres to the Zat Plugin Specification.".to_owned()
      )
    )
  }

  pub fn plugin_returned_error(plugin: &str, error: &str, exception: &str, fix: &str) -> ZatError {
    ZatError::PluginError(
      PluginErrorReason::PluginFailure(
        plugin.to_owned(),
        error.to_owned(),
        exception.to_owned(),
        fix.to_owned()
      )
    )
  }


  // -------------------------------------------------------------------------------------------------------------------
  // Choice Errors
  // -------------------------------------------------------------------------------------------------------------------

  pub fn could_not_get_choice_input(exception: String) -> ZatError {
    println!("Could not retreive user choice. {}", exception);
    todo!()
  }

  pub fn could_not_render_template(file: &str, content: &str, exception: String) -> ZatError {
    println!("Could not render template. file:{file} content:{content} exception:{exception}");
    todo!()
  }
}


impl std::fmt::Display for ZatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let string_rep = match self {
        ZatError::ProcessCommandError(ProcessCommandErrorReason::UserConfigError(error))                => ZatError::print_formatted_error("Got a configuration error", error),
        ZatError::ProcessCommandError(ProcessCommandErrorReason::VariableFileError(error))              => ZatError::print_formatted_error("Got an error processing variables", error),
        ZatError::ProcessCommandError(ProcessCommandErrorReason::TemplateProcessingError(error))        => ZatError::print_formatted_error("There was an error running the template", error),
        ZatError::ProcessCommandError(ProcessCommandErrorReason::PostProcessingError(error))            => ZatError::print_formatted_error("There was an error running the post processor", error),
        ZatError::BootstrapCommandError(error)                                                          =>
          ZatError::print_formatted_error("There was an error running the bootstrap process", error),
       ZatError::ProcessRemoteCommandError(error)                                                       =>
          ZatError::print_formatted_error("There was an error running a remote processing command", error),
       ZatError::PluginError(error)                                                       =>
          ZatError::print_formatted_error("There was an error running a plugin", error),
      };

      write!(f, "{}", string_rep)
    }
}
