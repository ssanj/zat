use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {

   /// The location of the template
   #[clap(short, long, value_parser)]
   pub template: String,

   /// Where to extract the template to
   #[clap(short, long, value_parser)]
   pub destination: String,
}

pub fn get_cli_args() -> Args {
  Args::parse()
}
