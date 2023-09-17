use std::ffi::OsStr;
use std::path::Path;
use std::println;

use crate::default_template_enricher::DefaultTemplateEnricher;
use crate::enriched_default_template_file_processor::DefaultEnrichedTemplateFileProcessor;
use crate::enriched_template_file_processor::EnrichedTemplateFile;
use crate::file_traverser::FileTraverser;
use crate::models::ZatResult;
use crate::shared_models::{ZatActionX, ZatResultX};
use crate::template_enricher::TemplateEnricher;

mod models;
mod variables;
mod tokens;
mod cli;
mod template_processor;
mod user_config_provider;
mod template_variable_provider;
mod template_config_validator;
mod template_selector;
mod template_proc;
mod template_renderer;
mod token_replacer;
mod shared_models;
mod default_user_config_provider;
mod default_template_variable_provider;
mod default_template_config_validator;
mod template_variable_expander;
mod default_template_variable_expander;
mod filter_applicator;
mod convert_case_filter_applicator;
mod aho_corasick_token_replacer;
mod key_tokenizer;
mod default_key_tokenizer;
mod file_traverser;
mod walk_dir_file_traverser;
mod file_chooser;
mod regex_file_chooser;
mod file_writer;
mod default_file_writer;
mod source_file;
mod destination_file;
mod directory_creator;
mod default_directory_creator;
mod enriched_template_file_processor;
mod enriched_default_template_file_processor;
mod template_enricher;
mod default_template_enricher;
mod string_token_replacer;

const KEY_TOKEN: &str = "$";

fn main() {
  run_zat()
}

fn run_zat() {
  use default_user_config_provider::DefaultUserConfigProvider;
  use user_config_provider::UserConfigProvider;

  use template_variable_provider::TemplateVariableProvider;
  use default_template_variable_provider::DefaultTemplateVariableProvider;

  use template_config_validator::TemplateConfigValidator;
  use default_template_config_validator::DefaultTemplateConfigValidator;

  use template_variable_expander::TemplateVariableExpander;
  use default_template_variable_expander::DefaultTemplateVariableExpander;
  use convert_case_filter_applicator::ConvertCaseFilterApplicator;
  use key_tokenizer::KeyTokenizer;
  use default_key_tokenizer::DefaultKeyTokenizer;
  use crate::template_config_validator::TemplateVariableReview;
  use crate::template_config_validator::ValidConfig;
  use aho_corasick_token_replacer::AhoCorasickTokenReplacer;
  use walk_dir_file_traverser::WalkDirFileTraverser;

  use crate::enriched_template_file_processor::EnrichedTemplateFileProcessor;
  use crate::DefaultEnrichedTemplateFileProcessor;

  let config_provider = DefaultUserConfigProvider::new();
  let user_config = config_provider.get_config().unwrap();

  let template_variable_provider = DefaultTemplateVariableProvider::new();
  let template_variables = template_variable_provider.get_tokens(user_config.clone()).unwrap();

  let template_config_validator = DefaultTemplateConfigValidator::new();
  let template_variable_review = template_config_validator.validate(user_config.clone(), template_variables.clone());

  let filter_applicator = ConvertCaseFilterApplicator;
  let template_variable_expander = DefaultTemplateVariableExpander::with_filter_applicator(Box::new(filter_applicator));

  println!("config: {:?}", user_config);
  println!("variables: {:?}", template_variables);
  println!("variable review: {:?}", template_variable_review);

  match template_variable_review {
    TemplateVariableReview::Accepted(ValidConfig { user_variables, user_config: _ }) => {
      let expanded_variables = template_variable_expander.expand_filters(template_variables.clone(), user_variables);
      let key_tokenizer = DefaultKeyTokenizer::new(KEY_TOKEN);
      let tokenized_key_expanded_variables = key_tokenizer.tokenize_keys(expanded_variables.clone());
      let aho_token_replacer = AhoCorasickTokenReplacer::new(tokenized_key_expanded_variables.clone());

      // TODO: This should be moved elsewhere
      let ignores: Vec<&str> =
        user_config
          .ignores.ignores
          .iter()
          .map(|i| i.as_str())
          .collect();

      let file_chooser = regex_file_chooser::RegExFileChooser::new(&ignores).expect("Could not create file chooser");
      let file_traverser = WalkDirFileTraverser::new(Box::new(file_chooser));
      let files_to_process = file_traverser.traverse_files(&user_config.template_dir);

      let template_enricher = DefaultTemplateEnricher::new(user_config);
      let enriched_template_file_processor = DefaultEnrichedTemplateFileProcessor::with_defaults();

      // TODO: Move this into an encapsulating module
      let zat_results: ZatActionX =
        files_to_process
          .into_iter()
          .map(|tf| template_enricher.enrich(tf))
          .collect::<ZatResultX<Vec<EnrichedTemplateFile>>>()
          .and_then(|enriched_templates|{
            enriched_template_file_processor.process_enriched_template_files(&enriched_templates, &aho_token_replacer)
          });

      match zat_results {
        Ok(()) => println!("Zat completed successfully"),
        Err(error) => println!("Zat got an error: {}", error)
      }
    },
    TemplateVariableReview::Rejected => println!("The user rejected the variables.")
  }
}



