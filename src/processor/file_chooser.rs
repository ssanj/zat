use super::TemplateFile;

/// Chooses files to include or exclude
/// property: is_included should be the inverse of is_ignored (meaning the same input can't be ignored and included at the same time)
pub trait FileChooser  {
  fn is_included(&self, file_type: TemplateFile) -> bool;

  fn is_ignored(&self, file_type: TemplateFile) -> bool {
    !self.is_included(file_type)
  }
}
