use crate::{config::{TargetDir, UserConfig}, error::ZatAction};

use super::PostProcessingHook;

pub struct ShellHook;

impl PostProcessingHook for ShellHook {
  fn run(&self, user_config: &UserConfig) -> ZatAction {
    // if a shell hook is found run it
    println!("post processor called");

    Ok(())
  }
}
