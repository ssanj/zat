use std::io::{stdin, BufRead};
use std::{fs::create_dir, collections::HashMap, path::Path};

use walkdir::WalkDir;
use std::fs::{self, File};
use std::io::Read;
use crate::models::*;
use crate::variables::*;
use aho_corasick::AhoCorasick;

mod models;
mod variables;
mod tokens;

fn main() {
  let template_dir =  TemplateDir::new("/Users/sanj/ziptemp/st-template");
  let target_dir =  TargetDir::new("/Users/sanj/ziptemp/template-expansion");

  let variables_file = Path::new(&template_dir.path).join(".variables.prompt");

  let user_tokens_supplied =
    if variables_file.exists() {
      println!("Loading variables file");
      let mut f = File::open(variables_file).unwrap();
      let mut variables_json = String::new();

      f.read_to_string(&mut variables_json).unwrap();

      let variables: Vec<TemplateVariable> = serde_json::from_str(&variables_json).unwrap();
      let stdin = std::io::stdin();

      let mut token_map = HashMap::new();

      println!("loaded: {:?}", &variables);

      for v in &variables {
        println!("{}:", v.prompt);
        let mut variable_value = String::new();
        if let Ok(read_count) = stdin.read_line(&mut variable_value) {
          if read_count > 0 {
            let _ = variable_value.pop();
          }

          token_map.insert(v.variable_name.clone(), variable_value);
          println!("filters: {:?}", v.filters);
        }
      }

      println!("tokens: {:?}", token_map);

      let updated_token_map = tokens::expand_filters(&variables, &token_map);

      println!("updated tokens: {:?}", updated_token_map);

      let updated_token_map_dollar_keys: HashMap<_, _> =
        updated_token_map
          .into_iter()
          .map(|(k, v)| (format!("${}$", k), v))
          .collect();

      println!("updated tokens dollar keys: {:?}", &updated_token_map_dollar_keys);

      updated_token_map_dollar_keys
    } else {
      println!("No variables file");
      HashMap::new()
    };

    println!("done")

  // process_template(&template_dir, &target_dir)
}

fn process_template(template_dir: &TemplateDir, target_dir: &TargetDir) {

  let target_files_it = get_files_to_process(&template_dir, &target_dir);

  // TODO: Create this from user settings
  let token_map =
    HashMap::from([
        ("$project$", "MyProjectName")
      ]);

  // Fix the ordering of these so they match
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
