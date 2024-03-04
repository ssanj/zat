use std::{path::{PathBuf, Path}};

#[cfg(test)]
use tempfile::TempDir;

/// Creates the `child` folder under a temp directory and returns the
/// parent temp directory.
pub fn temp_dir_with(child: &str) -> TempDir {
  let temp_dir = TempDir::new().unwrap();
  let child_path = temp_dir.path().join(child);

  std::fs::create_dir(
    child_path.as_path())
      .unwrap_or_else(|_|
        panic!(
          "could not create child path: {} inside: {}",
          child_path.as_path().to_string_lossy(),
          temp_dir.path().to_string_lossy()));
  temp_dir
}

/// Creates the `child` folder under a temp directory and returns the
/// parent temp directory and the path to the child directory respectively as a pair.
pub fn temp_dir_with_parent_child_pair(child: &str) -> (PathBuf, PathBuf) {
  let temp_dir = TempDir::new().unwrap();
  let temp_dir_path = temp_dir.into_path();
  let child_path = temp_dir_path.as_path().join(child);

  std::fs::create_dir_all(
    child_path.as_path())
      .unwrap_or_else(|_|
        panic!(
          "could not create child path: {}",
          child_path.as_path().to_string_lossy()));

  println!("child_path: {:?} - {}", child_path.as_path(), child_path.as_path().exists());

  (temp_dir_path, child_path)
}

/// Creates file supplied under a temp directory and returns the
/// parent temp directory and the path to the file respectively as a pair.
#[allow(unused)]
pub fn temp_dir_with_file_pair(file: &str, content: &[u8], maybe_permissions: Option<u32>) -> (TempDir, PathBuf) {
  use std::os::unix::fs::OpenOptionsExt;
  let temp_dir = TempDir::new().unwrap();
  let file_path = temp_dir.path().join(file);

  // let f = File::create(&file_path).expect(&format!("Could not create file: {}", &file_path.to_string_lossy().to_string()));
  let mut file_options = std::fs::OpenOptions::new();
  file_options
    .create(true)
    .write(true);

  if let Some(permissions) = maybe_permissions {
    println!("setting permissions on {} to {:o}", &file_path.to_string_lossy().to_string(), permissions);
    file_options.mode(permissions);
  }

  file_options
    .open(&file_path)
    .unwrap_or_else(|_| panic!("Could not create file: {}", &file_path.to_string_lossy().to_string()));

  std::fs::write(
    file_path.as_path(),
    content
  )
  .unwrap_or_else(|_|
    panic!(
      "could not create file contents for: {} inside: {}",
      file_path.as_path().to_string_lossy(),
      temp_dir.path().to_string_lossy()));

  (temp_dir, Path::new(&file_path).to_owned())
}

/// Creates file supplied under the directory given returns the path to the file.
pub fn create_file_in(parent_dir: &Path, file: &str, content: &[u8], maybe_permissions: Option<u32>) -> PathBuf {
  use std::os::unix::fs::OpenOptionsExt;
  let file_path = parent_dir.join(file);

  // let f = File::create(&file_path).expect(&format!("Could not create file: {}", &file_path.to_string_lossy().to_string()));
  let mut file_options = std::fs::OpenOptions::new();
  file_options
    .create(true)
    .write(true);

  if let Some(permissions) = maybe_permissions {
    println!("setting permissions on {} to {:o}", &file_path.to_string_lossy().to_string(), permissions);
    file_options.mode(permissions);
  }

  let new_file = file_options
    .open(&file_path)
    .unwrap_or_else(|_| panic!("Could not create file: {}", &file_path.to_string_lossy()));

  std::fs::write(
    file_path.as_path(),
    content
  )
  .unwrap_or_else(|_|
    panic!(
      "could not create file contents for: {} inside: {}",
      file_path.as_path().to_string_lossy(),
      parent_dir.to_string_lossy()));

  drop(new_file); // This should close the file explicitly
  Path::new(&file_path).to_owned()
}
