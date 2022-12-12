use crate::template_variable_expander::ExpandedKey;

#[derive(Debug, Clone)]
pub struct ContentWithTokens {
  pub value: String
}

impl ContentWithTokens {
  pub fn new(input: &str) -> Self {
    Self {
      value: input.to_owned()
    }
  }
}

impl From<ContentWithTokens> for ExpandedKey {
  fn from(field: ContentWithTokens) -> Self {
      ExpandedKey {
        value: field.value
      }
  }
}

impl AsRef<str> for ContentWithTokens {
  fn as_ref(&self) -> &str {
      &self.value
  }
}

pub trait TokenReplacer {
  /// content_with_tokens The content that has token to replace. Any matching tokens will be replaced and the updated content returned.
  /// If no matching tokens are found the original content will be returned
  fn replace_content_token(&self, content_with_tokens: ContentWithTokens) -> String;
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

    // Reference implementation
    impl TokenReplacer for HashMapTokenReplacer {
        fn replace_content_token(&self, content_with_token: ContentWithTokens) -> String {
          self
            .expanded_variables
            .iter()
            .fold(content_with_token.value, |acc, (k, v)|{
              acc.replace(&k.value, &v.value)
            })
        }
    }

    #[test]
    fn updates_content_tokens() {
      let user_variables =
        HashMap::from(
          [
            (ExpandedKey::new("token1"), ExpandedValue::new("replacement1")),
            (ExpandedKey::new("token2"), ExpandedValue::new("replacement2")),
            (ExpandedKey::new("token3"), ExpandedValue::new("replacement3"))
          ]
        );

      let token_replacer = HashMapTokenReplacer::new(user_variables);
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("token1")), "replacement1".to_owned());
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("token2")), "replacement2".to_owned());
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("token3")), "replacement3".to_owned());
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("This content has token1 in it twice token1")), "This content has replacement1 in it twice replacement1".to_owned());
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("All the token3 token2 token1 are present")), "All the replacement3 replacement2 replacement1 are present".to_owned());
    }

    #[test]
    fn returns_original_content_without_matching_tokens() {
      let user_variables =
        HashMap::from(
          [
            (ExpandedKey::new("token1"), ExpandedValue::new("replacement1")),
            (ExpandedKey::new("token2"), ExpandedValue::new("replacement2")),
            (ExpandedKey::new("token3"), ExpandedValue::new("replacement3"))
          ]
        );

      let token_replacer = HashMapTokenReplacer::new(user_variables);
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("This content has no tokens in it")), "This content has no tokens in it".to_owned());
    }
}


