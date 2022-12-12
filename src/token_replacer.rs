use crate::template_variable_expander::ExpandedKey;

#[derive(Debug, Clone)]
pub struct Token {
  pub value: String
}

// TODO: Rename, this is not so much a token as it is content
impl Token {
  pub fn new(input: &str) -> Self {
    Self {
      value: input.to_owned()
    }
  }
}

impl From<Token> for ExpandedKey {
  fn from(field: Token) -> Self {
      ExpandedKey {
        value: field.value
      }
  }
}

impl AsRef<str> for Token {
  fn as_ref(&self) -> &str {
      &self.value
  }
}

pub trait TokenReplacer {
  /// expanded_variables The key/values specified in the .variables file, expanded to include filters
  /// token The token to replace. If the token is found, then a replacement will be returned, if not the original value will be returned
  fn replace_token(&self, token: Token) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::template_variable_expander::ExpandedValue;

    struct HashMapTokenReplacer {
      expanded_variables: HashMap<ExpandedKey, ExpandedValue>
    }

    impl HashMapTokenReplacer {
      fn new(expanded_variables: HashMap<ExpandedKey, ExpandedValue>) -> Self {
        Self {
          expanded_variables
        }
      }
    }

    impl TokenReplacer for HashMapTokenReplacer {
        fn replace_token(&self, token: Token) -> String {
          self
            .expanded_variables
            .get(&ExpandedKey::from(token.clone()))
            .map(|t| t.value.clone())
            .unwrap_or_else(|| token.value)
        }
    }

    #[test]
    fn returns_matched_token() {
      let user_variables =
        HashMap::from(
          [
            (ExpandedKey::new("key1"), ExpandedValue::new("value1")),
            (ExpandedKey::new("key2"), ExpandedValue::new("value2")),
            (ExpandedKey::new("key3"), ExpandedValue::new("value3"))
          ]
        );

      let token_replacer = HashMapTokenReplacer::new(user_variables);
      assert_eq!(token_replacer.replace_token(Token::new("key1")), "value1".to_owned());
      assert_eq!(token_replacer.replace_token(Token::new("key2")), "value2".to_owned());
      assert_eq!(token_replacer.replace_token(Token::new("key3")), "value3".to_owned());
    }

    #[test]
    fn returns_token_if_match_not_found() {
      let user_variables =
        HashMap::from(
          [
            (ExpandedKey::new("key1"), ExpandedValue::new("value1")),
            (ExpandedKey::new("key2"), ExpandedValue::new("value2")),
            (ExpandedKey::new("key3"), ExpandedValue::new("value3"))
          ]
        );

      let token_replacer = HashMapTokenReplacer::new(user_variables);
      assert_eq!(token_replacer.replace_token(Token::new("key4")), "key4".to_owned());
    }
}


