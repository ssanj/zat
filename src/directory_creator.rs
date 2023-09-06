use crate::destination_file::DestinationFile;
use crate::shared_models::ZatResultX;

/// Creates the directory specified after replacing any tokens in its name
pub trait DirectoryCreator {
  fn create_directory<T>(&self, destination_directory: &DestinationFile, replacer: T) -> ZatResultX<()>
    where T: Fn(&str) -> String;
}
