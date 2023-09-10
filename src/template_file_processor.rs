use crate::destination_file::DestinationFile;
use crate::file_traverser::TemplateFile;
use crate::models::SourceFile;
use crate::shared_models::ZatResultX;

enum EnrichedTemplateFile {
  File(SourceFile, DestinationFile),
  Dir(DestinationFile),
}

pub trait TemplateFileProcessor {
  fn process_template_files<T>(&self, template_files: &[TemplateFile], replacer: T) -> ZatResultX<()>
    where T: Fn(&str) -> String;
  }
