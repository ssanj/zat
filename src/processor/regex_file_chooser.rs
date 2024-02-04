use crate::config::TemplateFilesDir;

use super::FileChooser;
use super::TemplateFile;
use regex::Regex;

pub struct RegExFileChooser<'a> {
  template_files_dir: &'a TemplateFilesDir,
  filters: Vec<Regex>
}


impl <'a> RegExFileChooser<'a> {
  pub fn new(template_files_dir: &'a TemplateFilesDir, values: &[&str]) -> Result<RegExFileChooser<'a>, regex::Error> {
    let maybe_filters: Result<Vec<Regex>, regex::Error> =
      values
        .iter()
        .map(|v|{
          Regex::new(v)
        }) //Iter<Result<RegEx, regex::Error>>
        .collect();

     maybe_filters.map(|filters|{
      RegExFileChooser {
        template_files_dir,
        filters
      }
     })
  }
}


impl <'a> FileChooser for RegExFileChooser<'a> {
    fn is_included(&self, file_type: TemplateFile) -> bool {
        let excluded =
          self
            .filters
            .iter()
            .any(|f|{
              match &file_type {
                TemplateFile::File(file) => f.is_match(&self.template_files_dir.relative_path(&file)),
                TemplateFile::Dir(dir) => f.is_match(&self.template_files_dir.relative_path(&dir)),
              }
            });

        !excluded
    }
}


#[cfg(test)]
mod tests {
    use crate::config::RepositoryDir;

    use super::*;

    #[test]
    fn includes_all_without_filters() {
      let filters = vec![];
      let repository_directory = RepositoryDir::new("/some/path");
      let template_files_dir: TemplateFilesDir = (&repository_directory).into();
      let file_chooser = RegExFileChooser::new(&template_files_dir, &filters).unwrap();
      assert!(file_chooser.is_included(TemplateFile::new_file("/some/path/.variables.zat-prompt")));
      assert!(file_chooser.is_included(TemplateFile::new_dir("/some/path/template/included")));
    }

    #[test]
    fn ignores_nothing_without_filters() {
      let filters = vec![];
      let repository_directory = RepositoryDir::new("/some/path");
      let template_files_dir: TemplateFilesDir = (&repository_directory).into();
      let file_chooser = RegExFileChooser::new(&template_files_dir, &filters).unwrap();
      assert!(!file_chooser.is_ignored(TemplateFile::new_file("/some/path/.variables.zat-prompt")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_dir("/some/path/template/included")));
    }

    #[test]
    fn ignores_file_matching_filter() {
      let filters =
        vec!
          [
            "^project.conf",
            r"^nested/path/excluded/.+\.txt", // matches nested folders as well
          ];

      let repository_directory = RepositoryDir::new("/some/path");
      let template_files_dir: TemplateFilesDir = (&repository_directory).into();
      let file_chooser = RegExFileChooser::new(&template_files_dir, &filters).unwrap();

      // Should be ignored
      assert!(file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/project.conf")));
      assert!(file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/nested/path/excluded/some_file.txt")));
      assert!(file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/nested/path/excluded/some_other_file.txt")));
      assert!(file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/nested/path/excluded/nested1/nested2/nested.txt")));

      // Should not be ignored
      assert!(!file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/.project2.conf")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/.variables")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/.xproject.conf")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_dir("/some/path/template/included")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/excluded/nested/some_file.pdf")));
    }

    #[test]
    fn handles_default_ignores() {
      let filters =
        crate::config::IgnoredFiles::DEFAULT_IGNORES;

      let repository_directory = RepositoryDir::new("/some/path");
      let template_files_dir: TemplateFilesDir = (&repository_directory).into();
      let file_chooser = RegExFileChooser::new(&template_files_dir, &filters).unwrap();

      // .git files should be ignored
      assert!(file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/.git/description")));
      assert!(file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/.git/hooks/README.sample")));
      assert!(file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/.git/index")));

      // .gitignore should not be ignored
      assert!(!file_chooser.is_ignored(TemplateFile::new_file("/some/path/template/.gitignore")));
    }
}
