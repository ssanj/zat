use crate::file_chooser::FileChooser;
use crate::file_traverser::TemplateFile;
use crate::user_config_provider::FileFilter;
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
                TemplateFile::Symlink(_) => false, // Don't match on symlinks
              }
            });

        !excluded
    }
}

//   // directories should end in the directory or have a <directory>/ match
//   let re = Regex::new(r"(\.git$|\.git/)").unwrap();

//   // files should end in the file name
//   let re2 = Regex::new(r"\.variables.prompt$").unwrap();
//   let git_file = "/Users/sanj/ziptemp/st-template/.git/hooks/commit-msg.sample";
//   let git_ignore = "/Users/sanj/ziptemp/st-template/.gitignore";
//   let git_dir = "/Users/sanj/ziptemp/st-template/.git";
//   let variables = "/Users/sanj/ziptemp/st-template/.variables.prompt";

//   println!("git file:{}", re.is_match(git_file));
//   println!("git ignore:{}", re.is_match(git_ignore));
//   println!("git_dir {}", re.is_match(git_dir));
//   println!("variables {}", re.is_match(variables));
//   println!("=======");
//   println!("git file:{}", re2.is_match(git_file));
//   println!("git ignore:{}", re2.is_match(git_ignore));
//   println!("git_dir {}", re2.is_match(git_dir));
//   println!("variables {}", re2.is_match(variables));
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_all_without_filters() {
      let filters = vec![];
      let file_chooser = RegExFileChooser::new(&filters).unwrap();
      assert!(file_chooser.is_included(TemplateFile::new_file(".variables.prompt")));
      assert!(file_chooser.is_included(TemplateFile::new_dir("/some/path/included")));
      assert!(file_chooser.is_included(TemplateFile::new_symlink("/some/path/linked")));
    }

    #[test]
    fn ignores_none_without_filters() {
      let filters = vec![];
      let file_chooser = RegExFileChooser::new(&filters).unwrap();
      assert!(!file_chooser.is_ignored(TemplateFile::new_file(".variables.prompt")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_dir("/some/path/included")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_symlink("/some/path/linked")));
    }

    #[test]
    fn ignores_file_matching_filter() {
      let filters =
        vec!
          [
            ".variables.prompt",
            r"/some/path/excluded/.+\.txt",
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
      assert!(!file_chooser.is_ignored(TemplateFile::new_symlink("/some/path/linked")));
      assert!(!file_chooser.is_ignored(TemplateFile::new_file("/some/path/excluded/nested/some_file.pdf")));
    }
}
