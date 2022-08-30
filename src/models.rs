use std::fmt;

#[derive(Debug, Clone)]
pub struct SourceFile(pub String);

#[derive(Debug, Clone)]
pub struct TargetFile(pub String);

impl fmt::Display for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TargetFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Clone)]
pub enum FileTypes {
  File(SourceFile, TargetFile),
  Dir(String),
}

impl fmt::Display for FileTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let path = match self {
        FileTypes::File(SourceFile(src), TargetFile(tgt)) => format!("FileTypes::File({}, {})", src, tgt),
        FileTypes::Dir(p) => format!("FileTypes::Dir({})", p),
      };

      write!(f, "{}", path)
    }
}
