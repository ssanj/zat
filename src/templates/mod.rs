pub mod template_variable_provider;
pub mod default_template_variable_provider;
pub mod template_config_validator;
pub mod default_template_config_validator;
pub mod variables;
pub mod choice;
pub mod user_choices;
pub mod scope;

mod plugin;

pub use template_variable_provider::TemplateVariableProvider;
pub use default_template_variable_provider::DefaultTemplateVariableProvider;
pub use template_config_validator::{TemplateConfigValidator, ValidConfig, TemplateVariableReview};
pub use default_template_config_validator::DefaultTemplateConfigValidator;
pub use variables::{FilterType, TemplateVariable, UserVariableKey, UserVariableValue, TemplateVariables, UserChoiceKey, UserChoiceValue};

pub use plugin::{Plugin, PluginRunResult, PluginRunStatus, ArgType};
pub use choice::Choice;
pub use user_choices::UserChoices;
pub use scope::{Scope, IncludeChoiceValue};

#[cfg(test)]
pub use variables::VariableFilter;

#[cfg(test)]
pub use plugin::PluginArg;
