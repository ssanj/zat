use super::ErrorFormat;
use std::format as s;

#[derive(Debug, Clone, PartialEq)]
pub enum PluginErrorReason {
  PluginFailure(String, String, String, String),
  CouldNotRunPlugin(String, String),
  CouldNotDecodePluginOutputToUtf8(String, String),
  CouldNotDecodePluginStdErrToUtf8(String, String),
  CouldNotDecodePluginResultToJson(String, String, String),
}

impl From<&PluginErrorReason> for ErrorFormat {

  fn from(error: &PluginErrorReason) -> ErrorFormat {

    let (plugin_name, error, ex, fix) = match error {
      PluginErrorReason::PluginFailure(plugin_name, error, exception, fix) => (plugin_name, error.to_owned(), exception.to_owned(), fix.to_owned()),
      PluginErrorReason::CouldNotRunPlugin(_, _) => todo!(),
      PluginErrorReason::CouldNotDecodePluginOutputToUtf8(_, _) => todo!(),
      PluginErrorReason::CouldNotDecodePluginStdErrToUtf8(_, _) => todo!(),
      PluginErrorReason::CouldNotDecodePluginResultToJson(_, _, _) => todo!(),
    };

    let error_reason = s!("Plugin {} returned the following error: {}", plugin_name, error);

    ErrorFormat {
      error_reason,
      exception: Some(ex),
      remediation: Some(fix)
    }
  }
}
