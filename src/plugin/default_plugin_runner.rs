use std::collections::HashMap;
use std::process::Command;

use super::PluginRunner;
use crate::config::UserConfig;
use crate::logging::{Logger, verbose_logger};
use crate::templates::{TemplateVariable, TemplateVariables, Plugin, PluginArg, PluginRunResult, PluginRunStatus};
use crate::error::{ZatResult, ZatError, ZatAction};
use std::{println as p, format as s};

pub struct DefaultPluginRunner;

#[derive(Debug, Clone, serde::Deserialize)]
pub enum PluginResult {
  Success(PluginSuccess),
  Error(PluginError)
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginSuccess {
  result: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginError {
  plugin_name: String,
  error: String,
  exception: Option<String>,
  fix: String
}

impl PluginRunner for DefaultPluginRunner {
  fn run_plugins(&self, template_variables: &mut TemplateVariables) -> ZatAction {
    DefaultPluginRunner::run_plugins(template_variables)
  }
}


impl DefaultPluginRunner {
  pub fn new() -> Self {
    DefaultPluginRunner
  }

  fn run_plugin(plugin: Plugin) -> ZatResult<PluginResult> {
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

  fn run_plugins(template_variables: &mut TemplateVariables) -> ZatAction {

    for tv in template_variables.tokens.iter_mut() {
      if let Some(plugin) = tv.plugin.as_mut() {
        let run_result = DefaultPluginRunner::run_plugin(plugin.clone());
        match run_result {
          Ok(PluginResult::Success(plugin_success)) => {
            plugin.result = PluginRunStatus::Run(PluginRunResult::new(&plugin_success.result));
          },
          Ok(PluginResult::Error(error)) => {
            let exception = &error.exception.unwrap_or("<No Exception>".to_owned());
            let zerr = ZatError::plugin_return_error(&error.plugin_name, &error.error, exception, &error.fix);
            return Err(zerr)
          },
          Err(error) => return Err(error),
        }
      }
    }

    Ok(())
  }

}
