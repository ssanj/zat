use std::fs::create_dir;

use walkdir::WalkDir;
use std::fs;
use crate::models::*;

mod models;

fn main() {
  let template_dir =  "/Users/sanj/ziptemp/st-template";
  let target_dir =  "/Users/sanj/ziptemp/template-expansion";

  let target_files_it =
    WalkDir::new(template_dir)
      .into_iter()
      .filter_map(|re| re.ok())
      .filter(|dir_entry| {
        let file_path = dir_entry.path().to_string_lossy();
        let is_git = file_path.contains(".git");
        let is_git_ignore = file_path.contains(".gitignore");
        !(is_git && !is_git_ignore)
      })
      .map(|dir_entry|{
        let file_path = dir_entry.path().to_string_lossy();
        let relative_target_path = file_path.strip_prefix(template_dir).expect(&format!("Could remove template prefix from directory: {}", file_path));
        let target_path = format!("{}{}", target_dir, relative_target_path);

        if dir_entry
            .metadata()
            .expect(&format!("Could not retrieve metadata for file: {}", file_path))
            .is_file() {
              FileTypes::File(SourceFile(file_path.to_string()), TargetFile(target_path))
        } else {
          FileTypes::Dir(target_path)
        }
      });

  for target_file in target_files_it {
    println!("{}", target_file);
    match target_file {
      FileTypes::File(source_file, target_file) => copy_file(source_file, target_file),
      FileTypes::Dir(dir_path) => create_directory(&dir_path),
    }
  }

  fn copy_file(source_file: SourceFile, target_file: TargetFile) {
    println!("copying file: {} -> {}", source_file, target_file);
    let content = fs::read(source_file.clone().0).expect(&format!("Could not read source file: {}", source_file.0));
    fs::write(target_file.clone().0, content).expect(&format!("Could not write target file: {}", target_file.0));
  }

  fn create_directory(directory_path: &str) {
    create_dir(directory_path).expect(&format!("Could not created target dir: {}", directory_path));
  }
}
