use args::cli;

mod args;

fn main() {
  let args = cli::get_cli_args();
  println!("verbose: {}", args.verbose);
}
