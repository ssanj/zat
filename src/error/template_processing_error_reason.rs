use super::ErrorFormat;

#[derive(Debug, PartialEq, Clone)]
pub enum TemplateProcessingErrorReason {
  NoFilesToProcessError(String, String),
  ReadingFileError(ReasonFileErrorReason),
  WritingFileError(String, String, String),
  DirectoryCreationError(String, String, String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReasonFileErrorReason {
  ReadingError(String, String, String),
  UnsupportedContentError(String, String, String),
  PrefixError(String, String, String),
}

impl From<&TemplateProcessingErrorReason> for ErrorFormat {
  fn from(error: &TemplateProcessingErrorReason) -> Self {
    let (error, exception, fix) = match error {
        TemplateProcessingErrorReason::NoFilesToProcessError(error, fix) => (error, None, fix),
        TemplateProcessingErrorReason::ReadingFileError(ReasonFileErrorReason::ReadingError(error, exception, fix)) => (error, Some(exception), fix),
        TemplateProcessingErrorReason::ReadingFileError(ReasonFileErrorReason::UnsupportedContentError(error, exception, fix)) => (error, Some(exception), fix),
        TemplateProcessingErrorReason::ReadingFileError(ReasonFileErrorReason::PrefixError(error, exception, fix)) => (error, Some(exception), fix),
        TemplateProcessingErrorReason::WritingFileError(error, exception, fix) => (error, Some(exception), fix),
        TemplateProcessingErrorReason::DirectoryCreationError(error, exception, fix) => (error, Some(exception), fix),
    };

    ErrorFormat {
      error_reason: error.to_owned(),
      exception: exception.cloned(),
      remediation: Some(fix.to_owned())
    }
  }
}
