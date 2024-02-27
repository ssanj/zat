use crate::error::ZatError;
use crate::error::{ZatResult, ZatAction};
use crate::logging::VerboseLogger;

use super::default_directory_creator::DefaultDirectoryCreator;
use super::ProcessTemplates;
use super::RegExFileChooser;
use super::WalkDirFileTraverser;
use super::TemplateEnricher;
use super::DefaultTemplateEnricher;
use super::default_file_writer::DefaultFileWriter;
use super::file_traverser::TemplateFile;
use super::{EnrichedTemplateFileProcessor, EnrichedTemplateFile};
use super::DefaultEnrichedTemplateFileProcessor;
use super::AhoCorasickTokenReplacer;
use super::FileTraverser;
use crate::config::{UserConfig, TemplateFilesDir};
use std::format as s;

pub struct DefaultProcessTemplates;

impl ProcessTemplates for DefaultProcessTemplates {
    fn process_templates(&self, user_config: UserConfig, tokenized_key_expanded_variables: crate::token_expander::key_tokenizer::TokenizedKeysExpandedVariables) -> ZatAction {
      let ignores: Vec<&str> =
        user_config
          .ignores.ignores
          .iter()
          .map(|i| i.as_str())
          .collect();

      // Choose files to include by respecting ignores
      let file_chooser = RegExFileChooser::new(&user_config.template_files_dir, &ignores).expect("Could not create file chooser");
      let file_traverser = WalkDirFileTraverser::new(Box::new(file_chooser));
      let template_files_dir = &user_config.template_files_dir;
      let files_to_process = file_traverser.traverse_files(template_files_dir);

      // We need a &[&TemplateFile] to pass to `has_template_files`.
      let template_files: Vec<_> =
        files_to_process
          .iter()
          .collect();

      // Converts template files into enriched files that include replaced file name and content tokens
      let template_enricher = DefaultTemplateEnricher::new(user_config.clone());

      let default_file_writer = DefaultFileWriter::with_user_config(&user_config);
      let default_directory_creator = DefaultDirectoryCreator::with_user_config(&user_config);
      let enriched_template_file_processor = DefaultEnrichedTemplateFileProcessor::new(&default_file_writer, &default_directory_creator, &user_config);

      let aho_token_replacer = AhoCorasickTokenReplacer::new(tokenized_key_expanded_variables.clone());

      DefaultProcessTemplates::log_files_to_process(&user_config, &files_to_process);

      if self.has_template_files(&template_files, template_files_dir) {
        files_to_process
          .into_iter()
          .map(|tf| template_enricher.enrich(tf)) // adds relative target file directory paths for each template
          .collect::<ZatResult<Vec<EnrichedTemplateFile>>>()
          .and_then(|enriched_templates|{
            // Writes out files and directories for each enriched template files while
            // replacing any tokens in the file names and content
            enriched_template_file_processor.process_enriched_template_files(&enriched_templates, &aho_token_replacer)
          })
      } else {
        Err(ZatError::no_template_files_to_process(template_files_dir.path()))
      }
    }
}

impl DefaultProcessTemplates {

  fn has_template_files(&self, template_files: &[&TemplateFile], template_files_dir: &TemplateFilesDir) -> bool {
    match template_files {
      [] => false, //empty, so no files
      [TemplateFile::Dir(one)] => one != template_files_dir.path(), //only contains the template files directory, so technically empty

      // Only contains the template files directory and a .keep file, so technically empty. Wait, what?
      // We need this so we can commit an integration test to verify the expected error when the templates directory
      // doesn't have any files.
      // Git doesn't allow committing empty directories, so we need a .keep file which is
      // technically not part of the template.
      // TODO: Do we need to worry about this order? I've seen directories come through first. We may have to add
      // another clause if that is not the case.
      [TemplateFile::Dir(root_dir), TemplateFile::File(keep_file)] => {
        !(root_dir == template_files_dir.path() &&
          keep_file == &std::path::Path::new(template_files_dir.path()).join(".keep").to_string_lossy().to_string()
        )
      },
      _ => true // not empty
    }
  }

  fn log_files_to_process(user_config: &UserConfig, files_to_process: &[TemplateFile]) {
    let files: Vec<String> =
      files_to_process
        .iter()
        .map(|file|{
          match file {
            TemplateFile::File(file) => {
                s!("file: {}", file)
            },
            TemplateFile::Dir(dir) => s!("dir: {}", dir),
          }
        })
        .collect();

    VerboseLogger::log_files_to_process(user_config, files);
  }
}

#[cfg(test)]
mod tests {
  use crate::config::{RepositoryDir, TemplateFilesDir};

use super::*;

  #[test]
  fn has_template_files_returns_false_if_there_are_no_files() {
      let rd = RepositoryDir::new(".");
      let tfd = TemplateFilesDir::from(&rd);
      let has_files = DefaultProcessTemplates.has_template_files(&[], &tfd);
      assert!(!has_files)
  }

  #[test]
  fn has_template_files_returns_false_if_only_file_is_the_template_files_directory() {
      let rd = RepositoryDir::new(".");
      let tfd = TemplateFilesDir::from(&rd);

      let root_dir = TemplateFile::Dir(tfd.path().to_owned());
      let template_files = &[&root_dir];

      let has_files = DefaultProcessTemplates.has_template_files(template_files, &tfd);
      assert!(!has_files)
  }

  #[test]
  fn has_template_files_returns_false_if_theres_only_a_dot_keep_file_in_the_template_files_directory() {
      let rd = RepositoryDir::new(".");
      let tfd = TemplateFilesDir::from(&rd);

      let keep_file = TemplateFile::File(s!("{}/.keep", tfd.path()));
      let root_dir = TemplateFile::Dir(tfd.path().to_owned());
      let template_files = &[&root_dir, &keep_file];

      let has_files = DefaultProcessTemplates.has_template_files(template_files, &tfd);
      assert!(!has_files)
  }

  #[test]
  fn has_template_files_returns_true_if_there_are_any_other_directories() {
      let rd = RepositoryDir::new(".");
      let tfd = TemplateFilesDir::from(&rd);
      let tf = TemplateFile::Dir("some-directory".to_owned());
      let has_files = DefaultProcessTemplates.has_template_files(&[&tf], &tfd);
      assert!(has_files)
  }

  #[test]
  fn has_template_files_returns_true_if_there_are_any_other_files() {
      let rd = RepositoryDir::new(".");
      let tfd = TemplateFilesDir::from(&rd);
      let tf = TemplateFile::File("some-file".to_owned());
      let has_files = DefaultProcessTemplates.has_template_files(&[&tf], &tfd);
      assert!(has_files)
  }

}
