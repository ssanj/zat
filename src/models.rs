use std::{fmt, path::Path, ffi::OsStr};

pub type ZatResult<A> = Result<A, ZatError>;

#[derive(Debug, Clone)]
pub struct SourceFile(pub String);

#[derive(Debug, Clone)]
pub struct TargetFile(pub String);


#[derive(Debug, Clone)]
pub struct TargetDir {
  pub path: String
}

impl TargetDir {
  pub fn new(path: &str) -> Self {
    TargetDir {
      path: path.to_owned()
    }
  }
}

impl AsRef<Path> for TargetDir {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.path)
  }
}

impl AsRef<OsStr> for TargetDir {
  fn as_ref(&self) -> &OsStr {
    self.path.as_ref()
  }
}


#[derive(Debug, Clone)]
pub struct TemplateDir {
  pub path: String
}

impl TemplateDir {
  pub fn new(path: &str) -> Self {
    TemplateDir {
      path: path.to_owned(),
    }
  }
}

impl AsRef<Path> for TemplateDir {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.path)
  }
}

impl AsRef<OsStr> for TemplateDir {
  fn as_ref(&self) -> &OsStr {
    self.path.as_ref()
  }
}

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

#[derive(Debug)]
pub enum ZatError {
  SerdeError(String),
  IOError(String),
  OtherError(String)
}

impl ZatError {
  pub fn inner_error(&self) -> &str {
    match self {
      ZatError::SerdeError(e) => &e,
      ZatError::IOError(e)    => &e,
      ZatError::OtherError(e) => &e,
    }
  }
}
