use super::ErrorFormat;

#[derive(Debug, PartialEq, Clone)]
pub enum UserConfigErrorReason {
  RepositoryDirDoesNotExist(String, String),
  TemplateFilesDirDoesNotExist(String, String),
  TargetDirectoryShouldNotExist(String, String),
}


impl From<&UserConfigErrorReason> for ErrorFormat {
  fn from(error: &UserConfigErrorReason) -> Self {

    let (error, fix) = match error {
        UserConfigErrorReason::RepositoryDirDoesNotExist(error, fix) => (error, fix),
        UserConfigErrorReason::TemplateFilesDirDoesNotExist(error, fix) => (error, fix),
        UserConfigErrorReason::TargetDirectoryShouldNotExist(error, fix) => (error, fix),
    };

    ErrorFormat {
      error_reason: error.to_owned(),
      exception: None,
      remediation: Some(fix.to_owned())
    }
  }
}

