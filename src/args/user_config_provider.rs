use crate::error::ZatResult;
use crate::config::UserConfig;


/// Behaviour to return configuration provided by the "user"
pub trait UserConfigProvider {
  /// Returns the UserConfig
  fn get_config(&self) -> ZatResult<UserConfig>;
}
