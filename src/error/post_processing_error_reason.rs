use super::ErrorFormat;

#[derive(Debug, PartialEq, Clone)]
pub enum PostProcessingErrorReason {
  ExecutionError(String, String, String),
  NonZeroStatusCode(String, String),
  ProcessInterrupted(String, String),
}

impl From<&PostProcessingErrorReason> for ErrorFormat {
  fn from(error: &PostProcessingErrorReason) -> Self {

    let (error, exception, fix) = match error {
        PostProcessingErrorReason::ExecutionError(error, exception, fix) => (error, Some(exception), fix),
        PostProcessingErrorReason::NonZeroStatusCode(error, fix) => (error, None, fix),
        PostProcessingErrorReason::ProcessInterrupted(error, fix) => (error, None, fix),
    };

    ErrorFormat {
      error_reason: error.to_owned(),
      exception: exception.cloned(),
      remediation: Some(fix.to_owned())
    }
  }
}
