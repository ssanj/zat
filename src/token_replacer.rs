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


impl AsRef<str> for ContentWithTokens {
  fn as_ref(&self) -> &str {
      &self.value
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContentTokensReplaced {
  pub value: String
}

impl ContentTokensReplaced {
  pub fn new(input: &str) -> Self {
    Self {
      value: input.to_owned()
    }
  }
}

impl AsRef<str> for ContentTokensReplaced {
  fn as_ref(&self) -> &str {
      &self.value
  }
}

pub trait TokenReplacer {
  /// content_with_tokens The content that has token to replace. Any matching tokens will be replaced and the updated content returned.
  /// If no matching tokens are found the original content will be returned
  fn replace_content_token(&self, content_with_tokens: ContentWithTokens) -> ContentTokensReplaced; // TODO: Should this be strongly typed?
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::{template_variable_expander::ExpandedValue, key_tokenizer::TokenizedExpandedKey};

    struct HashMapTokenReplacer {
      expanded_variables: HashMap<TokenizedExpandedKey, ExpandedValue>
    }

    impl HashMapTokenReplacer {
      fn new(expanded_variables: HashMap<TokenizedExpandedKey, ExpandedValue>) -> Self {
        Self {
          expanded_variables
        }
      }
    }

    // Reference implementation
    impl TokenReplacer for HashMapTokenReplacer {
        fn replace_content_token(&self, content_with_token: ContentWithTokens) -> ContentTokensReplaced {
          let with_tokens_replaced =
            self
              .expanded_variables
              .iter()
              .fold(content_with_token.value, |acc: String, (k, v):(&TokenizedExpandedKey, &ExpandedValue)|{
                acc.replace(&k.value, &v.value)
              });

          ContentTokensReplaced::new(&with_tokens_replaced)
        }
    }

    #[test]
    fn updates_content_tokens() {
      let user_variables =
        HashMap::from(
          [
            (TokenizedExpandedKey::new("token1"), ExpandedValue::new("replacement1")),
            (TokenizedExpandedKey::new("token2"), ExpandedValue::new("replacement2")),
            (TokenizedExpandedKey::new("token3"), ExpandedValue::new("replacement3"))
          ]
        );

      let token_replacer = HashMapTokenReplacer::new(user_variables);
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("token1")).as_ref(), "replacement1");
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("token2")).as_ref(), "replacement2");
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("token3")).as_ref(), "replacement3");
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("This content has token1 in it twice token1")).as_ref(), "This content has replacement1 in it twice replacement1");
      assert_eq!(token_replacer.replace_content_token(ContentWithTokens::new("All the token3 token2 token1 are present")).as_ref(), "All the replacement3 replacement2 replacement1 are present");
    }

    #[test]
    fn returns_original_content_without_matching_tokens() {
      let user_variables =
        HashMap::from(
          [
            (TokenizedExpandedKey::new("token1"), ExpandedValue::new("replacement1")),
            (TokenizedExpandedKey::new("token2"), ExpandedValue::new("replacement2")),
            (TokenizedExpandedKey::new("token3"), ExpandedValue::new("replacement3"))
          ]
        );

      let token_replacer = HashMapTokenReplacer::new(user_variables);
      assert_eq!(
        token_replacer.replace_content_token(ContentWithTokens::new("This content has no tokens in it")).as_ref(),
        "This content has no tokens in it"
      )
    }
}


