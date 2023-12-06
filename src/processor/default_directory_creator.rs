use super::DirectoryCreator;
use super::DestinationFile;
use crate::config::UserConfig;
use crate::error::{ZatError, ZatResult};
use crate::logging::VerboseLogger;
use super::StringTokenReplacer;
use std::{fs, format as s};

pub struct DefaultDirectoryCreator<'a> {
  user_config: &'a UserConfig
}

impl <'a> DefaultDirectoryCreator<'a> {
  pub fn with_user_config(user_config: &'a UserConfig) -> Self {
    Self {
      user_config
    }
  }
}

impl DirectoryCreator for DefaultDirectoryCreator<'_> {
    fn create_directory(&self, destination_directory: &DestinationFile, replacer: &dyn StringTokenReplacer) -> ZatResult<()> {

      let directory_path_with_tokens_replaced = destination_directory.map(|dd| replacer.replace(dd));
      VerboseLogger::log_content(self.user_config, &s!("Creating directory: {}", &directory_path_with_tokens_replaced));

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
      let user_config = UserConfig::default();
      let directory_creator = DefaultDirectoryCreator::with_user_config(&user_config);

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
      let user_config = UserConfig::default();
      let directory_creator = DefaultDirectoryCreator::with_user_config(&user_config);

      let working_directory = DestinationFile::from(tmp_directory.into_path());
      let destination_directory: DestinationFile = working_directory.join("some-$project$");
      let destination_directory_with_tokens_replaced: DestinationFile = working_directory.join("some-cool-project");

      assert!(!&destination_directory_with_tokens_replaced.as_ref().exists(), "destination directory: {} should not exist before creation", &destination_directory_with_tokens_replaced);


      let replacer = ReplacingStringTokenReplacer::new(&[("$project$", "cool-project")]);

      directory_creator.create_directory(&destination_directory, &replacer).unwrap();

      assert!(&destination_directory_with_tokens_replaced.as_ref().exists(), "destination directory: {} does not exist", &destination_directory_with_tokens_replaced)
    }
}
