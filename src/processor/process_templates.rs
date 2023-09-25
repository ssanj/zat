use crate::{shared_models::ZatActionX, token_expander::key_tokenizer::TokenizedKeysExpandedVariables};
use crate::config::user_config::UserConfig;

pub trait ProcessTemplates {
  fn process_templates(&self, user_config: UserConfig, tokenized_key_expanded_variables: TokenizedKeysExpandedVariables) -> ZatActionX;
}
