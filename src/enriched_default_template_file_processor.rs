use crate::{enriched_template_file_processor::{EnrichedTemplateFileProcessor, EnrichedTemplateFile}, shared_models::{ZatResultX, ZatErrorX}, file_traverser::TemplateFile, file_writer::FileWriter, directory_creator::DirectoryCreator, default_file_writer::DefaultFileWriter, default_directory_creator::DefaultDirectoryCreator, destination_file, string_token_replacer::StringTokenReplacer};

pub struct DefaultEnrichedTemplateFileProcessor<'a> {
  file_writer: &'a dyn FileWriter,
  directory_creator: &'a dyn DirectoryCreator
}

impl <'a> DefaultEnrichedTemplateFileProcessor<'a> {
  pub fn new(file_writer: &'a dyn FileWriter, directory_creator: &'a dyn DirectoryCreator) -> Self {
    Self {
      file_writer,
      directory_creator
    }
  }

  pub fn with_defaults() -> Self {
    Self {
      file_writer: &DefaultFileWriter,
      directory_creator: &DefaultDirectoryCreator
    }
  }
}

impl EnrichedTemplateFileProcessor for DefaultEnrichedTemplateFileProcessor<'_> {

  fn process_enriched_template_files(&self, template_files: &[EnrichedTemplateFile], replacer: &dyn StringTokenReplacer) -> ZatResultX<()> {

    let results: Vec<Result<(), ZatErrorX>> =
      template_files
        .iter()
        .map(|f|{
          match f {
              EnrichedTemplateFile::File(source_file, destination_file) => self.file_writer.write_source_to_destination(source_file, destination_file, replacer),
              EnrichedTemplateFile::Dir(destination_file) => self.directory_creator.create_directory(destination_file, replacer),
          }
        })
        .collect();

     let errors: Vec<ZatErrorX> =
       results
        .into_iter()
        .filter_map(|r| r.err())
        .collect();

     if !errors.is_empty() {
      Err(ZatErrorX::MultipleErrors(errors))
     } else {
      Ok(())
     }
  }
}

#[cfg(test)]
mod tests {
    use std::{todo, cell::Cell, vec, unimplemented};

    use super::*;
    use crate::{source_file::SourceFile, string_token_replacer::EchoingStringTokenReplacer, directory_creator};
    use destination_file::DestinationFile;
    use pretty_assertions::assert_eq;

    struct Succeeding;

    struct Failing<'a> {
      source_files: &'a [&'a SourceFile],
      destination_files: &'a [&'a DestinationFile]
    }

    struct NotImplemented;

    impl FileWriter for Succeeding {

      fn write_source_to_destination(&self, source_file: &SourceFile, destination_file: &DestinationFile, token_replacer: &dyn StringTokenReplacer) -> ZatResultX<()> {
        Ok(())
      }
    }

    impl DirectoryCreator for Succeeding {

      fn create_directory(&self, destination_directory: &DestinationFile, replacer: &dyn StringTokenReplacer) -> ZatResultX<()> {
        Ok(())
      }
    }

    impl StringTokenReplacer for NotImplemented {
      fn replace(&self, input: &str) -> String {
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
      fn write_source_to_destination(&self, source_file: &SourceFile, destination_file: &DestinationFile, token_replacer: &dyn StringTokenReplacer) -> ZatResultX<()> {
        if self.source_files.contains(&source_file) {
          Err(ZatErrorX::WritingFileError(format!("Could not write file: {}", source_file)))
        } else {
          Ok(())
        }
      }
    }

    impl <'a> DirectoryCreator for Failing<'a> {
      fn create_directory(&self, destination_directory: &DestinationFile, replacer: &dyn StringTokenReplacer) -> ZatResultX<()> {
        if self.destination_files.contains(&destination_directory) {
          Err(ZatErrorX::WritingFileError(format!("Could not write file: {}", destination_directory)))
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

      let template_processor = DefaultEnrichedTemplateFileProcessor::new(&file_writer, &directory_creator);
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

      let template_processor = DefaultEnrichedTemplateFileProcessor::new(&file_writer, &directory_creator);
      let result = template_processor.process_enriched_template_files(&enriched_templates, &token_replacer);

      let expected_errors =
        ZatErrorX::MultipleErrors(
          vec![
            ZatErrorX::WritingFileError("Could not write file: some/destination/dir2".to_owned()),
            ZatErrorX::WritingFileError("Could not write file: some/source/file2".to_owned()),
            ZatErrorX::WritingFileError("Could not write file: some/source/file3".to_owned()),
            ZatErrorX::WritingFileError("Could not write file: some/destination/dir4".to_owned()),
          ]
        );

      assert_eq!(result, Err(expected_errors))
    }

}
