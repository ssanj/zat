use std::ffi::OsStr;
use std::io::{stdin, BufRead};
use std::{fs::create_dir, collections::HashMap, path::Path};

use walkdir::WalkDir;
use std::fs::{self, File};
use std::io::Read;
use crate::models::*;
use crate::variables::*;
use crate::cli::Args;
use aho_corasick::AhoCorasick;


mod models;
mod variables;
mod tokens;
mod cli;

fn main() {

  let cli_args = cli::get_cli_args();

  let template_dir = TemplateDir::new(&cli_args.template);
  let target_dir = TargetDir::new(&cli_args.destination);

  let template_path_exists = does_path_exist(&template_dir);
  let target_path_exists = does_path_exist(&target_dir);

  if template_path_exists && !target_path_exists {
    let variables_file = Path::new(&template_dir.path).join(".variables.prompt");

    // TODO: We need a way to confirm variable values here
    // If they are wrong allow re-entry or exit
    let user_tokens_supplied = tokens::load_variables(&variables_file);
    // fs::create_dir_all(&target_dir.path).expect("Could not create target directory");
    process_template(&template_dir, &target_dir, user_tokens_supplied)
  } else if !template_path_exists {
    println!("Template path does not exist: {}", &template_dir.path)
  } else {
    println!("Target path already exists: {}. Please supply an empty directory for the target", &target_dir.path)
  }
}

fn does_path_exist<A>(path: A) -> bool where
  A: AsRef<OsStr>
{
  Path::new(&path).exists()
}


fn process_template(template_dir: &TemplateDir, target_dir: &TargetDir, token_map: HashMap<String, String>) {
  let ignored_files = [".variables.prompt"];
  let ignored_directories = [".git"];
  let target_files_it = get_files_to_process(&template_dir, &target_dir, &ignored_directories, &ignored_files);

  // Grab the keys and values so the orders are consistent (HashMap has inconsistent ordering)
  let mut token_keys: Vec<String> = vec![];
  let mut token_values: Vec<String> = vec![];
  for (key, value) in token_map {
    token_keys.push(key); // key
    token_values.push(value); // value
  };

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
}

fn get_files_to_process(template_dir: &TemplateDir, target_dir: &TargetDir, ignored_directories: &[&str], ignored_files: &[&str]) -> Vec<FileTypes> {
  WalkDir::new(template_dir)
    .into_iter()
    .filter_map(|re| re.ok())
    .filter(|dir_entry| {
      let file_path = dir_entry.path().to_string_lossy();
      let file_type = dir_entry.file_type();
      let is_ignored =
        if file_type.is_file() {
          let result = ignored_files.iter().any(|f| file_path.ends_with(f)) || ignored_directories.iter().any(|d| file_path.contains(d));
          println!("file: {}, ignored:{}", file_path, result);
          result
        } else if file_type.is_dir() {
          let result = ignored_directories.iter().any(|d| file_path.contains(d));
          println!("dir: {}, ignored:{}", file_path, result);
          result
        } else {
          println!("*******: {}, ignored: false", file_path);
          false
        };

      !is_ignored
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

  let target_file_path = Path::new(&target_file_with_tokens_replaced);
  println!("file: {} -> {}", &source_file.0, target_file_path.to_string_lossy());

  if let Some("tmpl") = target_file_path.extension().map(|p| p.to_string_lossy().to_owned()).as_deref() { // It's a template
    println!("file tmp");
    let target_dir_path = Path::new(&target_file.0).parent().expect(&format!("Could not get parent path for: {}", &target_file.0));
    let str_content = std::str::from_utf8(&content).expect("Could not convert content to bytes to String");
    let content_with_tokens_replaced = replace_tokens(&str_content);
    let target_file_path_templated = target_file_path.file_stem().expect("Could not retrieve file name stem");
    let full_target_file_path_templated = target_dir_path.join(target_file_path_templated);
    let full_target_file_path_templated_str = full_target_file_path_templated.to_string_lossy();
    println!("writing file: {} -> {}", &source_file.0, &full_target_file_path_templated_str);
    fs::write(&*full_target_file_path_templated_str, content_with_tokens_replaced).expect(&format!("Could not write target file: {}", &full_target_file_path_templated_str));
  } else {
    fs::write(&target_file_with_tokens_replaced, content).expect(&format!("Could not write target file: {}", &target_file_with_tokens_replaced));
  }
}

fn create_directory<F>(replace_tokens: F, directory_path: &str) where
  F: Fn(&str) -> String
{
  let directory_path_with_tokens_replaced = replace_tokens(directory_path);
  println!("dir: {} -> {}", directory_path, directory_path_with_tokens_replaced);
  create_dir(directory_path_with_tokens_replaced).expect(&format!("Could not created target dir: {}", directory_path));
}
