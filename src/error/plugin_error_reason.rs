use super::ErrorFormat;
use std::format as s;

#[derive(Debug, Clone, PartialEq)]
pub enum PluginErrorReason {
  PluginFailure(String, String, String, String),
  PluginReturnedInvalidExitCodeFailure(String, String, String),
  CouldNotRunPlugin(String, String, String, String),
  CouldNotDecodePluginOutputToUtf8(String, String),
  CouldNotDecodePluginStdErrToUtf8(String, String),
  CouldNotDecodePluginResultToJson(String, String, String),
}

impl From<&PluginErrorReason> for ErrorFormat {

  fn from(error: &PluginErrorReason) -> ErrorFormat {

    let (plugin_name, error, opt_exception, fix) = match error {
      PluginErrorReason::PluginFailure(plugin_name, error, exception, fix) => (plugin_name, error.to_owned(), Some(exception.to_owned()), fix.to_owned()),
      PluginErrorReason::PluginReturnedInvalidExitCodeFailure(plugin_name, error, fix) => (plugin_name, error.to_owned(), None, fix.to_owned()),
      PluginErrorReason::CouldNotRunPlugin(plugin_name, error, exception, fix) => (plugin_name, error.to_owned(), Some(exception.to_owned()), fix.to_owned()),
      PluginErrorReason::CouldNotDecodePluginOutputToUtf8(_, _) => todo!(),
      PluginErrorReason::CouldNotDecodePluginStdErrToUtf8(_, _) => todo!(),
      PluginErrorReason::CouldNotDecodePluginResultToJson(_, _, _) => todo!(),
    };

    let error_reason = s!("Plugin '{}' returned the following error: {}", plugin_name, error);

    ErrorFormat {
      error_reason,
      exception: opt_exception,
      remediation: Some(fix)
    }
  }
}
