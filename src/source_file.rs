use std::path::PathBuf;
use std::{fmt, path::Path, ffi::OsStr, borrow::Cow};
use std::fs;
use crate::shared_models::{ZatResultX, ZatErrorX};

#[derive(Debug, Clone)]
pub struct SourceFile(pub String);

// TODO: Test
// TODO: Should this be a trait?
impl SourceFile {

  pub fn read(&self) -> ZatResultX<String> {
    fs::read(&self.0)
      .map_err(|e|{
        ZatErrorX::ReadingFileError(format!("Could not read source file: {}\nCause: {}", self.0.as_str(), e.to_string()))
      })
      .and_then(|content| {
        std::str::from_utf8(&content)
          .map_err(|e| {
            ZatErrorX::ReadingFileError(
              format!("Could not convert content of {} from bytes to String:\n{}",
                &self.0,
                e.to_string())
              )
          })
          .map(|c| c.to_owned())
      })
  }

  pub fn strip_prefix(&self, prefix: &str)  -> ZatResultX<String> {
    (&self.0).strip_prefix(prefix)
    .ok_or_else(||{
      ZatErrorX::ReadingFileError(format!("Could remove path prefix: {} from directory: {}", prefix, &self.0))
    })
    .map(|p| p.to_owned())
  }
}

impl fmt::Display for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<Path> for SourceFile {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.0)
  }
}
