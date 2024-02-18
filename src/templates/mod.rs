pub mod template_variable_provider;
pub mod default_template_variable_provider;
pub mod template_config_validator;
pub mod default_template_config_validator;
pub mod variables;

mod plugin;

pub use template_variable_provider::TemplateVariableProvider;
pub use default_template_variable_provider::DefaultTemplateVariableProvider;
pub use template_config_validator::{TemplateConfigValidator, ValidConfig, TemplateVariableReview};
pub use default_template_config_validator::DefaultTemplateConfigValidator;
pub use variables::{FilterType, TemplateVariable, UserVariableKey, UserVariableValue, TemplateVariables};

pub use plugin::{Plugin, PluginRunResult, PluginRunStatus, ArgType};

#[cfg(test)]
pub use variables::VariableFilter;
