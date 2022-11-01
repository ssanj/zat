pub type ZatResultX<A> = Result<A, ZatErrorX>;
pub type ZatActionX = Result<(), ZatErrorX>;


#[derive(Debug, PartialEq)]
pub enum ZatErrorX {
  UserConfigError(String),
  VariableOpenError(String),
  VariableReadError(String),
  VariableDecodeError(String),
}

impl std::fmt::Display for ZatErrorX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let string_rep = match self {
        ZatErrorX::UserConfigError(error) => format!("UserConfigError({})", error),
        ZatErrorX::VariableOpenError(error) => format!("VariableOpenError({})", error),
        ZatErrorX::VariableReadError(error) => format!("VariableReadError({})", error),
        ZatErrorX::VariableDecodeError(error) => format!("VariableDecodeError({})", error),
      };

      write!(f, "{}", string_rep)
    }
}
