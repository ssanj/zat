pub mod error;
pub mod error_format;
pub mod user_config_error_reason;
pub mod variable_file_error_reason;
pub mod template_processing_error_reason;
pub mod post_processing_error_reason;
pub mod bootstrap_command_error_reason;
pub mod process_remote_command_error_reason;

pub use error::ZatAction;
pub use error::ZatError;
pub use error::ZatResult;

use error_format::ErrorFormat;
use user_config_error_reason::UserConfigErrorReason;
use variable_file_error_reason::VariableFileErrorReason;
use template_processing_error_reason::TemplateProcessingErrorReason;
use template_processing_error_reason::ReasonFileErrorReason;
use post_processing_error_reason::PostProcessingErrorReason;
use bootstrap_command_error_reason::BootstrapCommandErrorReason;
use process_remote_command_error_reason::ProcessRemoteCommandErrorReason;

#[cfg(test)]
pub use error::ProcessCommandErrorReason;
