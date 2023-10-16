use crate::{config::{TargetDir, UserConfig}, error::ZatAction};


pub trait PostProcessingHook {
  fn run(&self, user_config: &UserConfig) -> ZatAction;
}
