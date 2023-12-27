use crate::{error::ZatResult, logging::VerboseLogger, config::UserConfig};
use super::{EnrichedTemplateFileProcessor, EnrichedTemplateFile, FileWriter, DirectoryCreator, StringTokenReplacer};

pub struct DefaultEnrichedTemplateFileProcessor<'a> {
  file_writer: &'a dyn FileWriter,
  directory_creator: &'a dyn DirectoryCreator,
  user_config: &'a UserConfig
}

impl <'a> DefaultEnrichedTemplateFileProcessor<'a> {

  pub fn new(file_writer: &'a dyn FileWriter, directory_creator: &'a dyn DirectoryCreator, user_config: &'a UserConfig)-> Self {
      Self {
        file_writer,
        directory_creator,
        user_config
      }
  }
}

impl EnrichedTemplateFileProcessor for DefaultEnrichedTemplateFileProcessor<'_> {

  fn process_enriched_template_files(&self, template_files: &[EnrichedTemplateFile], replacer: &dyn StringTokenReplacer) -> ZatResult<()> {

    VerboseLogger::log_header(self.user_config, "Files being processed");

    let results: ZatResult<()> =
      template_files
        .iter()
        .map(|f|{
          match f {
              EnrichedTemplateFile::File(source_file, destination_file) => self.file_writer.write_source_to_destination(source_file, destination_file, replacer),
              EnrichedTemplateFile::Dir(destination_file) => self.directory_creator.create_directory(destination_file, replacer),
          }
        })
        .collect();

    results
  }
}

#[cfg(test)]
mod tests {
    use std::{vec, unimplemented};

    use super::*;
    use super::super::SourceFile;
    use super::super::DestinationFile;
    use pretty_assertions::assert_eq;
    use crate::error::template_processing_error_reason::TemplateProcessingErrorReason;
    use crate::error::error::{ZatError, ProcessCommandErrorReason};
    use std::{format as s};

    struct Succeeding;

    struct Failing<'a> {
      source_files: &'a [&'a SourceFile],
      destination_files: &'a [&'a DestinationFile]
    }

    struct NotImplemented;

    impl FileWriter for Succeeding {

      fn write_source_to_destination(&self, _source_file: &SourceFile, _destination_file: &DestinationFile, _token_replacer: &dyn StringTokenReplacer) -> ZatResult<()> {
        Ok(())
      }
    }

    impl DirectoryCreator for Succeeding {

      fn create_directory(&self, _destination_directory: &DestinationFile, _replacer: &dyn StringTokenReplacer) -> ZatResult<()> {
        Ok(())
      }
    }

    impl StringTokenReplacer for NotImplemented {
      fn replace(&self, _input: &str) -> String {
        unimplemented!()
      }
    }

    impl <'a> Failing<'a> {
      pub fn files(source_files: &'a [&'a SourceFile]) -> Self {
        Self {
          source_files,
          destination_files: &[]
        }
      }

      pub fn directories(destination_files: &'a [&'a DestinationFile]) -> Self {
        Self {
          source_files: &[],
          destination_files
        }
      }
    }

    impl <'a> FileWriter for Failing<'a> {
      fn write_source_to_destination(&self, source_file: &SourceFile, _destination_file: &DestinationFile, _token_replacer: &dyn StringTokenReplacer) -> ZatResult<()> {
        if self.source_files.contains(&source_file) {
          Err(
            ZatError::ProcessCommandError(
              ProcessCommandErrorReason::TemplateProcessingError(
                TemplateProcessingErrorReason::WritingFileError(s!("Could not write file: {}", source_file), None, "".to_string())
              )
            )
          )
        } else {
          Ok(())
        }
      }
    }

    impl <'a> DirectoryCreator for Failing<'a> {
      fn create_directory(&self, destination_directory: &DestinationFile, _replacer: &dyn StringTokenReplacer) -> ZatResult<()> {
        if self.destination_files.contains(&destination_directory) {
          Err(
            ZatError::ProcessCommandError(
              ProcessCommandErrorReason::TemplateProcessingError(
                TemplateProcessingErrorReason::WritingFileError(s!("Could not write file: {}", destination_directory), None, "".to_string())
              )
            )
          )
        } else {
          Ok(())
        }
      }
    }

    #[test]
    fn handles_successes() {
      let file_writer = Succeeding;
      let directory_creator = Succeeding;

      let source_file_1 = SourceFile::new("some/source/file1");
      let destination_file_1 = DestinationFile::new("some/destination/dir1");

      let source_file_2 = SourceFile::new("some/source/file2");
      let destination_file_2: DestinationFile = DestinationFile::new("some/destination/dir2");

      let destination_file_3 = DestinationFile::new("some/destination/dir3");


      let enriched_templates =
        vec![
          EnrichedTemplateFile::File(source_file_1, destination_file_1),
          EnrichedTemplateFile::Dir(destination_file_3),
          EnrichedTemplateFile::File(source_file_2, destination_file_2),
        ];

      let token_replacer = NotImplemented;

      let user_config = UserConfig::default();
      let template_processor = DefaultEnrichedTemplateFileProcessor::new(&file_writer, &directory_creator, &user_config);
      let result = template_processor.process_enriched_template_files(&enriched_templates, &token_replacer);

      assert_eq!(result, Ok(()))
    }

    #[test]
    fn handles_failures() {
      let source_file_1 = SourceFile::new("some/source/file1");
      let destination_file_1 = DestinationFile::new("some/destination/dir1");

      let source_file_2 = SourceFile::new("some/source/file2");
      let source_file_3 = SourceFile::new("some/source/file3");
      let source_file_4 = SourceFile::new("some/source/file4");

      let destination_file_2: DestinationFile = DestinationFile::new("some/destination/dir2");
      let destination_file_3 = DestinationFile::new("some/destination/dir3");
      let destination_file_4 = DestinationFile::new("some/destination/dir4");


      let failing_source_files = [&source_file_2, &source_file_3];
      let file_writer = Failing::files(&failing_source_files);

      let failing_destination_files = [&destination_file_2, &destination_file_4];
      let directory_creator = Failing::directories(&failing_destination_files);

      let enriched_templates =
        vec![
          EnrichedTemplateFile::File(source_file_1, destination_file_1),
          EnrichedTemplateFile::Dir(destination_file_2.clone()),
          EnrichedTemplateFile::File(source_file_2.clone(), destination_file_2.clone()),
          EnrichedTemplateFile::File(source_file_3.clone(), destination_file_4.clone()),
          EnrichedTemplateFile::Dir(destination_file_3.clone()),
          EnrichedTemplateFile::Dir(destination_file_4.clone()),
          EnrichedTemplateFile::File(source_file_4.clone(), destination_file_4.clone()),
        ];

      let token_replacer = NotImplemented;

      let user_config = UserConfig::default();
      let template_processor = DefaultEnrichedTemplateFileProcessor::new(&file_writer, &directory_creator, &user_config);
      let result = template_processor.process_enriched_template_files(&enriched_templates, &token_replacer);

      let expected_errors =
        ZatError::ProcessCommandError(
          ProcessCommandErrorReason::TemplateProcessingError(
            TemplateProcessingErrorReason::WritingFileError("Could not write file: some/destination/dir2".to_owned(), None, "".to_string())
          )
        );

      assert_eq!(result, Err(expected_errors))
    }

}
