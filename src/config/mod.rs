
pub mod ignored_files;
pub mod variable_file;
pub mod user_config;
pub mod filters;
pub mod repository_directory;
pub mod template_files_directory;
pub mod target_directory;

// Private Module
mod shell_hook_file;
mod config_shell_hook_status;


pub use ignored_files::IgnoredFiles;
pub use variable_file::VariableFile;
pub use variable_file::DOT_VARIABLES_PROMPT;
pub use user_config::UserConfig;
pub use filters::Filters;
pub use repository_directory::RepositoryDir;
pub use template_files_directory::TemplateFilesDir;
pub use target_directory::TargetDir;
pub use shell_hook_file::SHELL_HOOK_FILE;
pub use config_shell_hook_status::ConfigShellHookStatus;
