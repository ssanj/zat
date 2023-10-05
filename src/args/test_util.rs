use std::path::PathBuf;

#[cfg(test)]
use tempfile::TempDir;

/// Creates the `child` folder under a temp directory and returns the
/// parent temp directory.
pub fn temp_dir_with(child: &str) -> TempDir {
  let temp_dir = TempDir::new().unwrap();
  let child_path = temp_dir.path().join(child);

  std::fs::create_dir(
    child_path.as_path())
      .expect(
        &format!(
          "could not create child path: {} inside: {}",
          child_path.as_path().to_string_lossy().to_string(),
          temp_dir.path().to_string_lossy().to_string()));
  temp_dir
}

/// Creates the `child` folder under a temp directory and returns the
/// parent temp directory and the path to the child directory respectively as a pair.
pub fn temp_dir_with_parent_child_pair(child: &str) -> (TempDir, PathBuf) {
  let temp_dir = TempDir::new().unwrap();
  let child_path = temp_dir.path().join(child);

  std::fs::create_dir(
    child_path.as_path())
      .expect(
        &format!(
          "could not create child path: {} inside: {}",
          child_path.as_path().to_string_lossy().to_string(),
          temp_dir.path().to_string_lossy().to_string()));

  (temp_dir, child_path)
}

