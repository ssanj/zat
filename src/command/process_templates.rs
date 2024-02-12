use crate::error::ZatAction;
use crate::logging::{VerboseLogger, Logger};
use crate::post_processor::{PostProcessingHook, ShellHook};
use crate::processor::{ProcessTemplates as TemplateProcessing, DefaultProcessTemplates};
use crate::templates::{TemplateVariableProvider, DefaultTemplateVariableProvider};
use crate::templates::{TemplateConfigValidator, DefaultTemplateConfigValidator};
use crate::templates::TemplateVariableReview;
use crate::token_expander::{ExpandFilters, DefaultExpandFilters};
use crate::plugin::{PluginRunner, DefaultPluginRunner};
use crate::config::UserConfig;
use std::format as s;


pub struct ProcessTemplates;

impl ProcessTemplates {

  pub fn process(user_config: UserConfig) -> ZatAction {
    VerboseLogger::log_user_config(&user_config);

    // Reads the .variables.zat-prompt file into TemplateVariables
    let template_variable_provider = DefaultTemplateVariableProvider::new();
    let template_variables = template_variable_provider.get_tokens(user_config.clone())?;
    VerboseLogger::log_template_variables(&user_config, &template_variables);

    // TODO: Insert plugin running code here? We could process the plugin definitions and set the value from
    // each into the "default_value" field for each by updating the `template_variables`?
    let plugin_runner = DefaultPluginRunner::new();
    let updated_template_variables = plugin_runner.run_plugins(&user_config, template_variables)?;

    // Ask for the user for the value of each variable
    // Then verify all the variables supplied are correct
    let template_config_validator = DefaultTemplateConfigValidator::new();
    let template_variable_review = template_config_validator.validate(user_config.clone(), updated_template_variables.clone());

    match template_variable_review {
      TemplateVariableReview::Accepted(vc) => {
        VerboseLogger::log_user_supplied_variables(&user_config, &vc);
        let user_variables = vc.user_variables;
        let expand_filters = DefaultExpandFilters::new();
        let tokenized_key_expanded_variables = expand_filters.expand_filers(updated_template_variables, user_variables);

        VerboseLogger::expanded_tokens(&user_config, &tokenized_key_expanded_variables);
        DefaultProcessTemplates.process_templates(user_config.clone(), tokenized_key_expanded_variables)?;

        // Run post-processor if one exists
        ShellHook.run(&user_config)?;

        Logger::coloured(
          &s!("{}{}{}",
            Logger::info_str("Extracted template to '"),
            &user_config.target_dir.path.as_str(),
            Logger::info_str("'")
          ))
      },
      TemplateVariableReview::Rejected => {
        Logger::warn("The user rejected the variables.")
      }
    }

    Ok(())
  }
}
