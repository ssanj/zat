// Public modules
pub mod file_chooser;
pub mod token_replacer;
pub mod template_enricher;
pub mod process_templates;
pub mod default_process_templates;
pub mod enriched_template_file_processor;
pub mod file_writer;
pub mod directory_creator;
pub mod string_token_replacer;

// Module-private modules
mod regex_file_chooser;
mod default_template_enricher;
mod walk_dir_file_traverser;
mod aho_corasick_token_replacer;
mod default_file_writer;
mod enriched_default_template_file_processor;
mod default_directory_creator;
mod source_file;
mod destination_file;
mod file_traverser;

// Public exports
pub use file_chooser::FileChooser;
pub use token_replacer::{ContentTokensReplaced, ContentWithTokens, TokenReplacer};
pub use template_enricher::TemplateEnricher;
pub use process_templates::ProcessTemplates;
pub use default_process_templates::DefaultProcessTemplates;
pub use enriched_template_file_processor::{EnrichedTemplateFile, EnrichedTemplateFileProcessor};
pub use file_writer::FileWriter;
pub use directory_creator::DirectoryCreator;
pub use string_token_replacer::StringTokenReplacer;

// Module-private exports
use file_traverser::{FileTraverser, TemplateFile};
use regex_file_chooser::RegExFileChooser;
use default_template_enricher::DefaultTemplateEnricher;
use walk_dir_file_traverser::WalkDirFileTraverser;
use aho_corasick_token_replacer::AhoCorasickTokenReplacer;
use enriched_default_template_file_processor::DefaultEnrichedTemplateFileProcessor;
use source_file::SourceFile;
use destination_file::DestinationFile;

#[cfg(test)]
use string_token_replacer::{ReplacingStringTokenReplacer, EchoingStringTokenReplacer};
