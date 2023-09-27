use crate::error::ZatResult;
use crate::config::UserConfig;
use super::TemplateVariables;

/// Behaviour to return tokens defined in a template
pub trait TemplateVariableProvider {
  /// Returns the TemplateVariables
  fn get_tokens(&self, user_config: UserConfig) -> ZatResult<TemplateVariables>;
}
