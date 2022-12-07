use crate::filter_applicator::FilterApplicator;
use crate::variables::FilterType;
use convert_case::{Case, Casing};

struct ConvertCaseFilterApplicator;

  // See: https://docs.rs/convert_case/latest/convert_case/enum.Case.html
impl FilterApplicator for ConvertCaseFilterApplicator {
  fn apply(&self, filter_type: &FilterType, value: &str) -> String {
    match filter_type {
      FilterType::Camel  => value.to_case(Case::Camel),  /* "My variable NAME" -> "myVariableName"   */
      FilterType::Cobol  => value.to_case(Case::Cobol),  /* "My variable NAME" -> "MY-VARIABLE-NAME" */
      FilterType::Flat   => value.to_case(Case::Flat),   /* "My variable NAME" -> "myvariablename"   */
      FilterType::Kebab  => value.to_case(Case::Kebab),  /* "My variable NAME" -> "my-variable-name" */
      FilterType::Lower  => value.to_case(Case::Lower),  /* "My variable NAME" -> "my variable name" */
      FilterType::Noop   => value.to_owned(),            /* "My variable NAME" -> "My variable NAME" */
      FilterType::Pascal => value.to_case(Case::Pascal), /* "My variable NAME" -> "MyVariableName"   */
      FilterType::Snake  => value.to_case(Case::Snake),  /* "My variable NAME" -> "my_variable_name" */
      FilterType::Title  => value.to_case(Case::Title),  /* "My variable NAME" -> "My Variable Name" */
      FilterType::Upper  => value.to_case(Case::Upper),  /* "My variable NAME" -> "MY VARIABLE NAME" */
    }
  }
}


