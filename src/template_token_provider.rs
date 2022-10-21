use std::collections::HashMap;
use serde::Deserialize;

use crate::{shared_models::*, user_config_provider::UserConfig, variables::TemplateVariable};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TemplateTokens {
  pub tokens: Vec<TemplateVariable>
}

/// Behaviour to return configuration provided by the "user"
pub trait TemplateTokenProvider {
  /// Returns the UserConfig
  fn get_tokens(&self, user_config: UserConfig) -> ZatResultX<TemplateTokens>;
}
