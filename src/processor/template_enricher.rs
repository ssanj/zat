use crate::shared_models::ZatResultX;
use super::{file_traverser::TemplateFile, enriched_template_file_processor::EnrichedTemplateFile};

pub trait TemplateEnricher {
  fn enrich(&self, template_file: TemplateFile) -> ZatResultX<EnrichedTemplateFile>;
}
