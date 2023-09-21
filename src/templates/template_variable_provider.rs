use crate::{shared_models::ZatResultX, variables::TemplateVariables};
use crate::config::UserConfigX;

/// Behaviour to return tokens defined in a template
pub trait TemplateVariableProvider {
  /// Returns the TemplateVariables
  fn get_tokens(&self, user_config: UserConfigX) -> ZatResultX<TemplateVariables>;
}
