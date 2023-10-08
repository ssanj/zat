use crate::{config::TargetDir, error::ZatAction};

use super::PostProcessingHook;

pub struct ShellHook;

impl PostProcessingHook for ShellHook {
  fn run(&self, destination: TargetDir) -> ZatAction {
    println!("post processor called with: {}", destination.path);

    Ok(())
  }
}
