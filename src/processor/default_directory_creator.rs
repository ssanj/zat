use super::DirectoryCreator;
use super::DestinationFile;
use crate::error::{ZatError, ZatResult};
use super::StringTokenReplacer;
use std::fs;

pub struct DefaultDirectoryCreator;

impl DirectoryCreator for DefaultDirectoryCreator {
    fn create_directory(&self, destination_directory: &DestinationFile, replacer: &dyn StringTokenReplacer) -> ZatResult<()> {

      let directory_path_with_tokens_replaced = destination_directory.map(|dd| replacer.replace(dd));

      fs::create_dir(&directory_path_with_tokens_replaced)
        .map_err(|e| {
          ZatError::could_not_create_output_file_directory(&directory_path_with_tokens_replaced.0.as_str(), e.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use super::super::{EchoingStringTokenReplacer, ReplacingStringTokenReplacer};

    #[test]
    fn creates_supplied_directory() {
      let tmp_directory = TempDir::new().unwrap();
      let directory_creator = DefaultDirectoryCreator;

      let working_directory = DestinationFile::from(tmp_directory.into_path());
      let destination_directory: DestinationFile = working_directory.join("some-cool-directory");

      assert!(!&destination_directory.as_ref().exists(), "destination directory: {} should not exist before creation", &destination_directory);
      let replacer = EchoingStringTokenReplacer;

      directory_creator.create_directory(&destination_directory, &replacer).unwrap();

      assert!(&destination_directory.as_ref().exists(), "destination directory: {} does not exist", &destination_directory)
    }

    #[test]
    fn creates_supplied_directory_after_replacing_tokens() {
      let tmp_directory = TempDir::new().unwrap();
      let directory_creator = DefaultDirectoryCreator;

      let working_directory = DestinationFile::from(tmp_directory.into_path());
      let destination_directory: DestinationFile = working_directory.join("some-$project$");
      let destination_directory_with_tokens_replaced: DestinationFile = working_directory.join("some-cool-project");

      assert!(!&destination_directory_with_tokens_replaced.as_ref().exists(), "destination directory: {} should not exist before creation", &destination_directory_with_tokens_replaced);


      let replacer = ReplacingStringTokenReplacer::new(&[("$project$", "cool-project")]);

      directory_creator.create_directory(&destination_directory, &replacer).unwrap();

      assert!(&destination_directory_with_tokens_replaced.as_ref().exists(), "destination directory: {} does not exist", &destination_directory_with_tokens_replaced)
    }
}
