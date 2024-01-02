use super::ErrorFormat;

#[derive(Debug, PartialEq, Clone)]
pub enum BootstrapCommandErrorReason {
  RepositoryDirectoryShouldNotExist(String, String),
  CouldNotCreateRepositoryDirectory(String, String, String),
  CouldNotCreateFile(String, String, String),
}

impl From<&BootstrapCommandErrorReason> for ErrorFormat {
  fn from(error: &BootstrapCommandErrorReason) -> Self {
    let (error, exception, fix) = match error {
        BootstrapCommandErrorReason::RepositoryDirectoryShouldNotExist(error, fix) => (error, None, fix),
        BootstrapCommandErrorReason::CouldNotCreateRepositoryDirectory(error, exception, fix) => (error, Some(exception), fix),
        BootstrapCommandErrorReason::CouldNotCreateFile(error, exception, fix) => (error, Some(exception), fix),
    };

    ErrorFormat {
      error_reason: error.to_owned(),
      exception: exception.cloned(),
      remediation: Some(fix.to_owned())
    }
  }
}
