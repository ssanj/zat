use super::DestinationFile;
use super::SourceFile;
use crate::error::ZatResultX;
use super::StringTokenReplacer;

#[derive(PartialEq, Debug)]
pub enum EnrichedTemplateFile {
  File(SourceFile, DestinationFile),
  Dir(DestinationFile),
}

pub trait EnrichedTemplateFileProcessor {
  fn process_enriched_template_files(&self, template_files: &[EnrichedTemplateFile], replacer: &dyn StringTokenReplacer) -> ZatResultX<()>;
  }
