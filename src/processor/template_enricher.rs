use crate::error::ZatResult;
use super::{TemplateFile, EnrichedTemplateFile};

pub trait TemplateEnricher {
  fn enrich(&self, template_file: TemplateFile) -> ZatResult<EnrichedTemplateFile>;
}
