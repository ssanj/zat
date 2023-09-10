use std::todo;

use crate::{user_config_provider::UserConfigX, template_file_processor::TemplateFileProcessor, shared_models::ZatResultX, file_traverser::TemplateFile};

pub struct DefaultTemplateFileProcessor {
  config: UserConfigX
}

impl TemplateFileProcessor for DefaultTemplateFileProcessor {

  fn process_template_files<T>(&self, template_files: &[TemplateFile], replacer: T) -> ZatResultX<()>
    where T: Fn(&str) -> String {
    todo!()
  }
}
