use crate::{file_traverser::TemplateFile};

/// Example filters:
/// - .variable
/// - */input/*
/// - *.json
pub trait FileChooser  {
  fn is_included(&self, file_type: TemplateFile) -> bool;

  fn is_ignored(&self, file_type: TemplateFile) -> bool {
    !self.is_included(file_type)
  }
}
