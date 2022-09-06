// [
//   {
//     "variable_name": "project",
//     "description": "Name of project",
//     "prompt": "Please enter your project name"
//   },
//   {
//     "variable_name": "plugin_description",
//     "description": "Explain what your plugin is about",
//     "prompt": "Please enter your plugin description"
//   }
// ]
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateVariable {
  pub variable_name: String,
  pub description: String,
  pub prompt: String
}
