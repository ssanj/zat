use crate::{enriched_template_file_processor::{EnrichedTemplateFileProcessor, EnrichedTemplateFile}, shared_models::{ZatResultX, ZatErrorX}, file_traverser::TemplateFile, file_writer::FileWriter, directory_creator::DirectoryCreator, default_file_writer::DefaultFileWriter, default_directory_creator::DefaultDirectoryCreator, destination_file, string_token_replacer::StringTokenReplacer};

pub struct DefaultEnrichedTemplateFileProcessor {
  file_writer: DefaultFileWriter,
  directory_creator: DefaultDirectoryCreator
}

impl DefaultEnrichedTemplateFileProcessor {
  pub fn new() -> Self {
    Self {
      file_writer: DefaultFileWriter,
      directory_creator: DefaultDirectoryCreator
    }
  }
}

impl EnrichedTemplateFileProcessor for DefaultEnrichedTemplateFileProcessor {

  fn process_enriched_template_files<T>(&self, template_files: &[EnrichedTemplateFile], replacer: &dyn StringTokenReplacer) -> ZatResultX<()> {

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
