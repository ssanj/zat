use crate::token_expander::key_tokenizer::TokenizedKeysExpandedVariables;
use super::StringTokenReplacer;
use super::{ContentWithTokens, TokenReplacer, ContentTokensReplaced};

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

pub struct AhoCorasickTokenReplacer {
  ahocorasick: AhoCorasick,
  replacements: Vec<String>
}

impl AhoCorasickTokenReplacer {
  pub fn new(tokenized_keys_expanded_values: TokenizedKeysExpandedVariables) -> Self {
      // Grab the keys and values so the orders are consistent (HashMap has inconsistent ordering)
      let mut token_keys: Vec<String> = vec![];
      let mut token_values: Vec<String> = vec![];
      for (key, value) in tokenized_keys_expanded_values.value {
        token_keys.push(key.value); // key
        token_values.push(value.value); // value
      };

      let ahocorasick =
        AhoCorasickBuilder::new()
          .match_kind(MatchKind::LeftmostLongest)
          .build(token_keys);

      Self {
        ahocorasick,
        replacements: token_values
      }
  }
}

impl TokenReplacer for AhoCorasickTokenReplacer {
    // TODO: Deprecate this
    fn replace_content_token(&self, content_with_token: ContentWithTokens) -> ContentTokensReplaced {
      ContentTokensReplaced {
        value: self.ahocorasick.replace_all(&content_with_token.value, &self.replacements)
      }
    }
}

impl StringTokenReplacer for AhoCorasickTokenReplacer {
    fn replace(&self, input: &str) -> String {
      self.ahocorasick.replace_all(input, &self.replacements)
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashMap;
  use crate::token_expander::key_tokenizer::TokenizedExpandedKey;
  use crate::token_expander::template_variable_expander::ExpandedValue;

   const PROJECT_CONTENT: &str =
      r#"
        import sublime
        import sublime_plugin
        from typing import Optional, List

        class project_PascalCommand(sublime_plugin.WindowCommand):

          def run(self) -> None:
            window = self.window

            if window:
              print("project is running") // dump out project_Pascal
            else:
              sublime.message_dialog("Could not find Window")

          def is_enabled(self) -> bool:
            return True

          def is_visible(self) -> bool:
            return True
      "#;

    // We need to define this here or match indentation when we define this in a method as raw Strings
    // respect spaces from the margin.
    const EXPECTED_PROJECT_CONTENT: &str =
      r#"
        import sublime
        import sublime_plugin
        from typing import Optional, List

        class BleeBlueCommand(sublime_plugin.WindowCommand):

          def run(self) -> None:
            window = self.window

            if window:
              print("blee blue is running") // dump out BleeBlue
            else:
              sublime.message_dialog("Could not find Window")

          def is_enabled(self) -> bool:
            return True

          def is_visible(self) -> bool:
            return True
      "#;

    #[test]
    fn returns_matched_token() {
      let user_variables =
        HashMap::from(
          [
            (TokenizedExpandedKey::new("project"), ExpandedValue::new("blee blue")),
            (TokenizedExpandedKey::new("project_Pascal"), ExpandedValue::new("BleeBlue"))
          ]
        );

      let tokenized_keys_expanded_variables =
        TokenizedKeysExpandedVariables {
          value: user_variables
        };

      let replacer = AhoCorasickTokenReplacer::new(tokenized_keys_expanded_variables);
      assert_eq!(replacer.replace_content_token(ContentWithTokens::new("project")).as_ref(), "blee blue");
    }

    #[test]
    fn returns_matched_content() {
      let user_variables =
        HashMap::from(
          [
            (TokenizedExpandedKey::new("project"), ExpandedValue::new("blee blue")),
            (TokenizedExpandedKey::new("project_Pascal"), ExpandedValue::new("BleeBlue"))
          ]
        );

      let tokenized_keys_expanded_variables =
        TokenizedKeysExpandedVariables {
          value: user_variables
        };

      let replacer = AhoCorasickTokenReplacer::new(tokenized_keys_expanded_variables);
      assert_eq!(replacer.replace_content_token(ContentWithTokens::new(PROJECT_CONTENT)).as_ref(), EXPECTED_PROJECT_CONTENT);
    }

    #[test]
    fn returns_longest_matched_token_if_overlapping() {
      let user_variables =
        HashMap::from(
          [
            (TokenizedExpandedKey::new("project"), ExpandedValue::new("blee blue")),
            (TokenizedExpandedKey::new("project_Pascal"), ExpandedValue::new("BleeBlue"))
          ]
        );

      let tokenized_keys_expanded_variables =
        TokenizedKeysExpandedVariables {
          value: user_variables
        };

      let replacer = AhoCorasickTokenReplacer::new(tokenized_keys_expanded_variables);
      // Returns "BleeBlue" instead of matching on "project" and returning "blee blue_Pascal"
      assert_eq!(replacer.replace_content_token(ContentWithTokens::new("project_Pascal")).as_ref(), "BleeBlue");
    }

    #[test]
    fn returns_token_if_match_not_found() {
      let user_variables = HashMap::new();

      let tokenized_keys_expanded_variables =
        TokenizedKeysExpandedVariables {
          value: user_variables
        };

      let replacer = AhoCorasickTokenReplacer::new(tokenized_keys_expanded_variables);
      assert_eq!(replacer.replace_content_token(ContentWithTokens::new(PROJECT_CONTENT)).as_ref(), PROJECT_CONTENT);
    }

}
