use crate::error::ZatResult;
use crate::templates::Plugin;
use crate::plugin::PluginResult;

/// Runs a plugin and returns the result
pub trait PluginRunner {
  fn run_plugin(&self, plugin: Plugin) -> ZatResult<PluginResult>;
}
