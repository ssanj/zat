use crate::shared_models::ZatResultX;

use super::process_templates::ProcessTemplates;
use super::regex_file_chooser::RegExFileChooser;
use super::walk_dir_file_traverser::WalkDirFileTraverser;
use super::template_enricher::TemplateEnricher;
use super::default_template_enricher::DefaultTemplateEnricher;
use super::enriched_template_file_processor::{EnrichedTemplateFileProcessor, EnrichedTemplateFile};
use super::enriched_default_template_file_processor::DefaultEnrichedTemplateFileProcessor;
use super::aho_corasick_token_replacer::AhoCorasickTokenReplacer;
use super::file_traverser::FileTraverser;
use crate::config::user_config::UserConfig;

pub struct DefaultProcessTemplates;

impl ProcessTemplates for DefaultProcessTemplates {
    fn process_templates(&self, user_config: UserConfig, tokenized_key_expanded_variables: crate::token_expander::key_tokenizer::TokenizedKeysExpandedVariables) -> crate::shared_models::ZatActionX {
      let ignores: Vec<&str> =
        user_config
          .ignores.ignores
          .iter()
          .map(|i| i.as_str())
          .collect();

      // Choose files to include by respecting ignores
      let file_chooser = RegExFileChooser::new(&ignores).expect("Could not create file chooser");
      let file_traverser = WalkDirFileTraverser::new(Box::new(file_chooser));
      let files_to_process = file_traverser.traverse_files(&user_config.template_dir);

      // Converts template files into enriched files that include replaced file name and content tokens
      let template_enricher = DefaultTemplateEnricher::new(user_config);
      let enriched_template_file_processor = DefaultEnrichedTemplateFileProcessor::with_defaults();

      let aho_token_replacer = AhoCorasickTokenReplacer::new(tokenized_key_expanded_variables.clone());
      // TODO: Move this into an encapsulating module
      files_to_process
        .into_iter()
        .map(|tf| template_enricher.enrich(tf)) // adds relative target file directory paths for each template
        .collect::<ZatResultX<Vec<EnrichedTemplateFile>>>()
        .and_then(|enriched_templates|{
          // Writes out files and directories for each enriched template files while
          // replacing any tokens in the file names and content
          enriched_template_file_processor.process_enriched_template_files(&enriched_templates, &aho_token_replacer)
        })
    }
}
