use std::process::Command;

use super::{PluginResult, PluginRunner};
use crate::logging::Logger;
use crate::templates::Plugin;
use crate::error::{ZatResult, ZatError};
use std::format as s;

pub struct DefaultPluginRunner;

impl PluginRunner for DefaultPluginRunner {
  fn run_plugin(&self, plugin: Plugin) -> ZatResult<PluginResult> {
    Logger::info(&s!("Running {} plugin...", plugin.id));

    let mut command = Command::new(&plugin.id);

    for arg in &plugin.args {
      command
        .arg(s!("{}{}", &arg.prefix, &arg.name))
        .arg(&arg.value);
    }

    let program = Self::generate_command_string(&plugin);

    let output =
        command
          .output()
          .map_err(|e| ZatError::could_not_run_plugin(&program, e.to_string()))?;

    let result =
      std::str::from_utf8(&output.stdout).map_err(|e| ZatError::could_not_decode_plugin_result_to_utf8(&program, e.to_string()))?;

    let std_err =
      std::str::from_utf8(&output.stderr).map_err(|e| ZatError::could_not_decode_plugin_stderr_to_utf8(&program, e.to_string()))?;

    let plugin_result: PluginResult =
      serde_json::from_str(result)
        .map_err(|e| ZatError::could_not_decode_plugin_result_to_json(&program, e.to_string(), std_err))?;

    Ok(plugin_result)
  }
}

impl DefaultPluginRunner {
  pub fn new() -> Self {
    DefaultPluginRunner
  }

  fn generate_command_string(plugin: &Plugin) -> String {
    let program = plugin.id.as_str();

    let args =
      plugin
        .clone()
        .args
        .into_iter()
        .map(|arg| s!("{}{} {}", arg.prefix, arg.name, arg.value))
        .collect::<Vec<String>>()
        .join(" ");

    s!("{} {}", program, args)
  }
}
