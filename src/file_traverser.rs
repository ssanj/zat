use crate::user_config_provider::UserConfig;

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateFile {
  File(String),
  Dir(String),
  Symlink(String)
}

trait FileTraverser {
  /// user_config: Use the `template_dir` and `ignores` to return the files and directory to be considered
  fn traverse_files(&self, user_config: UserConfig) -> Vec<TemplateFile>;
}


#[cfg(test)]
mod tests {
    use crate::{user_config_provider::Ignores, models::{TemplateDir, TargetDir}};

    use super::*;

    #[derive(Clone, Debug)]
    struct VecFileTraverser {
      files: Vec<TemplateFile>
    }

    impl VecFileTraverser {
      fn new(files: Vec<TemplateFile>) -> Self {
        Self {
          files
        }
      }
    }


    impl FileTraverser for VecFileTraverser {
      fn traverse_files(&self, user_config: UserConfig) -> Vec<TemplateFile> {
        self
        .files
        .clone()
        .into_iter()
        .filter(|tp|{
          match tp {
            TemplateFile::File(file) => user_config.ignores.files.iter().all(|v| v != file),
            TemplateFile::Dir(dir) => user_config.ignores.directories.iter().all(|v| v != dir),
            TemplateFile::Symlink(_) => true
          }
        })
        .collect()
      }
    }

    #[test]
    fn should_handle_empty_file_lists_and_ignores() {
      let template_dir = TemplateDir::new("blah");
      let target_dir = TargetDir::new("blee");
      let ignores = Ignores::default();

      let user_config =
        UserConfig {
          template_dir,
          target_dir,
          ignores
        };

      let file_traverser = VecFileTraverser::new(vec![]); // No Files
      let files_to_process = file_traverser.traverse_files(user_config);

      assert!(files_to_process.is_empty(), "files_to_process should be empty")
    }

    #[test]
    fn should_handle_empty_ignores() {
      let template_dir = TemplateDir::new("blah");
      let target_dir = TargetDir::new("blee");
      let ignores = Ignores::default();

      let user_config =
        UserConfig {
          template_dir,
          target_dir,
          ignores
        };

      let valid_files = vec![TemplateFile::File("abc.txt".to_owned()), TemplateFile::File("two.json".to_owned())];
      let file_traverser = VecFileTraverser::new(valid_files.clone());
      let files_to_process = file_traverser.traverse_files(user_config);

      assert_eq!(files_to_process.len(), valid_files.len(), "files_to_process should be the same as valid_files")
    }

    #[test]
    fn should_respect_ignores() {
      let template_dir = TemplateDir::new("blah");
      let target_dir = TargetDir::new("blee");
      let files = vec!["two.json".to_owned(), "three.json".to_owned()];
      let directories = vec!["data/outputs".to_owned()];

      let ignores =
        Ignores {
          files,
          directories
        };

      let user_config =
        UserConfig {
          template_dir,
          target_dir,
          ignores
        };

      let input_files =
        vec![
          TemplateFile::File("plugin.py".to_owned()),
          TemplateFile::Dir("data/inputs".to_owned()),
          TemplateFile::Dir("data/outputs".to_owned()),
          TemplateFile::File("two.json".to_owned()),
          TemplateFile::File("main.py".to_owned()),
          TemplateFile::File("three.json".to_owned()),
          TemplateFile::Symlink("schema".to_owned())
        ];

      let valid_files =
        vec![
          TemplateFile::File("plugin.py".to_owned()),
          TemplateFile::Dir("data/inputs".to_owned()),
          TemplateFile::File("main.py".to_owned()),
          TemplateFile::Symlink("schema".to_owned())
        ];

      let file_traverser = VecFileTraverser::new(input_files.clone());
      let files_to_process = file_traverser.traverse_files(user_config);

      assert_eq!(files_to_process, valid_files, "files_to_process should be the same as valid_files")
    }
}
