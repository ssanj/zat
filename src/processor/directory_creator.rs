use super::DestinationFile;
use crate::error::ZatResultX;
use super::StringTokenReplacer;

/// Creates the directory specified after replacing any tokens in its name
pub trait DirectoryCreator {
  fn create_directory(&self, destination_directory: &DestinationFile, replacer: &dyn StringTokenReplacer) -> ZatResultX<()>;
}
