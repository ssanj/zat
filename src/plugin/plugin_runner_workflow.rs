use super::{PluginResult, PluginRunner};
use crate::templates::{TemplateVariables, PluginRunResult, PluginRunStatus};
use crate::error::{ZatError, ZatAction};

pub struct PluginRunnerWorkflow;

impl PluginRunnerWorkflow {

  pub fn run_plugins(plugin_runner: impl PluginRunner, template_variables: &mut TemplateVariables) -> ZatAction {

    for tv in template_variables.tokens.iter_mut() {
      if let Some(plugin) = tv.plugin.as_mut() {
        let run_result = plugin_runner.run_plugin(plugin.clone());
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
