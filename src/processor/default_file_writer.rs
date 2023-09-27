use super::FileWriter;
use super::SourceFile;
use super::DestinationFile;
use crate::error::{ZatError, ZatResult};
use super::StringTokenReplacer;
use std::{fs, path::Path, fmt::Display};

pub struct DefaultFileWriter;

impl FileWriter for DefaultFileWriter {

  fn write_source_to_destination(&self, source_file: &SourceFile, destination_file: &DestinationFile, token_replacer: &dyn StringTokenReplacer) -> ZatResult<()> {
    let content = source_file.read()?;

    let target_file_name_tokens_applied = destination_file.map(|df| token_replacer.replace(df));

    if let Some("tmpl") = &target_file_name_tokens_applied.get_extension().as_deref() { // It's a templates
      let parent_dir = &target_file_name_tokens_applied.parent_directory();
      let full_target_file_path_templated = parent_dir.join(&target_file_name_tokens_applied.file_stem());
      let content_with_tokens_applied = token_replacer.replace(&content);
      Self::write_file(&full_target_file_path_templated, &content_with_tokens_applied)
    } else {
      Self::write_file(&target_file_name_tokens_applied, &content)
    }
  }
}

impl DefaultFileWriter {
  fn write_file<C, T>(target_file_with_tokens_replaced: T, content: C) -> ZatResult<()> where
    T: AsRef<Path> + Display,
    C: AsRef<[u8]>
  {
    fs::write(&target_file_with_tokens_replaced, content)
      .map_err(|e| ZatError::WritingFileError(format!("Could not write target file: {}\nCause:{}", &target_file_with_tokens_replaced, e)))
  }
}


#[cfg(test)]
mod tests {
    use std::{io::Read, fs::OpenOptions};

    use super::super::{EchoingStringTokenReplacer, ReplacingStringTokenReplacer};

    use super::*;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn should_write_out_file_without_tokens_in_its_name() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let mut temp_destination_file = NamedTempFile::new().unwrap();

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_file = DestinationFile(temp_destination_file.path().to_string_lossy().to_string());

      let file_writer = DefaultFileWriter;
      let source_content = b"HelloWorld from $project_underscore$";
      fs::write(&source_file, &source_content).unwrap();

      let replacer = EchoingStringTokenReplacer;

      file_writer.write_source_to_destination(
        &source_file,
        &destination_file,
        &replacer
      ).unwrap();

      let mut destination_content = String::new();
      let _ = temp_destination_file.read_to_string(&mut destination_content).unwrap();
      let source_content_utf = std::str::from_utf8(source_content).unwrap();

      assert_eq!(&source_content_utf, &destination_content, "source content should be equal to the destination content");
    }

    #[test]
    fn should_write_out_file_with_tokens_in_its_name() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let temp_destination_dir = tempdir().unwrap();

      let destination_dir = DestinationFile(temp_destination_dir.into_path().to_string_lossy().to_string());

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_file = destination_dir.join("$project_underscore$.py");
      let token_replaced_destination_file = destination_dir.join("my-cool-project.py");

      let file_writer = DefaultFileWriter;
      let source_content = b"HelloWorld from $project_underscore$";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project_underscore$", "my-cool-project")]);

      file_writer.write_source_to_destination(
        &source_file,
        &destination_file,
        &replacer
      ).unwrap();

      let mut destination_content = String::new();

      let mut destination_file =
        fs::OpenOptions::new()
          .read(true)
          .create(false) // don't create this if it does not exist
          .open(&token_replaced_destination_file)
          .expect(&format!("Could not find file: {}", &token_replaced_destination_file));

      let _ = destination_file.read_to_string(&mut destination_content).unwrap();
      let source_content_utf = std::str::from_utf8(source_content).unwrap();

      assert_eq!(&source_content_utf, &destination_content, "source content should be equal to the destination content");
    }

    #[test]
    fn should_write_out_file_with_tokens_in_its_name_but_not_content_if_not_a_template() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let temp_destination_dir = tempdir().unwrap();

      let destination_dir = DestinationFile(temp_destination_dir.into_path().to_string_lossy().to_string());

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_file = destination_dir.join("$project_underscore$.py");
      let token_replaced_destination_file = destination_dir.join("my-cool-project.py");

      let file_writer = DefaultFileWriter;
      let source_content = b"HelloWorld from $project$";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project_underscore$", "my-cool-project"), ("$project$", "My Cool Project")]);

      file_writer.write_source_to_destination(
        &source_file,
        &destination_file,
        &replacer
      ).unwrap();

      let mut destination_content = String::new();

      let mut destination_file =
        OpenOptions::new()
          .read(true)
          .create(false) // don't create this if it does not exist
          .open(&token_replaced_destination_file)
          .expect(&format!("Could not find file: {}", &token_replaced_destination_file));

      let _ = destination_file.read_to_string(&mut destination_content).unwrap();
      let source_content_utf = std::str::from_utf8(source_content).unwrap();

      assert_eq!(&source_content_utf, &destination_content, "source content should be equal to the destination content");
    }

    #[test]
    fn should_write_out_file_with_tokens_in_its_name_and_replace_tokenised_content_in_template_file() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let temp_destination_dir = tempdir().unwrap();

      let destination_dir = DestinationFile(temp_destination_dir.into_path().to_string_lossy().to_string());

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_template_file = destination_dir.join("$project_underscore$.py.tmpl");
      let token_replaced_destination_file = destination_dir.join("my-cool-project.py");

      let file_writer = DefaultFileWriter;
      let source_content = b"HelloWorld from $project$";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project_underscore$", "my-cool-project"), ("$project$", "My Cool Project")]);

      let token_replaced_destination_content = b"HelloWorld from My Cool Project";

      file_writer.write_source_to_destination(
        &source_file,
        &destination_template_file,
        &replacer
      ).unwrap();

      let mut destination_content = String::new();

      let mut destination_file =
        fs::OpenOptions::new()
          .read(true)
          .create(false) // don't create this if it does not exist
          .open(&token_replaced_destination_file)
          .expect(&format!("Could not find file: {}", &token_replaced_destination_file));

      let _ = destination_file.read_to_string(&mut destination_content).unwrap();
      let expected_destination_content = std::str::from_utf8(token_replaced_destination_content).unwrap();

      assert_eq!(&expected_destination_content, &destination_content, "token replaced content should be equal to the destination content");
    }
}
