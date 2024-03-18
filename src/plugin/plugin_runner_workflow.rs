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
            let zerr = ZatError::plugin_returned_error(&error.plugin_name, &error.error, exception, &error.fix);
            return Err(zerr)
          },
          Err(error) => return Err(error),
        }
      }
    }

    Ok(())
  }
}


#[cfg(test)]
mod tests {
    use crate::assert_error_with;
    use crate::error::error_format::ErrorFormat;
    use crate::error::plugin_error_reason::PluginErrorReason;
    use crate::templates::{Plugin, TemplateVariable};
    use std::format as s;
    use super::*;
    use pretty_assertions::assert_eq;

    struct PanicingPluginRunner;

    struct SuccessfulPluginRunner(String);

    struct FailingPluginRunner(String, String, String, String);

    impl PluginRunner for PanicingPluginRunner {
        fn run_plugin(&self, _plugin: crate::templates::Plugin) -> crate::error::ZatResult<PluginResult> {
            panic!("Running the plugin failed")
        }
    }

    impl PluginRunner for SuccessfulPluginRunner {

      fn run_plugin(&self, _plugin: crate::templates::Plugin) -> crate::error::ZatResult<PluginResult> {
          Ok(PluginResult::success(self.0.clone()))
      }
    }

    impl PluginRunner for FailingPluginRunner {
      fn run_plugin(&self, _plugin: Plugin) -> crate::error::ZatResult<PluginResult> {
          Ok(PluginResult::error(self.0.clone(), self.1.clone(), Some(self.2.clone()), self.3.clone()))
      }
    }

    fn create_template_variable(variable_name: &str, description: &str, prompt: &str) -> TemplateVariable {

      TemplateVariable {
        variable_name: variable_name.to_owned(),
        description: description.to_owned(),
        prompt: prompt.to_owned(),
        filters: Vec::default(),
        default_value: Option::default(),
        plugin: Option::default(),
        choice: Vec::default(),
        scopes: Option::default(),
      }
    }

    fn create_template_variables(n: u8) -> TemplateVariables {
      let mut tokens: Vec<TemplateVariable> = vec![];

      for n in 0..n {
        tokens.push(
          create_template_variable(
            s!("variable_name-{}", n).as_str(),
            s!("description-{}", n).as_str(),
            s!("prompt-{}", n).as_str()
          )
        )
      }

      TemplateVariables {
        tokens
      }
    }

    #[test]
    fn should_not_run_plugins_if_not_defined() {
      let mut template_variables = create_template_variables(2);
      let  expected_template_variables = create_template_variables(2);

      // By using a PanicingPluginRunner we prove that it is not used
      let plugin_runner = PanicingPluginRunner;
      let result = PluginRunnerWorkflow::run_plugins(plugin_runner, &mut template_variables);

      assert_eq!(result, Ok(()));
      assert_eq!(template_variables, expected_template_variables)
    }

    #[test]
    fn should_handle_plugin_success() {
      let mut template_variables = create_template_variables(3);
      let mut expected_template_variables = create_template_variables(3);

      let variable: &mut TemplateVariable = template_variables.tokens.get_mut(1).unwrap();

      let plugin =
        Plugin {
          id: "My Plugin".to_owned(),
          args: Default::default(),
          result: PluginRunStatus::NotRun,
        };

      variable.plugin = Some(plugin.clone());

      let expected_variable = expected_template_variables.tokens.get_mut(1).unwrap();

      let mut expected_plugin = plugin;
        expected_plugin.result =
          PluginRunStatus::Run(
            PluginRunResult {
              result: "some result".to_owned()
            }
          );

      expected_variable.plugin = Some(expected_plugin);


      let plugin_runner = SuccessfulPluginRunner("some result".to_owned());
      let result = PluginRunnerWorkflow::run_plugins(plugin_runner, &mut template_variables);

      assert_eq!(result, Ok(()));
      assert_eq!(template_variables, expected_template_variables)
    }

    #[test]
    fn should_handle_plugin_failure() {
      let mut template_variables = create_template_variables(3);

      let variable: &mut TemplateVariable = template_variables.tokens.get_mut(1).unwrap();

      let plugin =
        Plugin {
          id: "My Plugin".to_owned(),
          args: Default::default(),
          result: PluginRunStatus::NotRun,
        };

      variable.plugin = Some(plugin);

      let expected_template_variables = template_variables.clone();

      let plugin_runner =
        FailingPluginRunner(
          "My Plugin".to_owned(),
          "some error".to_owned(),
          "some exception".to_owned(),
          "some fix".to_owned()
          );

      let result = PluginRunnerWorkflow::run_plugins(plugin_runner, &mut template_variables);

      let assert_error = |error: ErrorFormat| {

        let expected_error =
          ErrorFormat {
            error_reason: "some error".to_owned(),
            exception: Some("some exception".to_owned()),
            remediation: Some("some fix".to_owned())
          };

        assert_eq!(error, expected_error)
      };

      assert_error_with!(
        result,
        Err(ZatError::PluginError(PluginErrorReason::PluginFailure(_, error, exception, fix))) => {
          ErrorFormat {
            error_reason: error,
            exception: Some(exception),
            remediation: Some(fix) }},
        assert_error
      );
      assert_eq!(template_variables, expected_template_variables)
    }
}

