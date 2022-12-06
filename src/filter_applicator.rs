use crate::variables::FilterType;

pub trait FilterApplicator {
  fn apply(&self, filter_type: FilterType, value_to_filter: &str) -> String;
}
