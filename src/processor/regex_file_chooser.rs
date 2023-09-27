use super::FileChooser;
use super::TemplateFile;
use regex::Regex;

pub struct RegExFileChooser {
  filters: Vec<Regex>
}


impl RegExFileChooser {
  pub fn new(values: &[&str]) -> Result<RegExFileChooser, regex::Error> {
    let maybe_filters: Result<Vec<Regex>, regex::Error> =
      values
        .iter()
        .map(|v|{
          Regex::new(v)
        }) //Iter<Result<RegEx, regex::Error>>
        .collect();

     maybe_filters.map(|filters|{
      RegExFileChooser {
        filters
      }
     })
  }
}

impl Default for RegExFileChooser {
    fn default() -> Self {
        Self {
          filters: vec![]
        }
    }
}

impl FileChooser for RegExFileChooser {
    fn is_included(&self, file_type: TemplateFile) -> bool {
        let excluded =
          self
            .filters
            .iter()
            .any(|f|{
              match &file_type {
                TemplateFile::File(file) => f.is_match(&file),
                TemplateFile::Dir(dir) => f.is_match(&dir),
              }
            });

        !excluded
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_all_without_filters() {
      let filters = vec![];
      let file_chooser = RegExFileChooser::new(&filters).unwrap();
      assert!(file_chooser.is_included(TemplateFile::new_file(".variables.prompt")));
      assert!(file_chooser.is_included(TemplateFile::new_dir("/some/path/included")));
    }

    #[test]
    fn ignores_nothing_without_filters() {
      let filters = vec![];
      let file_chooser = RegExFileChooser::new(&filters).unwrap();
      assert!(!file_chooser.is_ignored(TemplateFile::new_file(".variables.prompt")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_dir("/some/path/included")));
    }

    #[test]
    fn ignores_file_matching_filter() {
      let filters =
        vec!
          [
            ".variables.prompt",
            r"/some/path/excluded/.+\.txt", // matches nested folders as well
          ];

      let file_chooser = RegExFileChooser::new(&filters).unwrap();

      // Should be ignored
      assert!(file_chooser.is_ignored(TemplateFile::new_file(".variables.prompt")));
      assert!(file_chooser.is_ignored(TemplateFile::new_file("/some/path/excluded/some_file.txt")));
      assert!(file_chooser.is_ignored(TemplateFile::new_file("/some/path/excluded/nested/some_other_file.txt")));
      assert!(file_chooser.is_ignored(TemplateFile::new_file("/some/path/excluded/nested1/nested2/nested.txt")));

      // Should not be ignored
      assert!(!file_chooser.is_ignored(TemplateFile::new_file(".variable2.prompt")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_file(".variables")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_file(".xariables.prompt")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_dir("/some/path/included")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_file("/some/path/excluded/nested/some_file.pdf")));
    }
}
