use std::path::PathBuf;
use std::{fmt, path::Path, ffi::OsStr, borrow::Cow};
use std::fs;

pub type ZatResult<A> = Result<A, ZatError>;
pub type ZatAction = Result<(), ZatError>;

#[derive(Debug, Clone)]
pub struct SourceFile(pub String);

impl SourceFile {

  pub fn read(&self) -> Result<String, ZatError> {
    fs::read(&self.0)
      .map_err(|e|{
        ZatError::IOError(format!("Could not read source file: {}\nCause: {}", self.0.as_str(), e.to_string()))
      })
      .and_then(|content| {
        std::str::from_utf8(&content)
          .map_err(|e| {
            ZatError::IOError(
              format!("Could not convert content of {} from bytes to String:\n{}",
                &self.0,
                e.to_string())
              )
          })
          .map(|c| c.to_owned())
      })
  }

  pub fn strip_prefix(&self, prefix: &str)  -> ZatResult<String> {
    (&self.0).strip_prefix(prefix)
    .ok_or_else(||{
      ZatError::IOError(format!("Could remove path prefix: {} from directory: {}", prefix, &self.0))
    })
    .map(|p| p.to_owned())
  }
}

impl fmt::Display for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Clone)]
pub struct TargetFile(pub String);

impl TargetFile {

  pub fn get_extension(&self) -> Option<Cow<'_, str>> {
     Path::new(&self.0)
      .extension()
      .map(|p| p.to_string_lossy().to_owned())
  }

  pub fn remove_extension(&self) -> TargetFile {
    let without_extension =
      Path::new(&self.0)
        .file_stem()
        .expect(&format!("Could not retrieve file name stem for: {}", &self.0))
        .to_string_lossy();

    TargetFile(without_extension.to_string())
  }

  pub fn parent_directory(&self) -> TargetFile {
    let parent_dir =
      Path::new(&self.0)
        .parent()
        .expect(&format!("Could not get parent path for: {}", &self.0))
        .to_string_lossy();

    TargetFile(parent_dir.to_string())
  }

  pub fn file_stem(&self) -> TargetFile {
    let file_stem =
      Path::new(&self.0)
        .file_name()
        .expect(&format!("Could not get file stem for: {}", &self.0))
        .to_string_lossy();

    TargetFile(file_stem.to_string())
  }

  pub fn join<P>(&self, other: P) -> TargetFile where
    P: AsRef<Path>
  {
    TargetFile(Path::new(&self.0).join(other).to_string_lossy().to_string())
  }

  pub fn map<F>(&self, f: F) -> TargetFile where
    F: Fn(&str) -> String
  {
    TargetFile(f(&self.0))
  }
}

impl From<PathBuf> for TargetFile {
  fn from(p: PathBuf) -> Self {
      TargetFile(p.to_string_lossy().to_string())
  }
}


impl AsRef<Path> for TargetFile {
  fn as_ref(&self) -> &Path {
      &Path::new(&self.0)
  }
}


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

  pub fn join<P>(&self, other: P) -> PathBuf where
    P: AsRef<Path>
  {
    Path::new(&self.path).join(other)
  }

  pub fn does_exist(&self) -> bool {
    Path::new(&self.path).exists()
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


#[derive(Debug, Clone, PartialEq)]
pub struct TemplateDir {
  pub path: String
}

impl TemplateDir {
  pub fn new(path: &str) -> Self {
    TemplateDir {
      path: path.to_owned(),
    }
  }

  pub fn join<P>(&self, other: P) -> PathBuf where
    P: AsRef<Path>
  {
    Path::new(&self.path).join(other)
  }


  pub fn does_exist(&self) -> bool {
    Path::new(&self.path).exists()
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


impl fmt::Display for TargetFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Clone)]
pub enum FileTypes {
  File(SourceFile, TargetFile),
  Dir(String),
  Symlink(String),
}

impl fmt::Display for FileTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let path = match self {
        FileTypes::File(SourceFile(src), TargetFile(tgt)) => format!("FileTypes::File({}, {})", src, tgt),
        FileTypes::Dir(p) => format!("FileTypes::Dir({})", p),
        FileTypes::Symlink(p) => format!("FileTypes::Symlink({})", p),
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

#[derive(Debug, Clone)]
pub enum TraversalFileType {
  File,
  Dir,
  Symlink,
}

#[derive(Debug, Clone)]
pub struct DirTraversalEntry {
  path: PathBuf,
  file_type: TraversalFileType,
}

impl DirTraversalEntry {

  pub fn new<T>(path: T, file_type: TraversalFileType) -> Self where
    T: AsRef<Path>
  {
    DirTraversalEntry {
      path: path.as_ref().to_path_buf(),
      file_type
    }
  }

  pub fn path_as_string(&self) -> String {
    self.path.to_string_lossy().to_string()
  }

  pub fn file_type(&self) -> TraversalFileType {
    self.file_type.clone()
  }
}

pub trait FileWriter {
  fn create_directory_x<P>(path: P) -> ZatResult<()> where
    P: AsRef<Path>;

  fn copy_file_x<P, C>(path: P, content: C) -> ZatResult<()> where
    P: AsRef<Path>,
    C: AsRef<[u8]>;
}

struct RealFileSystem;

impl FileWriter for RealFileSystem {
  fn create_directory_x<P>(path: P) -> ZatResult<()> where
    P: AsRef<Path> {
      fs::create_dir(path.as_ref())
        .map_err(|e| {
          ZatError::IOError(
            format!("Could not created target directory: {}\nCause:{}",
              path.as_ref().display(),
              e.to_string()
          ))
        })
      }

  fn copy_file_x<P, C>(path: P, content: C) -> ZatResult<()> where
    P: AsRef<Path>,
    C: AsRef<[u8]> {
      fs::write(&path, content)
        .map_err(|e| {
          ZatError::IOError(format!("Could not write target file: {}\nCause:{}", &path.as_ref().display(), e))
        })
  }
}

// TODO: Thoughts
// Using a single models.rs lacks modularity. We should have a models.rs for cli, templates and tokens
