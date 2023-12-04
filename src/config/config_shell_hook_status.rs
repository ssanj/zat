#[derive(Debug, Clone, PartialEq)]
pub enum ConfigShellHookStatus {
  NoShellHook,
  RunShellHook(String)
}


impl Default for ConfigShellHookStatus {
  fn default() -> Self {
    Self::NoShellHook
  }
}

