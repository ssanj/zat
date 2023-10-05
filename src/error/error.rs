pub type ZatResult<A> = Result<A, ZatError>;
pub type ZatAction = Result<(), ZatError>;


#[derive(Debug, PartialEq)]
pub enum ZatError {
  UserConfigError(String),
  VariableFileNotFound(String),
  VariableOpenError(String),
  VariableReadError(String),
  VariableDecodeError(String),
  ReadingFileError(String),
  WritingFileError(String),
  ProcessingErrors(Vec<ZatError>),
}

impl std::fmt::Display for ZatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let string_rep = match self {
        ZatError::UserConfigError(error)        => format!("Got a configuration error:\n    {}", error),
        ZatError::VariableFileNotFound(error)   => format!("Could not find variables file:\n    {}", error),
        ZatError::VariableOpenError(error)      => format!("Got an error opening variable file:\n    {}", error),
        ZatError::VariableReadError(error)      => format!("Got an error reading variable file:\n    {}", error),
        ZatError::VariableDecodeError(error)    => format!("Got an error decoding variable file:\n    {}", error),
        ZatError::ReadingFileError(error)       => format!("Could not read template file:\n    {}", error),
        ZatError::WritingFileError(error)       => format!("Could not write destination file:\n    {}", error),
        ZatError::ProcessingErrors(errors)      => {
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
