use clap::{Args as SubArgs, Parser, Subcommand};

/// A simple templating system to prevent copy-pasta overload.
///
/// To get started generate a bootstrap project and follow the instructions.
///
/// General repository structure:
///
/// - All Zat configuration files go in the root of the Zat repository. These include the '.variables.zat-prompt' configuration file and the 'shell-hook.zat-exec' shell hook file. The '.variables.zat-prompt' defines any tokens you want replaced. The values for these tokens will be requested from the user when the template is processed. The optional 'shell-hook.zat-exec' file should be an executable file (chmod +x). It will get invoked after the repository has been processed, with single argument of the target directory path. Use this file to handle any post-processing tasks.
///
/// - All templated files go in the 'templates' folder under the Zat repository folder. This can include regular files, files and folders with tokenised names and templates.
///
///  Regular files: Plain old files without any tokens in their name or in their content. These will get copied "as is" to the target directory when the repository is processed. Note: attributes of a file will not be copied. If you need some attributes maintained for a file, you can do that through a shell hook file.
///
///  Files and folders with tokenised names: Files and folders with tokens in their name but not in their content. Eg. '$project$_README.md'. These tokens will be replaced when the repository is processed and files and folders will be written to the target directory with the updated names.
///
/// Templates: Are files that end with a '.tmpl'. Eg. 'README.md.tmpl'. They can have tokens in their name and in their content. The tokens in their names and content will get replaced when the repository is processed. The '.tmpl' suffix is removed when the processed template is written to the target directory.
///
/// See https://github.com/ssanj/zat for more information on the '.variables.zat-prompt' format and more examples.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
  #[command(subcommand)]
  pub command: ZatCommand
}


#[derive(Subcommand, Debug, Clone)]
pub enum ZatCommand {
  /// Process templates defined in a Zat repository
  Process(ProcessTemplatesArgs),

  /// Generate a minimal bootstrap Zat repository
  Bootstrap(BootstrapProjectArgs),

  /// Process templates defined in a remote Zat repository
  ProcessRemote(ProcessRemoteTemplatesArgs),
}

#[derive(SubArgs, Debug, Clone)]
pub struct ProcessTemplatesArgs {
   /// The location of the Zat repository. This should exist.
   #[arg(long, value_parser)]
   pub repository_dir: String,

   /// Where to extract the template to. This should directory should not exist.
   #[arg(long, value_parser)]
   pub target_dir: String,

   /// One or more files to ignore within the 'template' directory. Supply multiple times for different files or folders.
   /// '.git' are always specified.
   /// Accepts any valid regular expressions.
   #[arg(long, value_parser,)]
   pub ignores: Vec<String>,

   /// Verbose debug logging
   #[arg(long)]
   pub verbose: bool
}

#[derive(SubArgs, Debug, Clone)]
pub struct BootstrapProjectArgs {

   /// The location of where to create the sample repository. This should directory should not exist.
   #[arg(long, value_parser)]
   pub repository_dir: String,
}

#[derive(SubArgs, Debug, Clone)]
pub struct ProcessRemoteTemplatesArgs {

  /// Remote http(s) URL of a Git repository.
  #[arg(long, value_parser)]
  pub repository_url: String,

   /// Where to extract the template to. This should directory should not exist.
   #[arg(long, value_parser)]
   pub target_dir: String,

   /// One or more files ignore. Supply multiple times for different files or folders.
   /// The files '.variables.zat-prompt' and '.git' are always specified.
   /// Accepts any valid regular expressions.
   #[arg(long, value_parser,)]
   pub ignores: Vec<String>,

   /// Verbose debug logging
   #[arg(long)]
   pub verbose: bool
}


pub fn get_cli_args() -> Args {
  Args::parse()
}
