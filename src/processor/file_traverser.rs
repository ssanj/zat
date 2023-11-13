use crate::config::TemplateFilesDir;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum TemplateFile {
  File(String),
  Dir(String),
}

pub trait FileTraverser {
  /// Template directory to traverse
  fn traverse_files(&self, template_dir: &TemplateFilesDir) -> Vec<TemplateFile>;
}


impl TemplateFile {

  #[cfg(test)]
  pub fn new_file(file: &str) -> Self {
    Self::File(file.to_owned())
  }

  #[cfg(test)]
  pub fn new_dir(dir: &str) -> Self {
    Self::Dir(dir.to_owned())
  }
}

