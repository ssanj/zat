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

  let replace_tokens = |haystack: &str| {
    let result = &ac.replace_all(haystack, &token_values);
    result.to_owned()
  };

  for target_file in target_files_it {
    match target_file {
      FileTypes::File(source_file, target_file) => copy_file(replace_tokens, source_file, target_file),
      FileTypes::Dir(dir_path) => create_directory(replace_tokens, &dir_path),
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

  fn copy_file<F>(replace_tokens: F, source_file: SourceFile, target_file: TargetFile) where
    F: Fn(&str) -> String
  {
    let target_file_with_tokens_replaced = replace_tokens(&target_file.0);

    let content = fs::read(source_file.clone().0).expect(&format!("Could not read source file: {}", source_file.0));
    fs::write(&target_file_with_tokens_replaced, content).expect(&format!("Could not write target file: {}", &target_file_with_tokens_replaced));
  }

  fn create_directory<F>(replace_tokens: F, directory_path: &str) where
    F: Fn(&str) -> String
  {
    let directory_path_with_tokens_replaced = replace_tokens(directory_path);
    println!("dir: {} -> {}", directory_path, directory_path_with_tokens_replaced);
    create_dir(directory_path_with_tokens_replaced).expect(&format!("Could not created target dir: {}", directory_path));
  }
}
