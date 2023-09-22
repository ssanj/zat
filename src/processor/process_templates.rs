use crate::{config::UserConfigX, shared_models::ZatActionX, token_expander::key_tokenizer::TokenizedKeysExpandedVariables};

pub trait ProcessTemplates {
  fn process_templates(&self, user_config: UserConfigX, tokenized_key_expanded_variables: TokenizedKeysExpandedVariables) -> ZatActionX;
}
