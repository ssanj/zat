use crate::templates::TemplateVariables;
use crate::error::ZatAction;

/// Runs any defined plugins and updates a TemplateVariable with the value.
pub trait PluginRunner {
  fn run_plugins(&self, template_variable: &mut TemplateVariables) -> ZatAction;
}
