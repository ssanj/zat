use std::collections::HashMap;
use std::io::{stdin, BufRead, Read};
use std::fmt::{self, Display};

use super::{Choice, Plugin, TemplateConfigValidator, TemplateVariable, TemplateVariableReview, ValidConfig};
use super::{UserVariableValue, UserVariableKey, UserChoiceKey, UserChoiceValue, TemplateVariables};
use crate::config::UserConfig;
use crate::error::{ZatError, ZatResult};
use crate::templates::PluginRunResult;
use ansi_term::Colour::{Yellow, Green, Blue, Red};
use ansi_term::Style;
use std::{println as p, format as s};
use crate::logging::Logger;


pub struct UserInput {
  variables: HashMap<UserVariableKey, UserVariableValue>,
  choices: HashMap<UserChoiceKey, UserChoiceValue>
}

impl UserInput {
  pub fn new(variables: HashMap<UserVariableKey, UserVariableValue>, choices: HashMap<UserChoiceKey, UserChoiceValue>) -> Self {
    Self {
      variables,
      choices
    }
  }
}

// This is a support trait to TemplateConfigValidator, so we define it here as opposed to in its own module.
trait UserInputProvider {
  fn get_user_input(&self, variables: TemplateVariables) -> ZatResult<UserInput>;
}

trait UserTemplateVariableValidator {
  fn review_user_template_variables(&self, user_config: UserConfig, user_input: UserInput) -> TemplateVariableReview;
}

enum UserVariablesValidity {
  Valid,
  Invalid
}

struct Cli;

#[derive(Debug, Clone, PartialEq)]
enum DynamicValueType {
  DefaultValue(String, String),
  PluginValue(String, String),
  Neither,
}

struct DynamicPair(String, String);

#[derive(Debug, Clone, PartialEq)]
enum ChoiceError {
  CouldNotReadInput(String),
  NotANumber(String),
  OutOfBounds(usize),
}

impl Display for ChoiceError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let prefix = match self {
      ChoiceError::CouldNotReadInput(input) => s!("Could not read input: {}", input),
      ChoiceError::NotANumber(input) => s!("Expected a number for choice, but got: {}", input),
      ChoiceError::OutOfBounds(number) => s!("Invalid index supplied: {}", number),
    };
    write!(f, "{}", prefix)
  }
}

impl UserInputProvider for Cli {
  fn get_user_input(&self, template_variables: TemplateVariables) -> ZatResult<UserInput> {
    let mut token_map = HashMap::new();
    let mut choices_map = HashMap::new();

    for v in template_variables.tokens {
      p!();

      // TODO: Make this an ADT
      if v.choice.is_empty() {
        let default_value = Cli::get_default_value(v.default_value.as_deref());
        let plugin_result_value: Option<PluginRunResult> = Cli::get_plugin_value(v.plugin.as_ref()
          );
        let dynamic_value = Cli::get_dynamic_values(default_value.as_deref(), plugin_result_value.as_ref());

        // Ask the user of values for each token
        match &dynamic_value {
          DynamicValueType::DefaultValue(dstring, _) => p!("{}{}", Yellow.paint(&v.prompt), dstring),
          DynamicValueType::PluginValue(pstring, _) => p!("{}{}", Yellow.paint(&v.prompt), pstring),
          DynamicValueType::Neither => p!("{}", Yellow.paint(&v.prompt)),
        }

        Cli::read_user_input(&mut token_map, &v, &dynamic_value);
      } else {
        let choices = v.choice.iter().collect::<Vec<_>>();
        let choice_value = Cli::get_choice(&v.prompt, &choices)?;
        choices_map.insert(UserChoiceKey::new(v.variable_name), UserChoiceValue::new(choice_value.clone()));
      }
    }

    Ok(UserInput::new(token_map, choices_map))
  }
}

impl UserTemplateVariableValidator for Cli {
    fn review_user_template_variables(&self, user_config: UserConfig, user_input: UserInput) -> TemplateVariableReview {
        Cli::print_user_input(&user_input.variables);
        Cli::print_user_choices(&user_input.choices);
        match  Cli::check_user_input() {
          UserVariablesValidity::Valid => {
            let valid_config = ValidConfig::new(user_input.variables, user_input.choices, user_config);
            TemplateVariableReview::Accepted(valid_config)
          },
          UserVariablesValidity::Invalid => TemplateVariableReview::Rejected,
        }
    }
}


