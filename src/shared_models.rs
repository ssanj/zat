pub type ZatResultX<A> = Result<A, ZatErrorX>;
pub type ZatActionX = Result<(), ZatErrorX>;


#[derive(Debug, PartialEq)]
pub enum ZatErrorX {
  UserConfigError(String),
  VariableReadError(String),
  VariableDecodeError(String),
}
