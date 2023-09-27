use crate::error::{ZatResult, ZatActionX};

use super::ProcessTemplates;
use super::RegExFileChooser;
use super::WalkDirFileTraverser;
use super::TemplateEnricher;
use super::DefaultTemplateEnricher;
use super::{EnrichedTemplateFileProcessor, EnrichedTemplateFile};
use super::DefaultEnrichedTemplateFileProcessor;
use super::AhoCorasickTokenReplacer;
use super::FileTraverser;
use crate::config::UserConfig;

pub struct DefaultProcessTemplates;

impl ProcessTemplates for DefaultProcessTemplates {
    fn process_templates(&self, user_config: UserConfig, tokenized_key_expanded_variables: crate::token_expander::key_tokenizer::TokenizedKeysExpandedVariables) -> ZatActionX {
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
        .collect::<ZatResult<Vec<EnrichedTemplateFile>>>()
        .and_then(|enriched_templates|{
          // Writes out files and directories for each enriched template files while
          // replacing any tokens in the file names and content
          enriched_template_file_processor.process_enriched_template_files(&enriched_templates, &aho_token_replacer)
        })
    }
}
