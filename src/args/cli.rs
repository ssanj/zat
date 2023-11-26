use clap::Parser;

/// A simple templating system to prevent copy-pasta overload
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {

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

pub fn get_cli_args() -> Args {
  Args::parse()
}
