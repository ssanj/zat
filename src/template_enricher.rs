use crate::{file_traverser::TemplateFile, enriched_template_file_processor::EnrichedTemplateFile, shared_models::ZatResultX};

pub trait TemplateEnricher {
  fn enrich(&self, template_file: TemplateFile) -> ZatResultX<EnrichedTemplateFile>;
}
