use crate::token_replacer::{Token, TokenReplacer};
use crate::template_variable_expander::{ExpandedKey, ExpandedValue};
use std::collections::HashMap;
use aho_corasick::AhoCorasickBuilder;
use aho_corasick::MatchKind::LeftmostLongest;

pub struct AhoCorasickTokenReplacer;

// TODO: We may need to move out the ashMap as we want to initialise it only once
impl TokenReplacer for AhoCorasickTokenReplacer {
    fn replace_token(&self, expanded_variables: &HashMap<ExpandedKey, ExpandedValue>, token: Token) -> String {
      // Grab the keys and values so the orders are consistent (HashMap has inconsistent ordering)
      let mut token_keys: Vec<&str> = vec![];
      let mut token_values: Vec<&str> = vec![];
      for (key, value) in expanded_variables {
        token_keys.push(&key.value); // key
        token_values.push(&value.value); // value
      };

      // TODO: This should be created once in the constructor
      let ac =
        AhoCorasickBuilder::new()
          .match_kind(LeftmostLongest)
          .build(token_keys);

      ac.replace_all(&token.value, &token_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            (ExpandedKey::new("project"), ExpandedValue::new("blee blue")),
            (ExpandedKey::new("project_Pascal"), ExpandedValue::new("BleeBlue"))
          ]
        );

      assert_eq!(AhoCorasickTokenReplacer.replace_token(&user_variables, Token::new("project")), "blee blue".to_owned());
    }

    #[test]
    fn returns_matched_content() {
      let user_variables =
        HashMap::from(
          [
            (ExpandedKey::new("project"), ExpandedValue::new("blee blue")),
            (ExpandedKey::new("project_Pascal"), ExpandedValue::new("BleeBlue"))
          ]
        );

      assert_eq!(AhoCorasickTokenReplacer.replace_token(&user_variables, Token::new(PROJECT_CONTENT)), EXPECTED_PROJECT_CONTENT.to_owned());
    }

    #[test]
    fn returns_longest_matched_token_if_overlapping() {
      let user_variables =
        HashMap::from(
          [
            (ExpandedKey::new("project"), ExpandedValue::new("blee blue")),
            (ExpandedKey::new("project_Pascal"), ExpandedValue::new("BleeBlue"))
          ]
        );

      // Returns "BleeBlue" instead of matching on "project" and returning "blee blue_Pascal"
      assert_eq!(AhoCorasickTokenReplacer.replace_token(&user_variables, Token::new("project_Pascal")), "BleeBlue".to_owned());
    }

    #[test]
    fn returns_token_if_match_not_found() {
      let user_variables = HashMap::new();
      assert_eq!(AhoCorasickTokenReplacer.replace_token(&user_variables, Token::new(PROJECT_CONTENT)), PROJECT_CONTENT.to_owned());
    }

}
