use super::FileWriter;
use super::SourceFile;
use super::DestinationFile;
use crate::config::UserConfig;
use crate::error::ZatAction;
use crate::error::{ZatError, ZatResult};
use crate::logging::VerboseLogger;
use crate::templates::UserChoices;
use super::StringTokenReplacer;
use std::{fs, path::Path, fmt::Display, format as s};
use tera::{Tera, Context};

#[allow(dead_code)]
pub struct DefaultFileWriter<'a> {
  user_config: &'a UserConfig,
  user_choices: &'a UserChoices,
  context: Context,
}

impl FileWriter for DefaultFileWriter<'_> {

  fn write_source_to_destination(&self, source_file: &SourceFile, destination_file: &DestinationFile, token_replacer: &dyn StringTokenReplacer) -> ZatResult<()> {
    let target_file_name_tokens_applied = destination_file.map(|df| token_replacer.replace(df));

    if let Some("tmpl") = &target_file_name_tokens_applied.get_extension().as_deref() { // It's a template
      VerboseLogger::log_content(self.user_config, &s!("Writing template file: {}", &target_file_name_tokens_applied));
      let mut content = source_file.read_text()?;

      let tera_tokens =
        [
          "{% if",
          "{%if",
          "{%- if",
          "{%-if",
        ];

      if self.user_choices.has_choices() && tera_tokens.iter().any(|tt| content.contains(tt)) { // It's Tera template, with an 'if' condition.
        VerboseLogger::log_content(self.user_config, &s!("Found Tera template file: {}", &target_file_name_tokens_applied));
        // Mutates content by rendering the Tera template
        self.render_str(&mut content, &source_file.0)?;
      }

      let parent_dir = &target_file_name_tokens_applied.parent_directory();
      let full_target_file_path_templated = parent_dir.join(target_file_name_tokens_applied.file_stem());
      let content_with_tokens_applied = token_replacer.replace(&content);
      Self::write_file(full_target_file_path_templated, content_with_tokens_applied)
    } else {
      VerboseLogger::log_content(self.user_config, &s!("Copying file: {}", &target_file_name_tokens_applied));
      let content = source_file.read_binary()?;
      Self::write_file(target_file_name_tokens_applied, content)
    }
  }
}

impl <'a> DefaultFileWriter<'a> {

  pub fn new(user_config: &'a UserConfig, user_choices: &'a UserChoices) -> Self {
    let mut context = Context::new();

    for (k, v) in user_choices.value.iter() {
      context.insert(k.value.as_str(), v.value.value.as_str())
    }

    Self {
      user_config,
      user_choices,
      context
    }
  }

