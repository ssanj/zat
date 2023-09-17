use crate::destination_file::DestinationFile;
use crate::file_traverser::TemplateFile;
use crate::source_file::SourceFile;
use crate::shared_models::ZatResultX;
use crate::string_token_replacer::StringTokenReplacer;

#[derive(PartialEq, Debug)]
pub enum EnrichedTemplateFile {
  File(SourceFile, DestinationFile),
  Dir(DestinationFile),
}

pub trait EnrichedTemplateFileProcessor {
  fn process_enriched_template_files(&self, template_files: &[EnrichedTemplateFile], replacer: &dyn StringTokenReplacer) -> ZatResultX<()>;
  }
