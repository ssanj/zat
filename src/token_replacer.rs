#[derive(Debug, Clone)]
pub struct Token(String);

impl AsRef<str> for Token {
  fn as_ref(&self) -> &str {
      &self.0.as_str()
  }
}

pub trait TokenReplacer {
  fn replace_token<T>(token: T) -> String where
    T: AsRef<str>;
}
