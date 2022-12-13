use crate::{key_tokenizer::{KeyTokenizer, TokenizedExpandedKey}, template_variable_expander::ExpandedVariables};
use std::collections::HashMap;
use crate::template_variable_expander::{ExpandedKey, ExpandedValue};

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
    fn tokenize_keys(&self, expanded_variables: ExpandedVariables) -> HashMap<TokenizedExpandedKey, ExpandedValue> {
        expanded_variables
          .expanded_variables
          .into_iter()
          .map(|(k, v)|{
            let tokenized_key = format!("{}{}{}", &self.token, k.value, &self.token);
            (TokenizedExpandedKey::new(&tokenized_key), v)
          })
          .collect()
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

      assert_eq!(user_variables.len(), tokenized_keys.len(), "user_variables and tokenized_keys HashMaps should be the same size");

      user_variables.iter().for_each(|(k, v)| {
        let tokenized_key = TokenizedExpandedKey::new(&format!("${}$", k.value));
        assert_eq!(Some(v), tokenized_keys.get(&tokenized_key), "Could not find entry for key: {} in {:?}", &tokenized_key.value, &tokenized_keys)
      })
    }
}
