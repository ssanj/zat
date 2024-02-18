use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ArgType {
  MutlipleArgs(Vec<PluginArg>),
  ArgLine(Vec<String>),
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Plugin {
  pub id: String,
  pub args: Option<ArgType>,

  #[serde(default)]
  pub result: PluginRunStatus,
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum PluginRunStatus {
  NotRun,
  Run(PluginRunResult)
}


impl Default for PluginRunStatus {
  fn default() -> Self {
      Self::NotRun
  }
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PluginRunResult {
  pub result: String,
}


impl PluginRunResult {
  pub fn new(result: &str) -> Self {
    Self {
      result: result.to_owned()
    }
  }
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PluginArg {
  pub name: String,
  pub value: String,
  pub prefix: String
}

impl PluginArg {

  #[cfg(test)]
  pub fn new(name: &str, value: &str) -> Self {
    let prefix =
      if name.len() > 1 {
        "--"
      } else {
        "-"
      };

    Self {
      name: name.to_owned(),
      value: value.to_owned(),
      prefix: prefix.to_owned()
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn decodes_plugin_with_args() {
    let config = r#"
      {
        "id": "scala-deps",
        "args":[
          {
            "name": "o",
            "value": "org.scala-lang",
            "prefix": "-"
          },
          {
            "name": "g",
            "value": "scala3-library",
            "prefix": "-"
          },
          {
            "name": "s",
            "value": "3",
            "prefix": "-"
          }
        ]
      }
    "#;

    let plugin: Plugin =
      serde_json::from_str(config)
        .map_err(|e| e.to_string())
        .unwrap();

    let arg1 = plugin_arg("o", "org.scala-lang", "-");
    let arg2 = plugin_arg("g", "scala3-library", "-");
    let arg3 = plugin_arg("s", "3", "-");

    let args = ArgType::MutlipleArgs(vec![arg1, arg2, arg3]);

    let expected_plugin =
      Plugin {
        id: "scala-deps".to_owned(),
        args: Some(args),
        result: PluginRunStatus::NotRun
    };

    assert_eq!(plugin, expected_plugin);
  }


  #[test]
  fn decodes_plugin_with_argline() {
    let config = r#"
      {
        "id": "scala-deps",
        "args":[
          "-o org.scala-lang",
          "-g scala3-library",
          "-s 3"
        ]
      }
    "#;

    let plugin: Plugin =
      serde_json::from_str(config)
        .map_err(|e| e.to_string())
        .unwrap();


    let args =
      vec![
      "-o org.scala-lang".to_owned(),
        "-g scala3-library".to_owned(),
        "-s 3".to_owned()
      ];

    let args = ArgType::ArgLine(args);

    let expected_plugin =
      Plugin {
        id: "scala-deps".to_owned(),
        args: Some(args),
        result: PluginRunStatus::NotRun
    };

    assert_eq!(plugin, expected_plugin);
  }


  #[test]
  fn decodes_plugin_missing_args() {
    let config = r#"
      {
        "id": "scala-deps"
      }
    "#;

    let plugin_result =
      serde_json::from_str::<Plugin>(config)
        .map_err(|e| e.to_string())
        .unwrap();

    let expected_plugin =
      Plugin {
        id: "scala-deps".to_owned(),
        args: None,
        result: PluginRunStatus::NotRun
    };

    assert_eq!(plugin_result, expected_plugin)
  }


  #[test]
  fn decoding_plugin_fails_with_invalid_args() {
    let config = r#"
      {
        "id": "scala-deps",
        args: 1234
      }
    "#;

    let plugin_result =
      serde_json::from_str::<Plugin>(config)
        .map_err(|e| e.to_string());

    assert!(plugin_result.is_err())
  }


  fn plugin_arg(name: &str, value: &str, prefix: &str) -> PluginArg {
    PluginArg {
      name: name.to_owned(),
      value: value.to_owned(),
      prefix: prefix.to_owned()
    }
  }

}

