use std::process::Command;

use super::{PluginResult, PluginRunner};
use crate::logging::Logger;
use crate::templates::{Plugin, ArgType};
use crate::error::{ZatResult, ZatError};
use std::format as s;

pub struct DefaultPluginRunner;

impl PluginRunner for DefaultPluginRunner {
  fn run_plugin(&self, plugin: Plugin) -> ZatResult<PluginResult> {
    Logger::info(&s!("Running {} plugin...", plugin.id));

    let mut command = Command::new(&plugin.id);

    match &plugin.args {
      ArgType::NoArgs => (),
      ArgType::MutlipleArgs(first, rest) => {
        let mut args = vec![first.clone()];
        let mut other = rest.clone();
        args.append(&mut other);

        for arg in args {
          command
            .arg(s!("{}{}", &arg.prefix, &arg.name))
            .arg(&arg.value);
        }
      },
      ArgType::ArgLine(args) => {
        command.args(args);
      },
    };

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

    let plugin_args = plugin.args.clone();

    let args_vec: Vec<String> =
      match plugin_args {
        ArgType::NoArgs => vec![],
        ArgType::MutlipleArgs(f, r) => {
          let mut items = vec![f];
          let mut other_items = r;
          items.append(&mut other_items);

          items
            .into_iter()
            .map(|i| s!("{}{} {}", i.prefix, i.name, i.value))
            .collect::<Vec<String>>()
        },
        ArgType::ArgLine(args) => args,
      };

    let args = args_vec.join(" ");

    s!("{} {}", program, args)
  }
}
