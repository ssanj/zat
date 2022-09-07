// [
//   {
//     "variable_name": "project",
//     "description": "Name of project",
//     "prompt": "Please enter your project name"
        // "filters": [
        //   {
        //     "name":"python",
        //     "filter": "snake"
        //   },
        //   { "name": "Command",
        //     "filter": "pascal"
        //   }
        // ]
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
  pub prompt: String,
  #[serde(default)] // use default value if not found in the input
  pub filters: Vec<VariableFilter>
}

#[derive(Debug, Clone, Deserialize)]
pub struct VariableFilter {
  pub name: String,
  pub filter: FilterType // make this an ADT
}

#[derive(Debug, Clone, Deserialize)]
pub enum FilterType {
  Snake,
  Lower,
  Pascal
}
