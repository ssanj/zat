use crate::destination_file::DestinationFile;
use crate::file_traverser::TemplateFile;
use crate::models::SourceFile;
use crate::shared_models::ZatResultX;

pub enum EnrichedTemplateFile {
  File(SourceFile, DestinationFile),
  Dir(DestinationFile),
}

pub trait EnrichedTemplateFileProcessor {
  fn process_enriched_template_files<T>(&self, template_files: &[EnrichedTemplateFile], replacer: T) -> ZatResultX<()>
    where T: Fn(&str) -> String;
  }
