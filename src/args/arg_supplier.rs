use super::Args;

/// Behaviour to return arguments supplied when running Zat.
pub trait ArgSupplier {

  /// Returns the arguments supplied to the program.
  fn get_args(&self) -> Args;
}
