#[derive(Debug)]
pub struct ErrorFormat {
  pub error_reason: String,
  pub exception: Option<String>,
  pub remediation: Option<String>
}
