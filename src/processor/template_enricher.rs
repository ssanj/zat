use crate::error::ZatResultX;
use super::{TemplateFile, EnrichedTemplateFile};

pub trait TemplateEnricher {
  fn enrich(&self, template_file: TemplateFile) -> ZatResultX<EnrichedTemplateFile>;
}
