pub type ZatResultX<A> = Result<A, ZatErrorX>;
pub type ZatActionX = Result<(), ZatErrorX>;


#[derive(Debug, PartialEq)]
pub enum ZatErrorX {
  UserConfigError(String),
  VariableOpenError(String),
  VariableReadError(String),
  VariableDecodeError(String),
  ReadingFileError(String),
  WritingFileError(String),
  ProcessingErrors(Vec<ZatErrorX>),
}

// TODO: Clean up this output
impl std::fmt::Display for ZatErrorX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let string_rep = match self {
        ZatErrorX::UserConfigError(error)     => format!("Got a configuration error:\n    {}", error),
        ZatErrorX::VariableOpenError(error)   => format!("Got an error opening variable file:\n    {}", error),
        ZatErrorX::VariableReadError(error)   => format!("Got an error reading variable file:\n    {}", error),
        ZatErrorX::VariableDecodeError(error) => format!("Got an error decoding variable file:\n    {}", error),
        ZatErrorX::ReadingFileError(error)    => format!("Could not read template file:\n    {}", error),
        ZatErrorX::WritingFileError(error)    => format!("Could not write destination file:\n    {}", error),
        ZatErrorX::ProcessingErrors(errors)   => {
          let error_str =
            errors
              .into_iter()
              .map(|e| format!("{}", e))
              .collect::<Vec<_>>()
              .join("\n    - ");

          format!("Got an error processing template files: {}", error_str)
        },
      };

      write!(f, "{}", string_rep)
    }
}
