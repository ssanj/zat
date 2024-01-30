use std::path::Path;

use super::{FileChooser, FileTraverser, TemplateFile};
use crate::config::TemplateFilesDir;
use walkdir::{WalkDir, DirEntry};

pub struct WalkDirFileTraverser<'a> {
  file_chooser: Box<dyn FileChooser + 'a>
}

impl FileTraverser for WalkDirFileTraverser<'_> {
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

impl <'a> WalkDirFileTraverser<'a> {
  pub fn new(file_chooser: Box<dyn FileChooser+ 'a>) -> Self {
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
    use std::format as s;

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

      // Describe which files and folders should be created under the 'template' directory.
      // TemplateDirFile -> 'template' folder
      // InputDirFilel   -> 'template/<random input>' folder
      // OutputDirFile   -> 'template/<random output>' folder
      // WorkingDirFile  -> 'template/<random working>' folder
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

      // These files should be ignored and not present in the target folder.
      let ignores =
        |template_dir_path: TemplateDirectory, _input_dir_path: InputDirectory, _output_dir_path: OutputDirectory, working_dir_path: WorkingDirectory| {
          let relative_working_dir = Path::new(&working_dir_path.0).strip_prefix(&template_dir_path.0).expect("Could not remove prefix").to_string_lossy();
          vec![
            r"^\.variables".to_owned(),
            r"input1\.json".to_owned(),
            r"output1\.json".to_owned(),
            s!("^{}", relative_working_dir)
          ]
      };

      // These are the files we expect at the target folder.
      let expected_files =
        |template_dir_path: TemplateDirectory, input_dir_path: InputDirectory, output_dir_path: OutputDirectory, _: WorkingDirectory| {

          let template_dir_path_string = template_dir_path.0;
          let input_dir_path_string = input_dir_path.0;
          let output_dir_path_string = output_dir_path.0;

        vec![
            TemplateFile::Dir(format!("{}", template_dir_path_string)),
            TemplateFile::Dir(format!("{}", input_dir_path_string)),
            TemplateFile::Dir(format!("{}", output_dir_path_string)),

            TemplateFile::File(format!("{}/blee.txt", template_dir_path_string)),
            TemplateFile::File(format!("{}/blah.txt", template_dir_path_string)),
            TemplateFile::File(format!("{}/input2.json", input_dir_path_string)),
            TemplateFile::File(format!("{}/output2.json", output_dir_path_string)),
        ]
      };

      assert_ignores(&source_files, ignores, expected_files)
    }


    fn assert_ignores<G, F>(source_files: &[InputFileType], ignores_fn: G, expected_files_fn: F) -> io::Result<()> where
      G: FnOnce(TemplateDirectory, InputDirectory, OutputDirectory, WorkingDirectory) -> Vec<String>,
      F: FnOnce(TemplateDirectory, InputDirectory, OutputDirectory, WorkingDirectory) -> Vec<TemplateFile>
    {
      let temp_dir = tempdir()?; // create a temporary working directory
      let template_files_dir = TemplateFilesDir::from(&RepositoryDir::from(temp_dir.path())); // template directory

      std::fs::create_dir(template_files_dir.as_ref()).expect("Could not create temporary template directory"); // create template directory

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

      // Get all the ignored files
      let ignored =
        ignores_fn(
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

      // Execute traversal
      let regex_patterns = RegExFileChooser::new(&template_files_dir, &ignored_refs).expect("Could not create regex patterns");
      let file_chooser = Box::new(regex_patterns);
      let traverser = WalkDirFileTraverser::new(file_chooser);
      let matches = traverser.traverse_files(&template_files_dir);

      eprintln!("template_dir_path_string: {}", &templat_dir_path_string);
      eprintln!("input_dir_path: {}", &input_dir_path_string);
      eprintln!("output_dir_path: {}", &output_dir_path_string);
      eprintln!("working_dir: {}", &working_dir_path_string);

      // Get the expected files
      let expected_matches =
        expected_files_fn(
          TemplateDirectory(templat_dir_path_string.to_owned()),
          InputDirectory(input_dir_path_string.to_owned()),
          OutputDirectory(output_dir_path_string.to_owned()),
          WorkingDirectory(working_dir_path_string.to_owned())
        );

      // Clean up working directory, as we have all the results we need.
      temp_dir.close()?;

      let matches_strings: Vec<_> =
        matches
          .into_iter()
          .map(|t| match t {
            TemplateFile::File(file) => file,
            TemplateFile::Dir(dir) => dir,
          })
          .collect();

      let expected_matches_strings: Vec<_> =
        expected_matches
          .into_iter()
          .map(|t| match t {
            TemplateFile::File(file) => file,
            TemplateFile::Dir(dir) => dir,
          })
          .collect();

      let matches_set: HashSet<String> = HashSet::from_iter(matches_strings);
      let expected_matches_set: HashSet<String> = HashSet::from_iter(expected_matches_strings);

      let mut sorted_matches: Vec<_> = matches_set.intersection(&expected_matches_set).collect();
      sorted_matches.sort();

      eprintln!("Expected files found");
      for f in sorted_matches {
        eprintln!("{}", f)
      }

      let mut missing_sorted_set: Vec<_> = matches_set.difference(&expected_matches_set).collect();
      missing_sorted_set.sort();

      eprintln!();
      eprintln!("Other files");
      for f in missing_sorted_set {
        eprintln!("{}", f)
      }

      let mut sorted_matches: Vec<_> = matches_set.into_iter().collect();
      sorted_matches.sort();

      eprintln!();
      eprintln!("All files at the destination");
      for f in &sorted_matches {
        eprintln!("{}", f)
      }

      let mut sorted_expected_matches: Vec<_> = expected_matches_set.into_iter().collect();
      sorted_expected_matches.sort();

      assert_eq!(sorted_matches, sorted_expected_matches, "Expected the same number of items");

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
