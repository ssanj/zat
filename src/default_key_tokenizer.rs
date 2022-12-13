use crate::key_tokenizer::{KeyTokenizer, TokenizedExpandedKey};
use std::collections::HashMap;
use crate::template_variable_expander::{ExpandedKey, ExpandedValue};

pub struct DefaultKeyTokenizer {
  token: String
}

impl DefaultKeyTokenizer {
  fn new(token: &str) -> Self {
    Self {
      token: token.to_owned()
    }
  }
}

impl KeyTokenizer for DefaultKeyTokenizer {
    fn tokenize_keys(&self, expanded_variables: HashMap<ExpandedKey, ExpandedValue>) -> HashMap<crate::key_tokenizer::TokenizedExpandedKey, ExpandedValue> {
        expanded_variables
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

      let key_tokenizer = DefaultKeyTokenizer::new("$");
      let tokenized_keys = key_tokenizer.tokenize_keys(user_variables.clone());

      assert_eq!(user_variables.len(), tokenized_keys.len());

      user_variables.iter().for_each(|(k, v)| {
        let tokenized_key = TokenizedExpandedKey::new(&format!("${}$", k.value));
        assert_eq!(Some(v), tokenized_keys.get(&tokenized_key), "Checking for key: {}", &tokenized_key.value)
      })
    }
}
