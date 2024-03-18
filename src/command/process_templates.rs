use crate::choice::{ChoiceRunner, DefaultChoiceRunner, DefaultChoiceScopeFilter, ChoiceScopeFilter, SelectedChoices};
use crate::error::ZatAction;
use crate::logging::{VerboseLogger, Logger};
use crate::post_processor::{PostProcessingHook, ShellHook};
use crate::processor::{ProcessTemplates as TemplateProcessing, DefaultProcessTemplates};
use crate::templates::{DefaultTemplateVariableProvider, TemplateVariableProvider, TemplateVariables, UserChoices};
use crate::templates::{TemplateConfigValidator, DefaultTemplateConfigValidator};
use crate::templates::TemplateVariableReview;
use crate::token_expander::{ExpandFilters, DefaultExpandFilters};
use crate::plugin::{DefaultPluginRunner, PluginRunnerWorkflow};
use crate::config::UserConfig;
use std::format as s;


pub struct ProcessTemplates;

impl ProcessTemplates {

  pub fn process(user_config: UserConfig) -> ZatAction {
    VerboseLogger::log_user_config(&user_config);

    // Reads the .variables.zat-prompt file into TemplateVariables
    let template_variable_provider = DefaultTemplateVariableProvider::new();
    let template_variables: TemplateVariables = template_variable_provider.get_tokens(user_config.clone())?;
    VerboseLogger::log_template_variables(&user_config, &template_variables);

    // Ask for user choices and separate choices from other variables
    let SelectedChoices { choices, mut other_variables } = DefaultChoiceRunner::run_choices(template_variables)?;

    DefaultChoiceScopeFilter::filter_scopes(&choices, &mut other_variables);
    VerboseLogger::log_template_variables_after_scope_filter(&user_config, &other_variables);

    // Runs any plugins that have been defined and updates template_variables with results
    let plugin_runner = DefaultPluginRunner::new();
    PluginRunnerWorkflow::run_plugins(plugin_runner, &mut other_variables)?;
    VerboseLogger::log_template_variables_after_plugins_run(&user_config, &other_variables);

    // Ask for the user for the value of each variable
    // Then verify all the variables supplied are correct
    let template_config_validator = DefaultTemplateConfigValidator::new();

    // TODO: Do we need this template_variables.clone()?
    let template_variable_review = template_config_validator.validate(user_config.clone(), &SelectedChoices::new(choices, other_variables.clone().tokens))?;

    match template_variable_review {
      TemplateVariableReview::Accepted(vc) => {
        VerboseLogger::log_user_supplied_variables(&user_config, &vc);
        let user_variables = vc.user_variables;
        let user_choices = UserChoices::new(vc.user_choices);
        let expand_filters = DefaultExpandFilters::new();
        let tokenized_key_expanded_variables = expand_filters.expand_filers(other_variables, user_variables);

        VerboseLogger::expanded_tokens(&user_config, &tokenized_key_expanded_variables);
        DefaultProcessTemplates.process_templates(user_config.clone(), tokenized_key_expanded_variables, user_choices)?;

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
        Logger::warn("The user rejected the input review.")
      }
    }

    Ok(())
  }
}
