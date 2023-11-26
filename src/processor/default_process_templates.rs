use std::{println, todo};

use crate::error::ZatError;
use crate::error::{ZatResult, ZatAction};

use super::ProcessTemplates;
use super::RegExFileChooser;
use super::WalkDirFileTraverser;
use super::TemplateEnricher;
use super::DefaultTemplateEnricher;
use super::file_traverser::TemplateFile;
use super::{EnrichedTemplateFileProcessor, EnrichedTemplateFile};
use super::DefaultEnrichedTemplateFileProcessor;
use super::AhoCorasickTokenReplacer;
use super::FileTraverser;
use crate::config::{UserConfig, TemplateFilesDir};

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
      let file_chooser = RegExFileChooser::new(&ignores).expect("Could not create file chooser");
      let file_traverser = WalkDirFileTraverser::new(Box::new(file_chooser));
      let template_files_dir = &user_config.template_files_dir;
      let files_to_process = file_traverser.traverse_files(&template_files_dir);
      let template_files: Vec<_> =
        files_to_process
          .iter()
          .map(|tf| tf)
          .collect();

      // Converts template files into enriched files that include replaced file name and content tokens
      let template_enricher = DefaultTemplateEnricher::new(user_config.clone());
      let enriched_template_file_processor = DefaultEnrichedTemplateFileProcessor::with_defaults();

      let aho_token_replacer = AhoCorasickTokenReplacer::new(tokenized_key_expanded_variables.clone());

      if user_config.verbose {
        println!("{:?} ==============================> {:?}", &template_files_dir, files_to_process);
      }

      if self.has_template_files(&template_files, &template_files_dir) {
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
      _ => true // not empty
    }
  }
}
