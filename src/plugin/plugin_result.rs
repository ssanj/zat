#[derive(Debug, Clone, serde::Deserialize)]
pub enum PluginResult {
  #[serde(rename = "success")]
  Success(PluginSuccess),
  #[serde(rename = "error")]
  Error(PluginError)
}

#[cfg(test)]
impl PluginResult {
  pub fn success(result: String) -> Self {
    PluginResult::Success(
      PluginSuccess {
        result
      }
    )
  }

  pub fn error(plugin_name: String, error: String, exception: Option<String>, fix: String) -> Self {
      PluginResult::Error(
        PluginError {
          plugin_name,
          error,
          exception,
          fix
        }
      )
  }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginSuccess {
  pub result: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginError {
  pub plugin_name: String,
  pub error: String,
  pub exception: Option<String>,
  pub fix: String
}
