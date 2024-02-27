pub trait StringTokenReplacer {
  fn replace(&self, input: &str) -> String;
}

pub struct EchoingStringTokenReplacer;

impl EchoingStringTokenReplacer {

  pub fn replace(input: &str) -> String {
    input.to_owned()
  }
}

impl StringTokenReplacer for EchoingStringTokenReplacer {
  fn replace(&self, input: &str) -> String {
    Self::replace(input)
  }
}

pub struct ReplacingStringTokenReplacer<'a> {
  replacements: &'a[(&'a str, &'a str)]
}

impl <'a> ReplacingStringTokenReplacer<'a> {

  #[cfg(test)]
  pub fn new(replacements: &'a[(&'a str, &'a str)]) -> Self {
    Self {
      replacements
    }
  }

  pub fn replace(&self, input: &str) -> String {
    self.replacements
      .iter()
      .fold(input.to_owned(), |i, r| i.replace(r.0, r.1))
  }
}

impl StringTokenReplacer for ReplacingStringTokenReplacer<'_> {
    fn replace(&self, input: &str) -> String {
      self.replace(input)
    }
}
