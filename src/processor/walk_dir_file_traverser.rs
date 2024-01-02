use std::path::Path;

use super::{FileChooser, FileTraverser, TemplateFile};
use crate::config::TemplateFilesDir;
use walkdir::{WalkDir, DirEntry};

pub struct WalkDirFileTraverser {
  file_chooser: Box<dyn FileChooser>
}

impl FileTraverser for WalkDirFileTraverser {
    fn traverse_files(&self, template_files_dir: &TemplateFilesDir) -> Vec<TemplateFile> {
      WalkDir::new(template_files_dir)
          .into_iter()
          .filter_map(|re| re.ok())
          .map(|dir_entry|{
            let p = dir_entry.path();
            self.categories_files(&dir_entry, p)
          })
          .filter_map(|tf|{
            tf.filter(|template| self.file_chooser.is_included(template.clone()))
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

  fn categories_files(&self, dir_entry: &DirEntry, path: &Path) -> Option<TemplateFile> {
    let string_path = path.to_string_lossy().to_string();
    let entry_file_type = dir_entry.file_type();
      if entry_file_type.is_file() {
        Some(TemplateFile::File(string_path))
      } else if entry_file_type.is_dir() {
        Some(TemplateFile::Dir(string_path))
      } else {
        None
      }
  }
}

// TODO: This test is hard to understand. Refactor.
#[cfg(test)]
mod tests {
    use super::super::regex_file_chooser::RegExFileChooser;
    use tempfile::{tempdir, tempdir_in};
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{self, Write};
    use crate::config::RepositoryDir;

    use super::*;
    use pretty_assertions::assert_eq;

    /// Models files in the multiple folders, template, input, output, working dirs
    /// The code that interprets this enum will create the appropriate folders and the specified directories
    enum InputFileType {
      TemplateDirFile(String, String),
      InputDirFile(String, String),
      OutputDirFile(String, String),
      WorkingDirFile(String, String)
    }


    struct TemplateDirectory(String);
    struct InputDirectory(String);
    struct OutputDirectory(String);
    struct WorkingDirectory(String);


    #[test]
    fn should_include_all_without_ignores() -> io::Result<()> {
      let source_files =
        [
          InputFileType::TemplateDirFile("blee.txt".to_owned(), "I said blee".to_owned()),
          InputFileType::TemplateDirFile("blah.txt".to_owned(), "I said blah".to_owned()),
          InputFileType::TemplateDirFile(".variables".to_owned(), "{}".to_owned()),
          InputFileType::InputDirFile("input1.json".to_owned(), "{}".to_owned()),
          InputFileType::InputDirFile("input2.json".to_owned(), "{}".to_owned()),
          InputFileType::OutputDirFile("output1.json".to_owned(), "{}".to_owned()),
          InputFileType::OutputDirFile("output2.json".to_owned(), "{}".to_owned()),
          InputFileType::WorkingDirFile("output1.wip".to_owned(), "{}".to_owned()),
          InputFileType::WorkingDirFile("output2.wip".to_owned(), "{}".to_owned()),
        ];


      let ignores =
        |_: TemplateDirectory, _: InputDirectory, _: OutputDirectory, _: WorkingDirectory| { vec![] };

      let expected_files =
        |template_dir_path: TemplateDirectory, input_dir_path: InputDirectory, output_dir_path: OutputDirectory, working_dir_path: WorkingDirectory| {

          let template_dir_path_string = template_dir_path.0;
          let input_dir_path_string = input_dir_path.0;
          let output_dir_path_string = output_dir_path.0;
          let working_dir_path_string = working_dir_path.0;

          vec![
            TemplateFile::Dir(format!("{}", template_dir_path_string)),
            TemplateFile::Dir(format!("{}", input_dir_path_string)),
            TemplateFile::Dir(format!("{}", output_dir_path_string)),
            TemplateFile::Dir(format!("{}", working_dir_path_string)),

            TemplateFile::File(format!("{}/blee.txt", template_dir_path_string)),
            TemplateFile::File(format!("{}/blah.txt", template_dir_path_string)),
            TemplateFile::File(format!("{}/.variables", template_dir_path_string)),

            TemplateFile::File(format!("{}/input1.json", input_dir_path_string)),
            TemplateFile::File(format!("{}/input2.json", input_dir_path_string)),

            TemplateFile::File(format!("{}/output1.json", output_dir_path_string)),
            TemplateFile::File(format!("{}/output2.json", output_dir_path_string)),

            TemplateFile::File(format!("{}/output1.wip", working_dir_path_string)),
            TemplateFile::File(format!("{}/output2.wip", working_dir_path_string)),
          ]
        };

       assert_ignores(&source_files, ignores, expected_files)
    }

    #[test]
    fn should_respect_ignores() -> io::Result<()> {
      let source_files =
        [
          InputFileType::TemplateDirFile("blee.txt".to_owned(), "I said blee".to_owned()),
          InputFileType::TemplateDirFile("blah.txt".to_owned(), "I said blah".to_owned()),
          InputFileType::TemplateDirFile(".variables".to_owned(), "{}".to_owned()),
          InputFileType::InputDirFile("input1.json".to_owned(), "{}".to_owned()),
          InputFileType::InputDirFile("input2.json".to_owned(), "{}".to_owned()),
          InputFileType::OutputDirFile("output1.json".to_owned(), "{}".to_owned()),
          InputFileType::OutputDirFile("output2.json".to_owned(), "{}".to_owned()),
          InputFileType::WorkingDirFile("output1.wip".to_owned(), "{}".to_owned()),
          InputFileType::WorkingDirFile("output2.wip".to_owned(), "{}".to_owned()),
        ];

      let ignores =
        |_template_dir_path: TemplateDirectory, _input_dir_path: InputDirectory, _output_dir_path: OutputDirectory, working_dir_path: WorkingDirectory| {
          vec![
            r"\.variables".to_owned(),
            r"input1\.json".to_owned(),
            r"output1\.json".to_owned(),
            working_dir_path.0.to_owned()
          ]
      };

      let expected_files =
        |template_dir_path: TemplateDirectory, input_dir_path: InputDirectory, output_dir_path: OutputDirectory, _: WorkingDirectory| {

          let template_dir_path_string = template_dir_path.0;
          let input_dir_path_string = input_dir_path.0;
          let output_dir_path_string = output_dir_path.0;

        vec![
          TemplateFile::Dir(format!("{}", template_dir_path_string)),
          TemplateFile::Dir(format!("{}", input_dir_path_string)),
          TemplateFile::File(format!("{}/input2.json", input_dir_path_string)),
          TemplateFile::Dir(format!("{}", output_dir_path_string)),
          TemplateFile::File(format!("{}/output2.json", output_dir_path_string)),
          TemplateFile::File(format!("{}/blee.txt", template_dir_path_string)),
          TemplateFile::File(format!("{}/blah.txt", template_dir_path_string)),
        ]
      };

      assert_ignores(&source_files, ignores, expected_files)
    }


    fn assert_ignores<G, F>(source_files: &[InputFileType], ignores: G, expected_files: F) -> io::Result<()> where
      G: FnOnce(TemplateDirectory, InputDirectory, OutputDirectory, WorkingDirectory) -> Vec<String>,
      F: FnOnce(TemplateDirectory, InputDirectory, OutputDirectory, WorkingDirectory) -> Vec<TemplateFile>
    {
      let temp_dir = tempdir()?; // create a temporary working directory
      let template_files_dir = TemplateFilesDir::from(&RepositoryDir::from(temp_dir.path())); // template files directory

      std::fs::create_dir(template_files_dir.as_ref()).expect("Could not create temporary template directory"); // create template files directory

      let template_files_dir_path = template_files_dir.as_ref();

      // template_files_dir_path/random input dir
      let input_dir = tempdir_in(template_files_dir_path)?;

      // template_files_dir_path/random output dir
      let output_dir = tempdir_in(template_files_dir_path)?;

      // template_files_dir_path/random working dir
      let working_dir = tempdir_in(template_files_dir_path)?;

      let input_dir_path = input_dir.path();
      let output_dir_path = output_dir.path();
      let working_dir_path = working_dir.path();

      // Create all source files requested under the specified sub directories
      for f in source_files {
        match f {
            InputFileType::TemplateDirFile(filename, content) =>  write_file(template_files_dir_path, filename, content)?,
            InputFileType::InputDirFile(filename, content)    =>  write_file(input_dir_path, filename, content)?,
            InputFileType::OutputDirFile(filename, content)   =>  write_file(output_dir_path, filename, content)?,
            InputFileType::WorkingDirFile(filename, content)  =>  write_file(working_dir_path, filename, content)?,
        }
      }

      let templat_dir_path_string = template_files_dir_path.to_string_lossy().to_string();
      let input_dir_path_string = input_dir_path.to_string_lossy().to_string();
      let output_dir_path_string = output_dir_path.to_string_lossy().to_string();
      let working_dir_path_string = working_dir_path.to_string_lossy().to_string();

      let ignored =
        ignores(
          TemplateDirectory(templat_dir_path_string.to_owned()),
          InputDirectory(input_dir_path_string.to_owned()),
          OutputDirectory(output_dir_path_string.to_owned()),
          WorkingDirectory(working_dir_path_string.to_owned())
        );

      let ignored_refs: Vec<&str> =
        ignored
          .iter()
          .map(|v| v.as_str())
          .collect();

      let regex_patterns = RegExFileChooser::new(&ignored_refs).expect("Could not create regex patterns");

      let file_chooser = Box::new(regex_patterns);
      let traverser = WalkDirFileTraverser::new(file_chooser);

      let matches = traverser.traverse_files(&template_files_dir);

      println!("templat_dir_path_string: {}", &templat_dir_path_string);
      println!("input_dir_path: {}", &input_dir_path_string);
      println!("output_dir_path: {}", &output_dir_path_string);
      println!("working_dir: {}", &working_dir_path_string);

      let expected_matches =
        expected_files(
          TemplateDirectory(templat_dir_path_string.to_owned()),
          InputDirectory(input_dir_path_string.to_owned()),
          OutputDirectory(output_dir_path_string.to_owned()),
          WorkingDirectory(working_dir_path_string.to_owned())
        );

      temp_dir.close()?;

      let matches_set: HashSet<TemplateFile> = HashSet::from_iter(matches);
      let expected_matches_set: HashSet<TemplateFile> = HashSet::from_iter(expected_matches);

      println!("matches_set: {:?}", &matches_set);
      println!("expected_matches_set: {:?}", &expected_matches_set);

      assert_eq!(matches_set, expected_matches_set, "Expected the same number of items");

      Ok(())
    }

    fn write_file(dir: &Path, file_name: &str, content: &str) -> io::Result<()> {
      println!("writing path: {}, filename: {}", dir.to_string_lossy().to_string(), file_name);
      let file_path = dir.join(file_name);
      let mut file = File::create(file_path)?;
      writeln!(file, "{}", content)?;

      Ok(())
    }
}
