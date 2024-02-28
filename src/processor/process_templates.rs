use crate::{error::ZatAction, token_expander::key_tokenizer::TokenizedKeysExpandedVariables};
use crate::config::UserConfig;
use crate::templates::UserChoices;

pub trait ProcessTemplates {
  fn process_templates(&self, user_config: UserConfig, tokenized_key_expanded_variables: TokenizedKeysExpandedVariables, user_choices: UserChoices) -> ZatAction;
}
