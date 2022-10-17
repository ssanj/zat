pub type ZatResultX<A> = Result<A, ZatErrorX>;
pub type ZatActionX = Result<(), ZatErrorX>;


#[derive(Debug)]
pub enum ZatErrorX {
  UserConfigError(String),
}
