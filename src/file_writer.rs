use crate::{file_traverser::TemplateFile};
use crate::ZatErrorX;

/// Responsible for writing out a file name and contents with any tokens replaced. The tokens within the content of the file
/// is only replaced if the file is a template file (extension of '.tmpl'). The template file written out without the '.tmpl'
/// extension.
/// Examples:
///   Given: $project$.py -> your_cool_project.py (assuming the value of the variable project is 'your_cool_project')
///   Given: $project$.py.tmpl -> your_cool_project.py (same as above but all tokens within the $project$.py.tmpl file will be replaced before it is written out)
///   Given: README.md.tmpl -> README.md (any tokens in README.md.tmpl will be replaced, before it is written out)
///   Given: README.md -> README.md (any tokens in README.md will NOT be replaced)
pub trait FileWriter  {
  fn write_file<P, C, T>(&self, file: P, content: C, token_replacer: T) -> ZatErrorX
    where P: AsRef<Path> + Display,
          C: AsRef<[u8]>,
          T: Fn(&str) -> String;
}
