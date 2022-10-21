use std::collections::HashMap;
use crate::{shared_models::*, user_config_provider::UserConfig};

pub struct TemplateTokens {
  pub tokens: Vec<String>
}

/// Behaviour to return configuration provided by the "user"
pub trait TemplateTokenProvider {
  /// Returns the UserConfig
  fn get_tokens(&self, user_config: UserConfig) -> ZatResultX<TemplateTokens>;
}
