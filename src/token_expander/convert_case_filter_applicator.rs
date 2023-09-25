use super::filter_applicator::FilterApplicator;
use crate::templates::variables::FilterType;
use convert_case::{Case, Casing};

pub struct ConvertCaseFilterApplicator;

  // See: https://docs.rs/convert_case/latest/convert_case/enum.Case.html
impl FilterApplicator for ConvertCaseFilterApplicator {
  fn apply_filter(&self, filter_type: &FilterType, value: &str) -> String {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
      assert_eq!(ConvertCaseFilterApplicator.apply_filter(&FilterType::Camel,  "Hello cool World"), "helloCoolWorld");
      assert_eq!(ConvertCaseFilterApplicator.apply_filter(&FilterType::Cobol,  "Hello cool World"), "HELLO-COOL-WORLD");
      assert_eq!(ConvertCaseFilterApplicator.apply_filter(&FilterType::Flat,   "Hello cool World"), "hellocoolworld");
      assert_eq!(ConvertCaseFilterApplicator.apply_filter(&FilterType::Kebab,  "Hello cool World"), "hello-cool-world");
      assert_eq!(ConvertCaseFilterApplicator.apply_filter(&FilterType::Lower,  "Hello cool World"), "hello cool world");
      assert_eq!(ConvertCaseFilterApplicator.apply_filter(&FilterType::Noop,   "Hello cool World"), "Hello cool World");
      assert_eq!(ConvertCaseFilterApplicator.apply_filter(&FilterType::Pascal, "Hello cool World"), "HelloCoolWorld");
      assert_eq!(ConvertCaseFilterApplicator.apply_filter(&FilterType::Snake,  "Hello cool World"), "hello_cool_world");
      assert_eq!(ConvertCaseFilterApplicator.apply_filter(&FilterType::Title,  "Hello cool World"), "Hello Cool World");
      assert_eq!(ConvertCaseFilterApplicator.apply_filter(&FilterType::Upper,  "Hello cool World"), "HELLO COOL WORLD");
    }
}

