use std::todo;

use crate::{user_config_provider::UserConfigX, enriched_template_file_processor::{EnrichedTemplateFileProcessor, EnrichedTemplateFile}, shared_models::ZatResultX, file_traverser::TemplateFile, file_writer::FileWriter, directory_creator::DirectoryCreator, default_file_writer::DefaultFileWriter, default_directory_creator::DefaultDirectoryCreator};

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

  fn process_enriched_template_files<T>(&self, template_files: &[EnrichedTemplateFile], replacer: T) -> ZatResultX<()>
    where T: Fn(&str) -> String {
    todo!()
  }
}
