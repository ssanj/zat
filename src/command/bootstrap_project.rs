use crate::args::BootstrapProjectArgs;
use crate::error::{ZatError, ZatAction};
use crate::config::TargetDir;

pub struct BootstrapProject;

impl BootstrapProject {

  pub fn process_bootstrap(bootstrap_project_args: BootstrapProjectArgs) -> ZatAction {
    let repository_directory = TargetDir::new(&bootstrap_project_args.repository_dir);

    if repository_directory.does_exist() {
      Err(ZatError::bootstrap_repository_dir_should_not_exist(&bootstrap_project_args.repository_dir))
    } else {
      // extract the sample files (return an error if this fails)
      // display a message on how to run the template
      todo!()
    }
  }
}
