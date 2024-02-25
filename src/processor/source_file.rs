use std::{fmt, path::Path};
use std::fs;
use crate::error::{ZatResult, ZatError};
use crate::spath;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceFile(pub String);

// TODO: Test
// TODO: Should this be a trait?
impl SourceFile {

  #[cfg(test)]
  pub fn new(file: &str) -> Self {
    Self(file.to_owned())
  }

  pub fn read_text(&self) -> ZatResult<String> {
    fs::read(&self.0)
      .map_err(|e|{
        ZatError::could_not_read_template_file(self.0.as_str(), e.to_string())
      })
      .and_then(|content| {
        std::str::from_utf8(&content)
          .map_err(|e| {
            ZatError::template_file_content_is_unsupported(&self.0, e.to_string())
          })
          .map(|c| c.to_owned())
      })
  }

  pub fn read_binary(&self) -> ZatResult<Vec<u8>> {
    fs::read(&self.0)
      .map_err(|e|{
        ZatError::could_not_read_template_file(self.0.as_str(), e.to_string())
      })
  }

  pub fn strip_prefix<P>(&self, prefix: P)  -> ZatResult<String>
    where P: AsRef<Path>
  {
    Path::new(&self.0)
      .strip_prefix(&prefix)
      .map_err(|e|{
        ZatError::could_not_determine_base_path_of_template_file(spath!(prefix.as_ref()), &self.0, e.to_string())
      })
      .map(|p| p.to_string_lossy().to_string())
  }
}

impl fmt::Display for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<Path> for SourceFile {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
  }
}
