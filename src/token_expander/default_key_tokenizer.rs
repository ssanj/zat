use crate::token_expander::{key_tokenizer::{KeyTokenizer, TokenizedExpandedKey, TokenizedKeysExpandedVariables}, template_variable_expander::ExpandedVariables};
use std::collections::HashMap;
use crate::token_expander::template_variable_expander::{ExpandedKey, ExpandedValue};

pub struct DefaultKeyTokenizer {
  token: String
}

impl DefaultKeyTokenizer {
  pub fn new(token: &str) -> Self {
    Self {
      token: token.to_owned()
    }
  }
}

impl KeyTokenizer for DefaultKeyTokenizer {
    fn tokenize_keys(&self, expanded_variables: ExpandedVariables) -> TokenizedKeysExpandedVariables {
        let value =
          expanded_variables
            .expanded_variables
            .into_iter()
            .map(|(k, v)|{
              let tokenized_key = format!("{}{}{}", &self.token, k.value, &self.token); // Surround keys with the token delimiter;  usually `$`
              (TokenizedExpandedKey::new(&tokenized_key), v)
            })
            .collect();

        TokenizedKeysExpandedVariables {
          value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_token_to_all_keys() {
      let user_variables =
        HashMap::from(
          [
            (ExpandedKey::new("project"), ExpandedValue::new("blee blue")),
            (ExpandedKey::new("project_Pascal"), ExpandedValue::new("BleeBlue"))
          ]
        );

      let expanded_variables =
        ExpandedVariables {
          expanded_variables: user_variables.clone()
        };

      let key_tokenizer = DefaultKeyTokenizer::new("$");
      let tokenized_keys = key_tokenizer.tokenize_keys(expanded_variables);

      assert_eq!(user_variables.len(), tokenized_keys.value.len(), "user_variables and tokenized_keys HashMaps should be the same size");

      user_variables.iter().for_each(|(k, v)| {
        let tokenized_key = TokenizedExpandedKey::new(&format!("${}$", k.value));
        assert_eq!(Some(v), tokenized_keys.value.get(&tokenized_key), "Could not find entry for key: {} in {:?}", &tokenized_key.value, &tokenized_keys)
      })
    }
}
