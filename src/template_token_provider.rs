use serde::Deserialize;

use crate::{shared_models::*, user_config_provider::UserConfig, variables::TemplateVariable};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TemplateTokens {
  pub tokens: Vec<TemplateVariable>
}

/// Behaviour to return tokens defined in a template
pub trait TemplateTokenProvider {
  /// Returns the TemplateTokens
  fn get_tokens(&self, user_config: UserConfig) -> ZatResultX<TemplateTokens>;
}
