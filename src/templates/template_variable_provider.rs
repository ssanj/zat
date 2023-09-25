use crate::{shared_models::ZatResultX, variables::TemplateVariables};
use crate::config::UserConfig;

/// Behaviour to return tokens defined in a template
pub trait TemplateVariableProvider {
  /// Returns the TemplateVariables
  fn get_tokens(&self, user_config: UserConfig) -> ZatResultX<TemplateVariables>;
}
