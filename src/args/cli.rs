use clap::{Args as SubArgs, Parser, Subcommand};

/// A simple templating system to prevent copy-pasta overload
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
  #[command(subcommand)]
  pub command: ZatCommand
}


#[derive(Subcommand, Debug, Clone)]
pub enum ZatCommand {
  Process(ProcessTemplatesArgs),
  Bootstrap(BootstrapProjectArgs)
}

#[derive(SubArgs, Debug, Clone)]
pub struct ProcessTemplatesArgs {
   /// The location of the template
   #[clap(long, value_parser)]
   pub template_dir: String,

   /// Where to extract the template to
   #[clap(long, value_parser)]
   pub target_dir: String,

   /// One or more files ignore. Supply multiple times for different files or folders.
   /// The files '.variables.zat-prompt' and '.git' are always specified.
   /// Accepts any valid regular expressions.
   #[clap(long, value_parser,)]
   pub ignores: Vec<String>,

   /// Verbose debug logging
   #[clap(long)]
   pub verbose: bool
}

#[derive(SubArgs, Debug, Clone)]
pub struct BootstrapProjectArgs {
   /// The location of where to create the sample repository
   #[clap(long, value_parser)]
   pub repository_dir: String,
}

pub fn get_cli_args() -> Args {
  Args::parse()
}
