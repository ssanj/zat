#[derive(Debug, Clone, serde::Deserialize)]
pub enum PluginResult {
  Success(PluginSuccess),
  Error(PluginError)
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
