use super::destination_file::DestinationFile;
use crate::shared_models::ZatResultX;
use super::string_token_replacer::StringTokenReplacer;

/// Creates the directory specified after replacing any tokens in its name
pub trait DirectoryCreator {
  fn create_directory(&self, destination_directory: &DestinationFile, replacer: &dyn StringTokenReplacer) -> ZatResultX<()>;
}
