use crate::destination_file::DestinationFile;
use crate::file_traverser::TemplateFile;
use crate::source_file::SourceFile;
use crate::shared_models::ZatResultX;

#[derive(PartialEq, Debug)]
pub enum EnrichedTemplateFile {
  File(SourceFile, DestinationFile),
  Dir(DestinationFile),
}

pub trait EnrichedTemplateFileProcessor {
  fn process_enriched_template_files<T>(&self, template_files: &[EnrichedTemplateFile], replacer: T) -> ZatResultX<()>
    where T: Fn(&str) -> String;
  }
