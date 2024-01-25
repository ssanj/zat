use super::error_format::ErrorFormat;

#[derive(Debug, PartialEq, Clone)]
pub enum ProcessRemoteCommandErrorReason {
  CouldNotCreateCheckoutDirectory(String, String, String),
  RemoteRepositoryUrlIsInvalid(String, String, String),
  RemoteRepositoryUrlHostnameIsInvalid(String, String),
  CouldNotCreateCheckoutDirectoryStructure(String, String, String),
  GitCloneFailed(String, String, String),
  GitCloneStatusError(String, String),
}


impl From<&ProcessRemoteCommandErrorReason> for ErrorFormat {
    fn from(error: &ProcessRemoteCommandErrorReason) -> Self {
      let (error_reason, exception, remediation) =
        match error {
          ProcessRemoteCommandErrorReason::CouldNotCreateCheckoutDirectory(error, exception, remediation) => (error.to_owned(), Some(exception.to_owned()), Some(remediation.to_owned())),
          ProcessRemoteCommandErrorReason::RemoteRepositoryUrlIsInvalid(error, exception, remediation) => (error.to_owned(), Some(exception.to_owned()), Some(remediation.to_owned())),
          ProcessRemoteCommandErrorReason::RemoteRepositoryUrlHostnameIsInvalid(error, remediation) => (error.to_owned(), None, Some(remediation.to_owned())),
          ProcessRemoteCommandErrorReason::CouldNotCreateCheckoutDirectoryStructure(error, exception, remediation) => (error.to_owned(), Some(exception.to_owned()), Some(remediation.to_owned())),
          ProcessRemoteCommandErrorReason::GitCloneFailed(error, exception, remediation) => (error.to_owned(), Some(exception.to_owned()), Some(remediation.to_owned())),
          ProcessRemoteCommandErrorReason::GitCloneStatusError(error, remediation) => (error.to_owned(), None, Some(remediation.to_owned())),
      };

      ErrorFormat {
        error_reason,
        exception,
        remediation,
      }
    }
}
