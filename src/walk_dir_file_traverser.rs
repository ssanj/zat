use std::path::Path;

use crate::file_traverser::{FileTraverser, TemplateFile};
use crate::models::TemplateDir;
use crate::user_config_provider::UserConfig;
use walkdir::{WalkDir, DirEntry};

pub struct WalkDirFileTraverser;

impl FileTraverser for WalkDirFileTraverser {
    fn traverse_files(&self, user_config: UserConfig) -> Vec<TemplateFile> {
      WalkDir::new(&user_config.template_dir)
          .into_iter()
          .filter_map(|re| re.ok())
          .map(|dir_entry|{
            let p = dir_entry.path();
            self.get_template_file(&dir_entry, p)
          })
          .collect()
    }
}

impl WalkDirFileTraverser {
  pub fn new() -> Self {
    Self
  }

  fn get_template_file(&self, dir_entry: &DirEntry, path: &Path) -> TemplateFile {
    let string_path = path.to_string_lossy().to_string();
    let entry_file_type = dir_entry.file_type();
      if entry_file_type.is_file() {
        TemplateFile::File(string_path)
      } else if entry_file_type.is_dir() {
        TemplateFile::Dir(string_path)
      } else {
        TemplateFile::Symlink(string_path)
      }
  }
}

#[cfg(test)]
mod tests {
    use crate::{models::TargetDir, user_config_provider::Ignores};
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::{self, Write};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_handle_empty_file_lists_and_ignores() -> io::Result<()> {
      let temp_dir = tempdir()?;
      let template_dir = TemplateDir::new(&temp_dir.path().to_string_lossy().to_string());
      let p = temp_dir.path();
      write_file(p, "blee.txt", "I said blee")?;
      write_file(p, "blah.txt", "I said blah")?;

      let traverser = WalkDirFileTraverser::new();
      let target_dir = TargetDir::new("");
      let ignores = Ignores::default();

      let user_config =
        UserConfig {
          template_dir,
          target_dir,
          ignores
        };

      let matches = traverser.traverse_files(user_config);
      let temp_path = p.to_string_lossy().to_string();
      let expected_matches =
        [
          TemplateFile::Dir(format!("{}", temp_path)),
          TemplateFile::File(format!("{}/blee.txt", temp_path)),
          TemplateFile::File(format!("{}/blah.txt", temp_path)),
        ];

      temp_dir.close()?;
      assert_eq!(matches, expected_matches);

      Ok(())
    }

    fn write_file(dir: &Path, file_name: &str, content: &str) -> io::Result<()> {
      let file_path = dir.join(file_name);
      let mut file = File::create(file_path)?;
      writeln!(file, "{}", content)?;

      Ok(())
    }
}
