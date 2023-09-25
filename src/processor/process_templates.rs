use crate::{config::UserConfig, shared_models::ZatActionX, token_expander::key_tokenizer::TokenizedKeysExpandedVariables};

pub trait ProcessTemplates {
  fn process_templates(&self, user_config: UserConfig, tokenized_key_expanded_variables: TokenizedKeysExpandedVariables) -> ZatActionX;
}
