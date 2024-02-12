use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Plugin {
  pub id: String,
  pub args: Vec<PluginArg>,

  #[serde(default)]
  pub result: PluginRunStatus,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum PluginRunStatus {
  NotRun,
  Run(PluginRunResult)
}

impl Default for PluginRunStatus {
  fn default() -> Self {
      Self::NotRun
  }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PluginRunResult {
  pub result: String,
}

impl PluginRunResult {
  pub fn new(result: &str) -> Self {
    Self {
      result: result.to_owned()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PluginArg {
  pub name: String,
  pub value: String,
  pub prefix: String
}
