#[derive(Debug, Clone, PartialEq)]
pub enum ShellHookStatus {
  NoShellHook,
  RunShellHook(String)
}


impl Default for ShellHookStatus {
    fn default() -> Self {
        Self::NoShellHook
    }
}

