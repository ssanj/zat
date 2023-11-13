use super::ErrorFormat;

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

impl From<&TemplateProcessingErrorReason> for ErrorFormat {
  fn from(error: &TemplateProcessingErrorReason) -> Self {
    let (error, exception, fix) = match error {
        TemplateProcessingErrorReason::NoFilesToProcessError(error, fix) => (error, &None, fix),
        TemplateProcessingErrorReason::ReadingFileError(ReasonFileErrorReason::ReadingError(error, exception, fix)) => (error, exception, fix),
        TemplateProcessingErrorReason::ReadingFileError(ReasonFileErrorReason::UnsupportedContentError(error, exception, fix)) => (error, exception, fix),
        TemplateProcessingErrorReason::ReadingFileError(ReasonFileErrorReason::PrefixError(error, exception, fix)) => (error, exception, fix),
        TemplateProcessingErrorReason::WritingFileError(error, exception, fix) => (error, exception, fix),
        TemplateProcessingErrorReason::DirectoryCreationError(error, exception, fix) => (error, exception, fix),
    };

    ErrorFormat {
      error_reason: error.to_owned(),
      exception: exception.to_owned(),
      remediation: Some(fix.to_owned())
    }
  }
}