  fn render_str(&self, input: &mut String, file: &str) -> ZatAction {
    let new_content =
      Tera::one_off(input, &self.context, false)
        .map_err(|e| {
          let offending_lines =
            input
              .split('\n')
              .filter(|line| line.contains(r#"{% if"#) || line.contains(r#"{%if"#))
              .collect::<Vec<_>>();

          ZatError::could_not_render_conditional_template(file, &offending_lines, e.to_string())
        })?;
    *input = new_content;
    Ok(())
  }

  fn write_file<C, T>(target_file_with_tokens_replaced: T, content: C) -> ZatResult<()> where
    T: AsRef<Path> + Display,
    C: AsRef<[u8]>
  {
    fs::write(&target_file_with_tokens_replaced, content)
      .map_err(|e| ZatError::could_not_write_output_file(target_file_with_tokens_replaced.to_string().as_str(), e.to_string()))
  }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::{io::Read, fs::OpenOptions};
    use super::super::{EchoingStringTokenReplacer, ReplacingStringTokenReplacer};
    use super::*;
    use tempfile::{tempdir, NamedTempFile};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_write_out_file_without_tokens_in_its_name() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let mut temp_destination_file = NamedTempFile::new().unwrap();

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_file = DestinationFile(temp_destination_file.path().to_string_lossy().to_string());

      let user_config = UserConfig::default();
      let user_choices = UserChoices::default();
      let file_writer = DefaultFileWriter::new(&user_config, &user_choices);
      let source_content = b"HelloWorld from $project__underscore$";
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
      let destination_file = destination_dir.join("$project__underscore$.py");
      let token_replaced_destination_file = destination_dir.join("my-cool-project.py");

      let user_config = UserConfig::default();
      let user_choices = UserChoices::default();
      let file_writer = DefaultFileWriter::new(&user_config, &user_choices);
      let source_content = b"HelloWorld from $project__underscore$";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project__underscore$", "my-cool-project")]);

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
      let destination_file = destination_dir.join("$project__underscore$.py");
      let token_replaced_destination_file = destination_dir.join("my-cool-project.py");

      let user_config = UserConfig::default();
      let user_choices = UserChoices::default();
      let file_writer = DefaultFileWriter::new(&user_config, &user_choices);
      let source_content = b"HelloWorld from $project$";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project__underscore$", "my-cool-project"), ("$project$", "My Cool Project")]);

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
    fn should_write_out_file_with_tokens_in_its_name_but_not_content_and_not_tera_template_if_not_a_template() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let temp_destination_dir = tempdir().unwrap();

      let destination_dir = DestinationFile(temp_destination_dir.into_path().to_string_lossy().to_string());

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_file = destination_dir.join("$project__underscore$.py");
      let token_replaced_destination_file = destination_dir.join("my-cool-project.py");

      let user_config = UserConfig::default();
      let user_choices = UserChoices::default();
      let file_writer = DefaultFileWriter::new(&user_config, &user_choices);
      let source_content = b"HelloWorld from {% if option=\"1\" %}my first{% else %}my second{% endif %} $project$";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project__underscore$", "my-cool-project"), ("$project$", "My Cool Project")]);

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
      let destination_template_file = destination_dir.join("$project__underscore$.py.tmpl");
      let token_replaced_destination_file = destination_dir.join("my-cool-project.py");

      let user_config = UserConfig::default();
      let user_choices = UserChoices::default();
      let file_writer = DefaultFileWriter::new(&user_config, &user_choices);
      let source_content = b"HelloWorld from $project$";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project__underscore$", "my-cool-project"), ("$project$", "My Cool Project")]);

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


    #[test]
    fn should_write_out_file_without_tokens_in_its_name_and_replace_tokenised_content_in_template_file() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let temp_destination_dir = tempdir().unwrap();

      let destination_dir = DestinationFile(temp_destination_dir.into_path().to_string_lossy().to_string());

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_template_file = destination_dir.join("myproject.py.tmpl");
      let template_replaced_destination_file = destination_dir.join("myproject.py");

      let user_config = UserConfig::default();
      let user_choices = UserChoices::default();
      let file_writer = DefaultFileWriter::new(&user_config, &user_choices);
      let source_content = b"HelloWorld from $project$";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project$", "My Cool Project")]);

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
          .open(&template_replaced_destination_file)
          .expect(&format!("Could not find file: {}", &template_replaced_destination_file));

      let _ = destination_file.read_to_string(&mut destination_content).unwrap();
      let expected_destination_content = std::str::from_utf8(token_replaced_destination_content).unwrap();

      assert_eq!(&expected_destination_content, &destination_content, "token replaced content should be equal to the destination content");
    }

    #[test]
    fn should_write_out_file_without_tokens_in_its_name_and_replace_tokenised_content_in_template_file_and_execute_tera_template() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let temp_destination_dir = tempdir().unwrap();

      let destination_dir = DestinationFile(temp_destination_dir.into_path().to_string_lossy().to_string());

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_template_file = destination_dir.join("myproject.py.tmpl");
      let template_replaced_destination_file = destination_dir.join("myproject.py");

      let user_config = UserConfig::default();
      let user_choices = UserChoices::new(HashMap::from_iter([("project_type".into(), ("X", "Y", "first").into())]));
      let file_writer = DefaultFileWriter::new(&user_config, &user_choices);
      let source_content = b"HelloWorld from {% if project_type == \"first\" %}first $project${% else %}next $project${% endif%}";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project$", "Cool Project")]);

      let token_replaced_destination_content = b"HelloWorld from first Cool Project";

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
          .open(&template_replaced_destination_file)
          .expect(&format!("Could not find file: {}", &template_replaced_destination_file));

      let _ = destination_file.read_to_string(&mut destination_content).unwrap();
      let expected_destination_content = std::str::from_utf8(token_replaced_destination_content).unwrap();

      assert_eq!(&expected_destination_content, &destination_content, "token replaced content should be equal to the destination content");
    }

