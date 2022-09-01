use std::{fs::create_dir, collections::HashMap};

use walkdir::WalkDir;
use std::fs;
use crate::models::*;
use aho_corasick::AhoCorasick;

mod models;

fn main() {
  let template_dir =  TemplateDir::new("/Users/sanj/ziptemp/st-template");
  let target_dir =  TargetDir::new("/Users/sanj/ziptemp/template-expansion");

  let target_files_it = get_files_to_process(&template_dir, &target_dir);

  // TODO: Create this from user settings
  let token_map =
    HashMap::from([
        ("$project$", "MyProjectName")
      ]);

  let token_keys: Vec<&&str> = token_map.keys().collect();
  let token_values: Vec<&&str> = token_map.values().collect();
  let ac = AhoCorasick::new(token_keys);

  for target_file in target_files_it {
    match target_file {
      FileTypes::File(source_file, target_file) => copy_file(&ac, &token_values, source_file, target_file),
      FileTypes::Dir(dir_path) => create_directory(&ac, &token_values, &dir_path),
    }
  }

  fn get_files_to_process(template_dir: &TemplateDir, target_dir: &TargetDir) -> Vec<FileTypes> {
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
        let relative_target_path = file_path.strip_prefix(&template_dir.path).expect(&format!("Could remove template prefix from directory: {}", file_path));
        let target_path = format!("{}{}", target_dir.path, relative_target_path);

        if dir_entry
            .metadata()
            .expect(&format!("Could not retrieve metadata for file: {}", file_path))
            .is_file() {
              FileTypes::File(SourceFile(file_path.to_string()), TargetFile(target_path))
        } else {
          FileTypes::Dir(target_path)
        }
      })
      .collect()
  }

  fn copy_file(token_map: &AhoCorasick, token_values: &[&&str], source_file: SourceFile, target_file: TargetFile) {
    let target_file_with_tokens_replaced = token_map.replace_all(&target_file.0, token_values);

    let content = fs::read(source_file.clone().0).expect(&format!("Could not read source file: {}", source_file.0));
    fs::write(&target_file_with_tokens_replaced, content).expect(&format!("Could not write target file: {}", &target_file_with_tokens_replaced));
  }

  fn create_directory(token_map: &AhoCorasick, token_values: &[&&str], directory_path: &str) {
    let directory_path_with_tokens_replaced = token_map.replace_all(directory_path, token_values);
    println!("dir: {} -> {}", directory_path, directory_path_with_tokens_replaced);
    create_dir(directory_path_with_tokens_replaced).expect(&format!("Could not created target dir: {}", directory_path));
  }
}
