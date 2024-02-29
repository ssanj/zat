pub mod zat_error;
pub mod error_format;
pub mod user_config_error_reason;
pub mod variable_file_error_reason;
pub mod template_processing_error_reason;
pub mod post_processing_error_reason;
pub mod bootstrap_command_error_reason;
pub mod process_remote_command_error_reason;
pub mod plugin_error_reason;
pub mod generic_error_reason;

pub use zat_error::ZatAction;
pub use zat_error::ZatError;
pub use zat_error::ZatResult;

use error_format::ErrorFormat;
use user_config_error_reason::UserConfigErrorReason;
use variable_file_error_reason::VariableFileErrorReason;
use template_processing_error_reason::TemplateProcessingErrorReason;
use template_processing_error_reason::ReasonFileErrorReason;
use post_processing_error_reason::PostProcessingErrorReason;
use bootstrap_command_error_reason::BootstrapCommandErrorReason;
use process_remote_command_error_reason::ProcessRemoteCommandErrorReason;
use plugin_error_reason::PluginErrorReason;
use generic_error_reason::GenericErrorReason;

#[cfg(test)]
pub use zat_error::ProcessCommandErrorReason;
