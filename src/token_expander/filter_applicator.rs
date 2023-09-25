use crate::templates::variables::FilterType;

pub trait FilterApplicator {
  fn apply_filter(&self, filter_type: &FilterType, value_to_filter: &str) -> String;
}
