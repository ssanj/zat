use std::borrow::Cow;
use std::path::PathBuf;
use std::{fmt, fmt::Display, path::Path};
use std::fs;

#[derive(Debug, Clone)]
pub struct DestinationFile(pub String);

impl DestinationFile {

  pub fn get_extension(&self) -> Option<Cow<'_, str>> {
     Path::new(&self.0)
      .extension()
      .map(|p| p.to_string_lossy().to_owned())
  }

  pub fn remove_extension(&self) -> DestinationFile {
    let without_extension =
      Path::new(&self.0)
        .file_stem()
        .expect(&format!("Could not retrieve file name stem for: {}", &self.0))
        .to_string_lossy();

    DestinationFile(without_extension.to_string())
  }

  pub fn parent_directory(&self) -> DestinationFile {
    let parent_dir =
      Path::new(&self.0)
        .parent()
        .expect(&format!("Could not get parent path for: {}", &self.0))
        .to_string_lossy();

    DestinationFile(parent_dir.to_string())
  }

  pub fn file_stem(&self) -> DestinationFile {
    let file_stem =
      Path::new(&self.0)
        .file_stem()
        .expect(&format!("Could not get file stem for: {}", &self.0))
        .to_string_lossy();

    DestinationFile(file_stem.to_string())
  }

  pub fn join<P>(&self, other: P) -> DestinationFile where
    P: AsRef<Path>
  {
    DestinationFile(Path::new(&self.0).join(other).to_string_lossy().to_string())
  }

  pub fn map<F>(&self, f: F) -> DestinationFile where
    F: Fn(&str) -> String
  {
    DestinationFile(f(&self.0))
  }
}

impl From<PathBuf> for DestinationFile {
  fn from(p: PathBuf) -> Self {
      DestinationFile(p.to_string_lossy().to_string())
  }
}


impl AsRef<Path> for DestinationFile {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.0)
  }
}

impl Display for DestinationFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
    }
}
