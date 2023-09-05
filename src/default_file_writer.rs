use crate::file_writer::FileWriter;
use crate::source_file::SourceFile;
use crate::destination_file::DestinationFile;
use crate::shared_models::{ZatErrorX, ZatResultX};
use std::{fs, todo, path::Path, fmt::Display};
use std::io;

pub struct DefaultFileWriter;

impl FileWriter for DefaultFileWriter {

  fn write_source_to_destination<T>(&self, source_file: &SourceFile, destination_file: &DestinationFile, token_replacer: T) -> ZatResultX<()>
    where T: Fn(&str) -> String {
     let content = source_file.read()?;

    // let target_file_name_tokens_applied = target_file.map(&replace_tokens);
    let target_file_name_tokens_applied = destination_file;

    // if let Some("tmpl") = &target_file.get_extension().as_deref() { // It's a template

    //   let parent_dir = &target_file_name_tokens_applied.parent_directory();
    //   let full_target_file_path_templated = parent_dir.join(&target_file_name_tokens_applied.file_stem());
    //   let content_with_tokens_applied = &replace_tokens(&content);
    //   write_file(&full_target_file_path_templated, &content_with_tokens_applied)
    // } else {
      Self::write_file(&target_file_name_tokens_applied, &content)
    }
}

impl DefaultFileWriter {
  fn write_file<C, T>(target_file_with_tokens_replaced: T, content: C) -> ZatResultX<()> where
    T: AsRef<Path> + Display,
    C: AsRef<[u8]>
  {
    fs::write(&target_file_with_tokens_replaced, content)
      .map_err(|e| ZatErrorX::WritingFileError(format!("Could not write target file: {}\nCause:{}", &target_file_with_tokens_replaced, e)))
  }
}


#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;
    use tempfile::{tempdir, tempfile, NamedTempFile};

    #[test]
    fn should_write_out_file_without_tokens_in_its_name() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let mut temp_destination_file = NamedTempFile::new().unwrap();

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_file = DestinationFile(temp_destination_file.path().to_string_lossy().to_string());

      let file_writer = DefaultFileWriter;
      let source_content = b"HelloWorld";
      fs::write(&source_file, &source_content).unwrap();

      let replacer = |c:&str| c.to_owned();

      file_writer.write_source_to_destination(
        &source_file,
        &destination_file,
        replacer
      ).unwrap();

      let mut destination_content = String::new();
      let _ = temp_destination_file.read_to_string(&mut destination_content).unwrap();
      let source_content_utf = std::str::from_utf8(source_content).unwrap();

      assert_eq!(&source_content_utf, &destination_content, "source content should be equal to the destination content");
    }
}