impl Cli {

  fn print_user_input(user_variables: &HashMap<UserVariableKey, UserVariableValue>) {
    Logger::info("Please confirm the variable mappings below are correct:");

    for (k, v) in user_variables.iter() {
      p!("{} -> {}", Blue.paint(k.value.as_str()), Green.paint(v.value.as_str()))
    }
  }

  fn print_user_choices(user_choices: &HashMap<UserChoiceKey, UserChoiceValue>) {
    if !user_choices.is_empty() {
      Logger::info("Please confirm the choices selected below are correct:");

      for (k, v) in user_choices.iter() {
        p!("{} -> {}", Blue.paint(k.value.as_str()), Green.paint(v.value.display.as_str()))
      }
    }
  }

  fn check_user_input() -> UserVariablesValidity {
    // Check if variables are ok
    Logger::coloured(
      &s!("{}{}{}",
        Yellow.paint("Press "),
        Style::new().bold().paint("y"),
        Yellow.paint(" if correct, and any other key if not.")
      )
    );

    let mut user_response = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut user_response).expect("Could not read from stdin"); // Unexpected, so throw
    let line = user_response.lines().next().expect("Could not extract line from buffer"); // Unexpected, so throw

    match line {
      "y" => UserVariablesValidity::Valid,
      _ => UserVariablesValidity::Invalid,
    }
  }

  fn get_dynamic_values(opt_default_value: Option<&str>, plugin_run_result: Option<&PluginRunResult>) -> DynamicValueType {

    let default_type: Option<DynamicPair> =
      opt_default_value.map(|default_value| {
          let default_prompt = s!(". Press {} to accept the default value of: {}.", Style::new().underline().paint("enter"), Green.paint(default_value));

          DynamicPair(default_prompt, default_value.to_owned())
      });

    let plugin_type: Option<DynamicPair> =
      plugin_run_result.map(|plugin_result| {
        let plugin_prompt = s!(". Press {} to accept the plugin result value of: {}.", Style::new().underline().paint("enter"), Green.paint(&plugin_result.result));

        DynamicPair(plugin_prompt, plugin_result.clone().result)
      });

    match (default_type, plugin_type) {
      (Some(DynamicPair(dprompt, dvalue)), None) => DynamicValueType::DefaultValue(dprompt, dvalue),
      (None, Some(DynamicPair(pprompt, pvalue))) => DynamicValueType::PluginValue(pprompt, pvalue.to_owned()),
      (Some(_), Some(DynamicPair(pprompt, pvalue))) => DynamicValueType::PluginValue(pprompt, pvalue.to_owned()), // plugin value overrides default value
      (None, None) => DynamicValueType::Neither,
    }
  }

  fn read_user_input(token_map: &mut HashMap<UserVariableKey, UserVariableValue>, template_variable: &TemplateVariable, dynamic_value: &DynamicValueType) {

    let mut variable_value = String::new();

    if let Ok(read_count) = stdin().read_line(&mut variable_value) {
      if read_count > 0 { // Read at least one character
        let _ = variable_value.pop(); // Remove newline //TODO: trim here
        if !variable_value.is_empty() { // User entered a value
          token_map.insert(UserVariableKey::new(template_variable.variable_name.clone()), UserVariableValue::new(variable_value));
        } else { // User pressed enter
          match dynamic_value {
            DynamicValueType::DefaultValue(_, dvalue) => { // Default value
              token_map.insert(UserVariableKey::new(template_variable.variable_name.clone()), UserVariableValue::new(dvalue.to_owned()));
            },
            DynamicValueType::PluginValue(_, pvalue) => { // Plugin value
              token_map.insert(UserVariableKey::new(template_variable.variable_name.clone()), UserVariableValue::new(pvalue.to_owned()));
            },
            // TODO: This allows us to skip entering values for a variable. We should ask for the input again.
            DynamicValueType::Neither => (), // No plugin or default value, so do nothing
          }
        }
      }
    }
  }

  fn get_default_value(opt_default_value: Option<&str>) -> Option<String> {
    opt_default_value.map(|dv| dv.to_owned())
  }

  fn get_plugin_value(opt_plugin_value: Option<&Plugin>) -> Option<PluginRunResult> {
    let plugin = opt_plugin_value?;

    match &plugin.result {
      crate::templates::PluginRunStatus::NotRun => None,
      crate::templates::PluginRunStatus::Run(run_result) => Some(run_result.to_owned()),
    }
  }

  fn print_menu<'a>(prompt: &str, items: &'a [&'a Choice]) -> Result<&'a Choice, ChoiceError> {
    println!("{}", Yellow.paint(prompt));

    let it =
      items
        .iter()
        .enumerate()
        .map(|(n, v)| format!("  {} {} {}", n + 1, v.display, v.description))
        .collect::<Vec<_>>();

    println!("{}", it.join("\n"));

    let mut buffer = String::new();
    stdin()
      .read_line(&mut buffer)
      .map_err(|e| ChoiceError::CouldNotReadInput(e.to_string()))
      .and({
        buffer
          .trim()
          .parse::<usize>()
          .map_err(|_| ChoiceError::NotANumber(buffer.clone()))
          .and_then(|n| {
            if n > 0 && n <= items.len() {
              Ok(
                items[n-1]
              )
            } else {
              Err(ChoiceError::OutOfBounds(n))
            }
          })
      })
  }


  fn get_choice<'a>(prompt: &str, items: &'a [&'a Choice]) -> ZatResult<&'a Choice> {
    let mut result = Self::print_menu(prompt, items);
    while let Err(error) = result {
      let error_message = match error {
        ChoiceError::CouldNotReadInput(error) => format!("Could not read input: {error}"),
        ChoiceError::NotANumber(input) => format!("Selection has to be a number: {} is not a number.", input.trim()),
        ChoiceError::OutOfBounds(index) => format!("Selected index: {} is out of bounds. It should be between 1 - {}", index, items.len())
      };
      println!("{}", Red.paint(error_message));
      println!("press {} to continue", Style::new().underline().paint("ENTER"));
      let mut char_buf = [0;1];
      let _ = stdin().read(&mut char_buf);
      println!();
      println!();
      result = Self::print_menu(prompt, items);
    }

    result.map_err(|e| ZatError::generic_error("Could not get successful result from choice. ERROR_ID: 1000", e.to_string()))
  }
}


pub struct DefaultTemplateConfigValidator {
  user_input_provider: Box<dyn UserInputProvider>,
  user_template_variable_validator: Box<dyn UserTemplateVariableValidator>,
}


impl DefaultTemplateConfigValidator {

  pub fn new() -> Self {
    DefaultTemplateConfigValidator {
      user_input_provider: Box::new(Cli),
      user_template_variable_validator: Box::new(Cli)
    }
  }

  #[cfg(test)]
  fn with_all_dependencies(user_input_provider: Box<dyn UserInputProvider>, user_template_variable_validator: Box<dyn UserTemplateVariableValidator>) -> Self {
    DefaultTemplateConfigValidator {
      user_input_provider,
      user_template_variable_validator
    }
  }
}

impl TemplateConfigValidator for DefaultTemplateConfigValidator {

  fn validate(&self, user_config: UserConfig, template_variables: TemplateVariables) -> ZatResult<TemplateVariableReview> {
      let user_variables = self.user_input_provider.get_user_input(template_variables)?;
      Ok(self.user_template_variable_validator.review_user_template_variables(user_config, user_variables))
  }
}

#[cfg(test)]
mod tests {

use super::super::TemplateVariable;
use super::*;
use pretty_assertions::assert_eq;
use crate::config::user_config::UserConfig;
use crate::templates::PluginRunStatus;

  #[derive(Debug, Default)]
  struct SimpleInput {
    tokens: HashMap<String, String>,
    choices: HashMap<String, (String, String, String)>
  }

  impl SimpleInput {

    fn with_tokens(tokens: HashMap<String, String>) -> Self {
      Self {
        tokens,
        choices: HashMap::default()
      }
    }
  }


  impl UserInputProvider for SimpleInput {
    fn get_user_input(&self, variables: TemplateVariables) -> ZatResult<UserInput> {

      let token_pairs =
        variables
        .tokens
        .iter()
        .filter_map(|tv| {
          self.tokens.get(tv.variable_name.as_str())
            .map(|variable|{
              (UserVariableKey::new(tv.variable_name.to_owned()), UserVariableValue::new(variable.to_owned()))
            })
        });

      let choice_pairs =
        variables
        .tokens
        .iter()
        .filter_map(|tv| {
          self.choices.get(tv.variable_name.as_str())
            .map(|(dis, des, val)|{
              (UserChoiceKey::new(tv.variable_name.to_owned()), UserChoiceValue::from((dis.as_str(), des.as_str(), val.as_str())))
            })
        });

        let variables = HashMap::from_iter(token_pairs);
        let choices = HashMap::from_iter(choice_pairs);

        Ok(UserInput::new(variables, choices))
    }
  }


  struct RejectedUserTemplateVariables;

  struct AcceptedUserTemplateVariables {
    user_config: UserConfig,
    user_variables: HashMap<UserVariableKey, UserVariableValue>,
    user_choices: HashMap<UserChoiceKey, UserChoiceValue>
  }

  impl From<&AcceptedUserTemplateVariables> for ValidConfig {
    fn from(field: &AcceptedUserTemplateVariables) -> Self {
        ValidConfig {
            user_variables: field.user_variables.clone(),
            user_config: field.user_config.clone(),
            user_choices: field.user_choices.clone(),
        }
    }
  }


  impl UserTemplateVariableValidator for RejectedUserTemplateVariables {
    fn review_user_template_variables(&self, _user_config_: UserConfig, _user_input_: UserInput) -> TemplateVariableReview {
        TemplateVariableReview::Rejected
    }
  }

  impl UserTemplateVariableValidator for AcceptedUserTemplateVariables {
    fn review_user_template_variables(&self, _user_config_: UserConfig, _user_input_: UserInput) -> TemplateVariableReview {
      let valid_config: ValidConfig = ValidConfig::from(self);
      TemplateVariableReview::Accepted(valid_config)
    }
  }


  impl Default for RejectedUserTemplateVariables {
    fn default() -> Self {
        RejectedUserTemplateVariables
    }
  }

  fn template_variable_from_name(name: &str) -> TemplateVariable {
    TemplateVariable {
      variable_name: name.to_owned(),
      description: String::default(),
      prompt: String::default(),
      default_value: None,
      plugin: None,
      filters: Vec::default(),
      choice: Vec::default()
    }
  }

  fn user_template_variables(key_values: &[(&str, &str)]) -> HashMap<UserVariableKey, UserVariableValue> {
    key_values.iter().map(|kv|{
      (UserVariableKey::new(kv.0.to_owned()), UserVariableValue::new(kv.1.to_owned()))
    }).collect()
  }

  fn user_choices(key_values: &[(&str, &str, &str, &str)]) -> HashMap<UserChoiceKey, UserChoiceValue> {
    key_values.iter().map(|kv|{
      (UserChoiceKey::new(kv.0.to_owned()), UserChoiceValue::from((kv.1, kv.2, kv.3)))
    }).collect()
  }


  #[test]
  fn returns_valid_user_input() {
    let tokens: HashMap<String, String> =
      HashMap::from([
        ("token1".to_owned(), "value1".to_owned()),
        ("token2".to_owned(), "value2".to_owned())
      ]);

    let input = SimpleInput::with_tokens(tokens);

    let template_variables =
      TemplateVariables {
        tokens: vec![
          template_variable_from_name("tokenA"),
          template_variable_from_name("tokenB"),
          template_variable_from_name("token1"),
          template_variable_from_name("token2"),
          template_variable_from_name("tokenC")
        ]
      };


    let validated_user_variables =
      user_template_variables(
        &[
          ("token1", "value1"),
          ("token2", "value2"),
        ]
      );

    let validated_user_choices =
      user_choices(
        &[
           ("command_type", "Text", "Text Command", "text"),
           ("readme_type", "Long", "A long readme", "long"),
         ]
      );

    let user_config =
      UserConfig::new("template_dir", "target_idr");

    let user_template_variables =
      AcceptedUserTemplateVariables {
        user_config: user_config.clone(),
        user_variables: validated_user_variables,
        user_choices: validated_user_choices,
      };

    let config_validator = DefaultTemplateConfigValidator::with_all_dependencies(Box::new(input), Box::new(user_template_variables));

    let validation_result = config_validator.validate(user_config.clone(), template_variables).expect("validation failed");

    let expected_config =
      ValidConfig {
        user_variables:
          HashMap::from([
            (UserVariableKey::new("token1".to_owned()), UserVariableValue::new("value1".to_owned())),
            (UserVariableKey::new("token2".to_owned()), UserVariableValue::new("value2".to_owned()))
          ]),
        user_choices: HashMap::from([
            (UserChoiceKey::new("command_type".to_owned()), ("Text", "Text Command", "text").into()),
            (UserChoiceKey::new("readme_type".to_owned()), ("Long", "A long readme", "long").into()),
          ]),
        user_config
      };

    assert_eq!(validation_result, TemplateVariableReview::Accepted(expected_config))
  }


  #[test]
  fn returns_rejected_input() {
    let input = SimpleInput::default();
    let user_variable_validator = RejectedUserTemplateVariables;
    let config_validator = DefaultTemplateConfigValidator::with_all_dependencies(Box::new(input), Box::new(user_variable_validator));
    let template_variables = TemplateVariables::default();

    let user_config =
      UserConfig::new("template_dir", "target_idr");

    let validation_result = config_validator.validate(user_config, template_variables).expect("validation failed.");

    assert_eq!(validation_result, TemplateVariableReview::Rejected)
  }

  #[test]
  fn get_plugin_value_returns_none_when_plugin_has_not_run() {
    let plugin = Plugin {
        id: "MyPlugin".to_owned(),
        args: Default::default(),
        result: PluginRunStatus::default(),
      };

    let result = Cli::get_plugin_value(Some(&plugin));
    assert_eq!(result, None)
  }

  #[test]
  fn get_plugin_value_returns_run_result_when_plugin_has_run() {
    let plugin_result = PluginRunResult::new("my result");

    let plugin = Plugin {
        id: "MyPlugin".to_owned(),
        args: Default::default(),
        result: PluginRunStatus::Run(plugin_result.clone()),
      };

    let result = Cli::get_plugin_value(Some(&plugin));
    assert_eq!(result, Some(plugin_result))
  }

  #[test]
  fn get_default_value_returns_none_if_not_set() {
    let default_value = None;
    let result = Cli::get_default_value(default_value);

    assert_eq!(result, None)
  }

  #[test]
  fn get_default_value_returns_default_if_set() {
    let default_value = Some("my default");
    let result = Cli::get_default_value(default_value);

    assert_eq!(result, default_value.map(|dv| dv.to_owned()))
  }

  #[test]
  fn get_dynamic_values_returns_plugin_if_set() {
    let plugin_result_value = "my plugin result";
    let plugin_result = PluginRunResult::new(plugin_result_value);
    let result: DynamicValueType = Cli::get_dynamic_values(None, Some(plugin_result).as_ref());

    match result {
      r @ DynamicValueType::DefaultValue(..) => panic!("Expected PluginValue, got DefaultValue: {:?}", r),
      DynamicValueType::PluginValue(_, value) => assert_eq!(value, plugin_result_value),
      DynamicValueType::Neither => panic!("Expected PluginValue, got NeitherValue"),
    }
  }

  #[test]
  fn get_dynamic_values_returns_default_value_if_set() {
    let default_value = "my default value";
    let result: DynamicValueType = Cli::get_dynamic_values(Some(default_value), None);

    match result {
      DynamicValueType::DefaultValue(_, value) => assert_eq!(value, default_value),
      r @ DynamicValueType::PluginValue(..) => panic!("Expected DefaultValue, got PluginValue: {:?}", r),
      DynamicValueType::Neither => panic!("Expected PluginValue, got NeitherValue"),
    }
  }

  #[test]
  fn get_dynamic_values_returns_neither_if_no_values_are_set() {
    let result: DynamicValueType = Cli::get_dynamic_values(None, None);

    assert_eq!(result, DynamicValueType::Neither)
  }
}

#[test]
fn get_dynamic_values_returns_plugin_preferrentially() {
  let plugin_result_value = "my plugin result";
  let plugin_result = PluginRunResult::new(plugin_result_value);
  let default_value = "my default value";

  let result = Cli::get_dynamic_values(Some(default_value), Some(plugin_result).as_ref());

    match result {
      r @ DynamicValueType::DefaultValue(..) => panic!("Expected PluginValue, got DefaultValue: {:?}", r),
      DynamicValueType::PluginValue(_, value) => assert_eq!(value, plugin_result_value),
      DynamicValueType::Neither => panic!("Expected PluginValue, got NeitherValue"),
    }
}