    #[test]
    fn should_not_execute_tera_template_in_a_non_template_file() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let temp_destination_dir = tempdir().unwrap();

      let destination_dir = DestinationFile(temp_destination_dir.into_path().to_string_lossy().to_string());

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_template_file = destination_dir.join("myproject.py");

      let user_config = UserConfig::default();
      let user_choices = UserChoices::new(HashMap::from_iter([("project_type".into(), ("X", "Y", "first").into())]));
      let file_writer = DefaultFileWriter::new(&user_config, &user_choices);
      let source_content = b"HelloWorld from {% if project_type == \"first\" %}first $project${% else %}next $project${% endif%}";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project$", "Cool Project")]);

      let token_replaced_destination_content = b"HelloWorld from {% if project_type == \"first\" %}first $project${% else %}next $project${% endif%}";

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
          .open(&destination_template_file)
          .expect(&format!("Could not find file: {}", &destination_template_file));

      let _ = destination_file.read_to_string(&mut destination_content).unwrap();
      let expected_destination_content = std::str::from_utf8(token_replaced_destination_content).unwrap();

      assert_eq!(&expected_destination_content, &destination_content, "token replaced content should be equal to the destination content");
    }

    #[test]
    fn should_replace_tokenised_content_in_template_file_and_ignore_raw_tera_template() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let temp_destination_dir = tempdir().unwrap();

      let destination_dir = DestinationFile(temp_destination_dir.into_path().to_string_lossy().to_string());

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_template_file = destination_dir.join("myproject.py.tmpl");
      let template_replaced_destination_file = destination_dir.join("myproject.py");

      let user_config = UserConfig::default();
      let user_choices = UserChoices::new(HashMap::from_iter([("project_type".into(), ("X", "Y", "first").into())]));
      let file_writer = DefaultFileWriter::new(&user_config, &user_choices);
      let source_content = b"HelloWorld from {% raw %}{% if project_type == \"first\" %}first $project${% else %}next $project${% endif%}{% endraw %}";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project$", "Cool Project")]);

      let token_replaced_destination_content = b"HelloWorld from {% if project_type == \"first\" %}first Cool Project{% else %}next Cool Project{% endif%}";

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
          .open(&template_replaced_destination_file)
          .expect(&format!("Could not find file: {}", &template_replaced_destination_file));

      let _ = destination_file.read_to_string(&mut destination_content).unwrap();
      let expected_destination_content = std::str::from_utf8(token_replaced_destination_content).unwrap();

      assert_eq!(&expected_destination_content, &destination_content, "token replaced content should be equal to the destination content");
    }

    #[test]
    fn should_write_out_file_with_tokens_in_its_name_and_replace_tokenised_content_in_template_file_and_execute_tera_template() {
      let temp_source_file = NamedTempFile::new().unwrap();
      let temp_destination_dir = tempdir().unwrap();

      let destination_dir = DestinationFile(temp_destination_dir.into_path().to_string_lossy().to_string());

      let source_file = SourceFile(temp_source_file.path().to_string_lossy().to_string());
      let destination_template_file = destination_dir.join("$project__underscore$.py.tmpl");
      let token_replaced_destination_file = destination_dir.join("my_cool_project.py");

      let user_config = UserConfig::default();
      let user_choices = UserChoices::new(HashMap::from_iter([("project_type".into(), ("X", "Y", "second").into())]));
      let file_writer = DefaultFileWriter::new(&user_config, &user_choices);
      let source_content = b"HelloWorld from {% if project_type == \"first\" %}first $project${% else %}next $project${% endif%}";
      fs::write(&source_file, &source_content).unwrap();

      let replacer =
        ReplacingStringTokenReplacer::new(&[("$project__underscore$", "my_cool_project"), ("$project$", "Cool Project")]);

      let token_replaced_destination_content = b"HelloWorld from next Cool Project";

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
