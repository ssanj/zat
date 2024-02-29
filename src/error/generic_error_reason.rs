use super::error_format::ErrorFormat;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericErrorReason {
  pub error: String,
  pub exception: String,
  pub fix: String
}

impl From<&GenericErrorReason> for ErrorFormat {
  fn from(field: &GenericErrorReason) -> Self {
    ErrorFormat {
      error_reason: field.error.clone(),
      exception: Some(field.exception.clone()),
      remediation: Some(field.fix.clone())
    }
  }
}
