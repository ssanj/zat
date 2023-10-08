use crate::{config::TargetDir, error::ZatAction};


pub trait PostProcessingHook {
  fn run(&self, destination: TargetDir) -> ZatAction;
}
