use super::{Args, ArgSupplier, cli};

pub struct CliArgSupplier;

impl ArgSupplier for CliArgSupplier {
  fn get_args(&self) -> Args {
    cli::get_cli_args()
  }
}
