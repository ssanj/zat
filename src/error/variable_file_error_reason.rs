use super::ErrorFormat;

#[derive(Debug, PartialEq, Clone)]
pub enum VariableFileErrorReason {
  VariableFileNotFound(String, String),
  VariableOpenError(String, String),
  VariableReadError(String, String),
  VariableDecodeError(String, String),
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
      error_reason: error.to_owned(),
      exception: None,
      remediation: Some(fix.to_owned())
    }
  }
}
