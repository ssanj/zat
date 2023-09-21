use crate::variables::{TemplateVariables, UserVariableKey, UserVariableValue};

use super::expand_filters::ExpandFilters;
use super::convert_case_filter_applicator::ConvertCaseFilterApplicator;
use super::template_variable_expander::TemplateVariableExpander;
use super::default_template_variable_expander::DefaultTemplateVariableExpander;
use super::key_tokenizer::KeyTokenizer;
use super::default_key_tokenizer::DefaultKeyTokenizer;
use super::key_tokenizer::TokenizedKeysExpandedVariables;

pub struct DefaultExpandFilters<'a> {
  token: &'a str
}

impl <'a> DefaultExpandFilters<'a> {
  pub fn new() -> Self {
    Self {
      token: "$"
    }
  }
}

impl <'a> ExpandFilters for DefaultExpandFilters<'a> {
    fn expand_filers(&self, template_variables: TemplateVariables, user_variables: std::collections::HashMap<UserVariableKey, UserVariableValue>) -> TokenizedKeysExpandedVariables {
      // Expands variables names (VARIABLENAME__FILTER_NAME) for each variable supplied and mapped to their supplied values
      let filter_applicator = ConvertCaseFilterApplicator;
      let template_variable_expander = DefaultTemplateVariableExpander::with_filter_applicator(Box::new(filter_applicator));
      let expanded_variables = template_variable_expander.expand_filters(template_variables.clone(), user_variables);

      // Surround expanded variable names with a token - the default is KEY_TOKEN ($)
      let key_tokenizer = DefaultKeyTokenizer::new(self.token);
      key_tokenizer.tokenize_keys(expanded_variables.clone())
    }
}
