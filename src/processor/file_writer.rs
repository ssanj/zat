use crate::error::ZatResult;
use super::SourceFile;
use super::DestinationFile;
use super::StringTokenReplacer;

/// Responsible for writing out a file name and contents with any tokens replaced. The tokens within the content of the file
/// is only replaced if the file is a template file (extension of '.tmpl'). The template file is written out without the '.tmpl'
/// extension.
/// Examples:
///   Given: $project$.py -> your_cool_project.py (assuming the value of the variable project is 'your_cool_project')
///   Given: $project$.py.tmpl -> your_cool_project.py (same as above but all tokens within the $project$.py.tmpl file will be replaced before it is written out)
///   Given: README.md.tmpl -> README.md (any tokens in README.md.tmpl will be replaced, before it is written out)
///   Given: README.md -> README.md (any tokens in README.md will NOT be replaced)
pub trait FileWriter  {
  fn write_source_to_destination(&self, source_file: &SourceFile, destination_file: &DestinationFile, token_replacer: &dyn StringTokenReplacer) -> ZatResult<()>;
}
