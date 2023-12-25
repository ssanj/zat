use crate::args::UserConfigProvider;
use crate::args::DefaultUserConfigProvider;

use crate::error::ZatAction;

use crate::templates::TemplateVariableProvider;
use crate::templates::DefaultTemplateVariableProvider;
use crate::templates::TemplateConfigValidator;
use crate::templates::DefaultTemplateConfigValidator;
use crate::templates::TemplateVariableReview;

use crate::token_expander::ExpandFilters;
use crate::token_expander::DefaultExpandFilters;

use crate::processor::ProcessTemplates;
use crate::processor::DefaultProcessTemplates;

use crate::post_processor::PostProcessingHook;
use crate::post_processor::ShellHook;

use crate::logging::VerboseLogger;
use crate::logging::Logger;
use std::format as s;

pub struct Workflow;

impl Workflow {

  pub fn execute() -> ZatAction {
    // Verifies that the source dir exists, and the destination does not and handles ignores (defaults and supplied).
    // Basically everything from the cli config.
    let config_provider = DefaultUserConfigProvider::new();
    let user_config = config_provider.get_config()?;
    VerboseLogger::log_user_config(&user_config);

    // Reads the .variables.zat-prompt file into TemplateVariables
    let template_variable_provider = DefaultTemplateVariableProvider::new();
    let template_variables = template_variable_provider.get_tokens(user_config.clone())?;
    VerboseLogger::log_template_variables(&user_config, &template_variables);

    // Ask for the user for the value of each variable
    // Then verify all the variables supplied are correct
    let template_config_validator = DefaultTemplateConfigValidator::new();
    let template_variable_review = template_config_validator.validate(user_config.clone(), template_variables.clone());

    match template_variable_review {
      TemplateVariableReview::Accepted(vc) => {
        VerboseLogger::log_user_supplied_variables(&user_config, &vc);
        let user_variables = vc.user_variables;
        let expand_filters = DefaultExpandFilters::new();
        let tokenized_key_expanded_variables = expand_filters.expand_filers(template_variables, user_variables);

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
      TemplateVariableReview::Rejected => Logger::warn("The user rejected the variables.")
    }

    Ok(())
  }
}
