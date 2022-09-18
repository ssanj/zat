use crate::models::*;
use aho_corasick::AhoCorasick;
use walkdir::{WalkDir, DirEntry};
use std::fs::{self, File};
use std::{fs::create_dir, collections::HashMap, path::Path};

pub fn process_template(template_dir: &TemplateDir, target_dir: &TargetDir, token_map: HashMap<String, String>) -> ZatResult<()> {
  let ignored_files = [".variables.prompt"];
  let ignored_directories = [".git"];
  let target_files = get_files_to_process(&template_dir, &target_dir, &ignored_directories, &ignored_files)?;

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

  target_files
    .into_iter()
    .map(|file_type|{
      match file_type {
        FileTypes::File(source_file, target_file) => copy_file(replace_tokens, &source_file, &target_file),
        FileTypes::Dir(dir_path) => create_directory(replace_tokens, &dir_path),
      }
    })
    .collect::<ZatResult<Vec<()>>>()
    .map(|_| ())
}

// TODO: Use this in the above function
fn build_token_replacer(token_map: HashMap<String, String>) -> impl Fn(&str) -> String {
    // Grab the keys and values so the orders are consistent (HashMap has inconsistent ordering)
    let mut token_keys: Vec<String> = vec![];
    let mut token_values: Vec<String> = vec![];
    for (key, value) in token_map {
      token_keys.push(key); // key
      token_values.push(value); // value
    };

    let ac = AhoCorasick::new(token_keys);

    move |haystack: &str| {
      let result = &ac.replace_all(haystack, &token_values);
      result.to_owned()
    }
  }

fn get_files_to_process(template_dir: &TemplateDir, target_dir: &TargetDir, ignored_directories: &[&str], ignored_files: &[&str]) -> ZatResult<Vec<FileTypes>> {
  WalkDir::new(template_dir)
    .into_iter()
    .filter_map(|re| re.ok())
    .filter(|dir_entry| required_entries(dir_entry, ignored_directories, ignored_files))
    .map(|dir_entry| get_file_type(&dir_entry, &template_dir, &target_dir))
    .collect::<ZatResult<Vec<FileTypes>>>()
}

fn required_entries(dir_entry: &DirEntry, ignored_directories: &[&str], ignored_files: &[&str]) -> bool {
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
}

fn get_file_type(dir_entry: &DirEntry, template_dir: &TemplateDir, target_dir: &TargetDir) -> ZatResult<FileTypes> {
  let file_path = dir_entry.path().to_string_lossy();
  let source_file = SourceFile(file_path.to_string());

  file_path
    .strip_prefix(&template_dir.path)
    .ok_or_else(||{
      ZatError::IOError(format!("Could remove template prefix from directory: {}", file_path))
    })
    .and_then(|relative_target_path|{
      classify_file_types(dir_entry, relative_target_path, &source_file, target_dir)
    }).map(|(is_file_result, source_path, target_path)|{
      if is_file_result {
        FileTypes::File(source_path.clone(), target_path.clone())
      } else {
        FileTypes::Dir(target_path.0.clone())
      }
    })
}

fn classify_file_types<'a>(dir_entry: &'a DirEntry, relative_target_path: &str, file_path: &'a SourceFile, target_dir: &'a TargetDir) -> ZatResult<(bool, &'a SourceFile, TargetFile)> {
  let target_path = TargetFile(format!("{}{}", target_dir.path, relative_target_path));
  dir_entry
    .metadata()
    .map_err(|e|{
      ZatError::IOError(format!("Could not retrieve metadata for file: {}\nCause: {}", &file_path.0, e.to_string()))
    })
    .map(move |md| (md.is_file(), file_path, target_path))
}

fn copy_file<F>(replace_tokens: F, source_file: &SourceFile, target_file: &TargetFile) -> ZatResult<()> where
  F: Fn(&str) -> String
{
  let target_file_with_tokens_replaced = replace_tokens(&target_file.0);
  let content =
    fs::read(source_file.clone().0)
      .map_err(|e|{
        ZatError::IOError(format!("Could not read source file: {}\nCause: {}", &source_file.0, e.to_string()))
      })?;

  let target_file_path = Path::new(&target_file_with_tokens_replaced);
  println!("file: {} -> {}", &source_file.0, target_file_path.to_string_lossy());

  if let Some("tmpl") =
    target_file_path
      .extension()
      .map(|p| p.to_string_lossy().to_owned())
      .as_deref() { // It's a template
    write_template_file(replace_tokens, source_file.clone(), target_file.clone(), &target_file_path, &content)
  } else {
    write_file(&target_file_with_tokens_replaced, &content)
  }

  Ok(())
}

fn write_file(target_file_with_tokens_replaced: &str, content: &[u8]) {
  fs::write(target_file_with_tokens_replaced, content)
    .expect(&format!("Could not write target file: {}", &target_file_with_tokens_replaced))
}

fn write_template_file<F>(replace_tokens: F, source_file: SourceFile, target_file: TargetFile, target_file_path: &Path, content:  &[u8]) where
F: Fn(&str) -> String {
  let target_dir_path = Path::new(&target_file.0).parent().expect(&format!("Could not get parent path for: {}", &target_file.0));
  let str_content = std::str::from_utf8(&content).expect("Could not convert content to bytes to String");
  let content_with_tokens_replaced = replace_tokens(&str_content);
  let target_file_path_templated = target_file_path.file_stem().expect("Could not retrieve file name stem");
  let full_target_file_path_templated = target_dir_path.join(target_file_path_templated);
  let full_target_file_path_templated_str = full_target_file_path_templated.to_string_lossy();

  println!("writing file: {} -> {}", &source_file.0, &full_target_file_path_templated_str);

  fs::write(&*full_target_file_path_templated_str, content_with_tokens_replaced)
    .expect(&format!("Could not write target file: {}", &full_target_file_path_templated_str))
}

fn create_directory<F>(replace_tokens: F, directory_path: &str) -> ZatResult<()> where
  F: Fn(&str) -> String
{
  let directory_path_with_tokens_replaced = replace_tokens(directory_path);
  println!("dir: {} -> {}", directory_path, directory_path_with_tokens_replaced);
  create_dir(directory_path_with_tokens_replaced)
    .map_err(|e| {
      ZatError::IOError(
        format!("Could not created target directory: {}\nCause:{}",
          directory_path,
          e.to_string()
        ))
    })
}
