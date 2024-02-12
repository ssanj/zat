use std::collections::HashMap;
use std::process::Command;

use super::PluginRunner;
use crate::logging::Logger;
use crate::templates::{TemplateVariable, TemplateVariables, Plugin, PluginArg, PluginRunResult, PluginRunStatus};
use crate::error::{ZatResult, ZatError};
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
  display_result: String
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginError {
  header: String,
  error: String,
  exception: Option<String>,
  fix: String
}

impl PluginRunner for DefaultPluginRunner {
  // TODO: We need to maintain template definition order.
  // TODO: should template_variables be mut?
  fn run_plugins(&self, template_variables: TemplateVariables) -> ZatResult<TemplateVariables> {

    // Store results of running plugins against the variable name in the template.
    let mut plugin_hash: HashMap<String, PluginSuccess> = HashMap::new();

    // Put the template variables into a Hash, keyed against the variable name so we can match them with the plugin hash above.
    let mut template_variable_hash: HashMap<String, TemplateVariable> = HashMap::new();

    for tv in template_variables.clone().tokens {
      // Add variable to template_variable_hash
      if let Some(plugin) = &tv.plugin {
        let variable_name = tv.clone().variable_name;
        let run_result = DefaultPluginRunner::run_plugin(plugin.clone());
        match run_result {
          Ok(PluginResult::Success(success)) => {
            plugin_hash.insert(variable_name.clone(), success);
          },
          Ok(PluginResult::Error(error)) => {
            let zerr = ZatError::plugin_return_error("blah", &error.error, &error.exception.unwrap_or("<No Exception>".to_owned()).as_str(), &error.fix);
            return Err(zerr)
          },
          Err(error) =>  return Err(error),
        }

        template_variable_hash.insert(variable_name.clone(), tv);
      } else {
        template_variable_hash.insert(tv.clone().variable_name, tv);
      }
    }


    let mut results_vec: Vec<TemplateVariable> = vec![];

    for (variable_name, template_variable) in template_variable_hash {
      if let Some(plugin_result) = plugin_hash.get(&variable_name) {
        template_variable
          .clone()
          .plugin
          .into_iter()
          .for_each(|plugin| {
            let new_result = PluginRunStatus::Run(PluginRunResult::new(&plugin_result.result, &plugin_result.display_result));


            let mut new_plugin = plugin.clone();
            new_plugin.result = new_result;

            let mut new_template_variable = template_variable.clone();
            new_template_variable.plugin = Some(new_plugin);

            results_vec.push(new_template_variable)
          });
      } else {
        results_vec.push(template_variable)
      }
    }

    println!("---------------> {:?}", results_vec.clone());

    let result = TemplateVariables {
      tokens: results_vec
    };

    Ok(result)
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
      std::str::from_utf8(&output.stdout).map_err(|e| ZatError::could_not_decode_plugin_result_to_UTF8(&program, e.to_string()))?;

    let std_err =
      std::str::from_utf8(&output.stderr).map_err(|e| ZatError::could_not_decode_plugin_stderr_to_UTF8(&program, e.to_string()))?;

    let plugin_result: PluginResult =
      serde_json::from_str(result)
        .map_err(|e| ZatError::could_not_decode_plugin_result_to_JSON(&program, e.to_string(), std_err))?;

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
}
