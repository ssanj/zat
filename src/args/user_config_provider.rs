use crate::error::ZatResult;
use crate::config::UserConfig;
use super::cli::ProcessTemplatesArgs;


/// Behaviour to return configuration provided by the "user"
pub trait UserConfigProvider {

  /// Returns the UserConfig
  fn get_user_config(&self, args: ProcessTemplatesArgs) -> ZatResult<UserConfig>;
}
