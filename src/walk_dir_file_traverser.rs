use std::path::Path;

use crate::file_chooser::FileChooser;
use crate::file_traverser::{FileTraverser, TemplateFile};
use crate::models::TemplateDir;
use crate::user_config_provider::{UserConfigX, Ignores};
use walkdir::{WalkDir, DirEntry};

pub struct WalkDirFileTraverser {
  file_chooser: Box<dyn FileChooser>
}

impl FileTraverser for WalkDirFileTraverser {
    fn traverse_files(&self, template_dir: &TemplateDir) -> Vec<TemplateFile> {
      WalkDir::new(template_dir)
          .into_iter()
          .filter_map(|re| re.ok())
          .map(|dir_entry|{
            let p = dir_entry.path();
            self.get_template_file(&dir_entry, p)
          })
          .filter(|tf|{
            self.file_chooser.is_included(tf.clone())
            // self.remove_ignores(tf, &user_config.ignores)
          })
          .collect()
    }
}

impl WalkDirFileTraverser {
  pub fn new(file_chooser: Box<dyn FileChooser>) -> Self {
    Self {
      file_chooser
    }
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

  // fn remove_ignores(&self, template_file: &TemplateFile,ignores: &Ignores) -> bool {
  //   let is_ignored =
  //     match template_file {
  //       TemplateFile::File(file_path) => {
  //         ignores
  //           .files
  //           .iter()
  //           .any(|files_to_ignore| {
  //             file_path.ends_with(files_to_ignore) ||
  //             ignores
  //               .directories
  //               .iter()
  //               .any(|directories_to_ignore| file_path.contains(directories_to_ignore)) //TODO: we should break this into path segments
  //           })
  //       },
  //       TemplateFile::Dir(directory_path) => ignores.directories.iter().any(|directories_to_ignore| directory_path.contains(directories_to_ignore)),
  //       TemplateFile::Symlink(_) => false,
  //     };

  //   !is_ignored
  // }
}

#[cfg(test)]
mod tests {
    use crate::regex_file_chooser::RegExFileChooser;
    use crate::{models::TargetDir, user_config_provider::Ignores};
    use tempfile::{tempdir, tempdir_in};
    use std::fs::File;
    use std::io::{self, Write};

    use super::*;
    use pretty_assertions::assert_eq;


    // #[test]
    // fn should_handle_empty_ignores() -> io::Result<()> {
    //   let temp_dir = tempdir()?;
    //   let template_dir = TemplateDir::new(&temp_dir.path().to_string_lossy().to_string());
    //   let p = temp_dir.path();
    //   write_file(p, "blee.txt", "I said blee")?;
    //   write_file(p, "blah.txt", "I said blah")?;

    //   let traverser = WalkDirFileTraverser::new();
    //   let target_dir = TargetDir::new("");
    //   let ignores = Ignores::default();

    //   let user_config =
    //     UserConfig {
    //       template_dir,
    //       target_dir,
    //       ignores
    //     };

    //   let matches = traverser.traverse_files(user_config);
    //   let temp_path = p.to_string_lossy().to_string();
    //   let expected_matches =
    //     [
    //       TemplateFile::Dir(format!("{}", temp_path)),
    //       TemplateFile::File(format!("{}/blee.txt", temp_path)),
    //       TemplateFile::File(format!("{}/blah.txt", temp_path)),
    //     ];

    //   temp_dir.close()?;
    //   assert_eq!(matches, expected_matches);

    //   Ok(())
    // }

    #[test]
    fn should_respect_ignores() -> io::Result<()> {
      let temp_dir = tempdir()?;
      let template_dir = TemplateDir::new(&temp_dir.path().to_string_lossy().to_string());
      let template_dir_path = temp_dir.path();

      let input_dir = tempdir_in(template_dir_path)?;
      let output_dir = tempdir_in(template_dir_path)?;
      let working_dir = tempdir_in(template_dir_path)?;

      let input_dir_path = input_dir.path();
      let output_dir_path = output_dir.path();

      let working_dir_path = working_dir.path();

      write_file(template_dir_path, "blee.txt", "I said blee")?;
      write_file(template_dir_path, "blah.txt", "I said blah")?;
      write_file(template_dir_path, ".variables", "{}")?;
      write_file(input_dir_path, "input1.json", "{}")?;
      write_file(input_dir_path, "input2.json", "{}")?;
      write_file(output_dir_path, "output1.json", "{}")?;
      write_file(output_dir_path, "output2.json", "{}")?;
      write_file(working_dir_path, "output1.wip", "{}")?;
      write_file(working_dir_path, "output2.wip", "{}")?;

      let ignores =
        [
          r"\.variables",
          r"input1\.json",
          r"output1\.json",
          &working_dir_path.to_string_lossy().to_string()
        ];

      let regex_patterns = RegExFileChooser::new(&ignores).expect("Could not create regex patterns");
      let file_chooser = Box::new(regex_patterns);
      let traverser = WalkDirFileTraverser::new(file_chooser);

      let matches = traverser.traverse_files(&template_dir);
      let templat_dir_path_string = template_dir_path.to_string_lossy().to_string();
      let input_dir_path_string = input_dir_path.to_string_lossy().to_string();
      let output_dir_path_string = output_dir_path.to_string_lossy().to_string();
      let working_dir_path_string = working_dir_path.to_string_lossy().to_string();

      println!("input_dir_path: {}", input_dir_path_string);
      println!("output_dir_path: {}", output_dir_path_string);
      println!("working_dir: {}", working_dir_path_string);

      let expected_matches =
        [
          TemplateFile::Dir(format!("{}", templat_dir_path_string)),
          TemplateFile::Dir(format!("{}", input_dir_path_string)),
          TemplateFile::File(format!("{}/input2.json", input_dir_path_string)),
          TemplateFile::Dir(format!("{}", output_dir_path_string)),
          TemplateFile::File(format!("{}/output2.json", output_dir_path_string)),
          TemplateFile::File(format!("{}/blee.txt", templat_dir_path_string)),
          TemplateFile::File(format!("{}/blah.txt", templat_dir_path_string)),
        ];

      temp_dir.close()?;
      assert_eq!(matches.len(), expected_matches.len(), "Expected the same number of items returned and expected");

      let variable_file = TemplateFile::File(format!("{}/.variables", templat_dir_path_string));
      assert!(!expected_matches.contains(&variable_file), "should not contain {:?}", variable_file);

      let input1_file = TemplateFile::File(format!("{}/input1.json", input_dir_path_string));
      assert!(!expected_matches.contains(&input1_file), "should not contain {:?}", input1_file);

      let output1_file = TemplateFile::File(format!("{}/output1.json", output_dir_path_string));
      assert!(!expected_matches.contains(&output1_file), "should not contain {:?}", output1_file);

      let working_dir_output1_file = TemplateFile::Dir(format!("{}/output1.wip", working_dir_path_string));
      let working_dir_output2_file = TemplateFile::Dir(format!("{}/output1.wip", working_dir_path_string));
      assert!(!expected_matches.contains(&working_dir_output1_file), "should not contain {:?}", working_dir_output1_file);
      assert!(!expected_matches.contains(&working_dir_output2_file), "should not contain {:?}", working_dir_output2_file);

      Ok(())
    }

    fn write_file(dir: &Path, file_name: &str, content: &str) -> io::Result<()> {
      let file_path = dir.join(file_name);
      let mut file = File::create(file_path)?;
      writeln!(file, "{}", content)?;

      Ok(())
    }
}
