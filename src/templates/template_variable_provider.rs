use crate::error::ZatResultX;
use crate::config::user_config::UserConfig;
use super::variables::TemplateVariables;

/// Behaviour to return tokens defined in a template
pub trait TemplateVariableProvider {
  /// Returns the TemplateVariables
  fn get_tokens(&self, user_config: UserConfig) -> ZatResultX<TemplateVariables>;
}
