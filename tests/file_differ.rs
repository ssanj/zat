use std::{path::Path, collections::HashSet, fmt, println};

use walkdir::{WalkDir, DirEntry};
use similar::{ChangeTag, TextDiff};
use ansi_term::Colour;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum FileType {
  File(String),
  Dir(String)
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let string = match self {
        FileType::File(file) => format!("File: {}", file),
        FileType::Dir(dir) => format!("Dir: {}", dir)
      };

      write!(f, "{}", string)
    }
}


struct Common(pub FileType);
struct OnlyInSource(pub FileType);
struct OnlyInDestination(pub FileType);


struct Changes {
  only_in_source: Vec<OnlyInSource>,
  only_in_destination: Vec<OnlyInDestination>,
  common: Vec<Common>,
}

// TODO: This definitely needs colour
pub fn print_changes<S: AsRef<Path>, D: AsRef<Path>>(expected_target_directory: S, target_directory: D) {
  let changes = diff(&expected_target_directory, &target_directory);

  if !changes.only_in_source.is_empty() {
    println!("Files only in expected render");
    for source in changes.only_in_source {
      println!("{}", source.0)
    }
    println!("");
  } else {
    println!("No changes in example render");
  }


  if !changes.only_in_destination.is_empty() {
    println!("Files only in actual render");
    for destination in changes.only_in_destination {
      println!("{}", destination.0)
    }
    println!("");
  } else {
    println!("No changes in actual render");
  }

  println!("");

  let files: Vec<_> =
    changes
      .common
      .iter()
      .filter_map(|c|{
        match &c.0 {
          FileType::Dir(_) => None,
          FileType::File(file) => Some(file)
        }
      })
      .collect();

  for file in files {
    let expected_file = expected_target_directory.as_ref().join(file);
    let actual_file = target_directory.as_ref().join(file);

    let expected_content = read_file(expected_file);
    let actual_content = read_file(actual_file);

    if expected_content != actual_content {
      println!("Changes found in: {}", Colour::Red.paint(file.as_str()).to_string());
      let text_diff = TextDiff::from_lines(&expected_content, &actual_content);
      for change in text_diff.iter_all_changes() {
          let sign = match change.tag() {
              ChangeTag::Delete => Colour::Red.paint("-").to_string(),
              ChangeTag::Insert => Colour::Green.paint("+").to_string(),
              ChangeTag::Equal => Colour::RGB(128, 128, 128).paint("|").to_string(),
          };
          print!("  {}{}", sign, change);
      }
    }
  }

  // We also need the files that are identical
}

pub fn print_diff(actual_content: &str, expected_content: &str) {
  if expected_content != actual_content {
    println!("Changes found.\nexpected content:\n{}\n\nactual content:\n{}", expected_content, actual_content);
    let text_diff = TextDiff::from_lines(expected_content, actual_content);
    for change in text_diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => Colour::Red.paint("-").to_string(),
            ChangeTag::Insert => Colour::Green.paint("+").to_string(),
            ChangeTag::Equal => Colour::RGB(128, 128, 128).paint("|").to_string(),
        };
        print!("  {}{}", sign, change);
    }
  }
}

fn read_file(file: std::path::PathBuf) -> String {
    std::fs::read_to_string(&file).expect(&format!("could not read file: {}", file.to_string_lossy().to_string()))
}

fn diff<S: AsRef<Path>, D: AsRef<Path>>(source_dir: S, destination_dir: D) -> Changes {
  let source_files: HashSet<FileType> =
    WalkDir::new(&source_dir)
      .into_iter()
      .filter_map(|re| re.ok())
      .filter_map(|dir_entry|{
        let p = dir_entry.path().strip_prefix(source_dir.as_ref()).unwrap();
        categorise_files(&dir_entry, p)
      })
      .collect();

  let destination_files: HashSet<FileType> =
    WalkDir::new(&destination_dir)
      .into_iter()
      .filter_map(|re| re.ok())
      .filter_map(|dir_entry|{
        let p = dir_entry.path().strip_prefix(destination_dir.as_ref()).unwrap();
        categorise_files(&dir_entry, p)
      })
      .collect();

  let only_in_source: Vec<OnlyInSource> =
    source_files
      .difference(&destination_files)
      .map(|so| OnlyInSource(so.clone()))
      .collect();

  let only_in_destination: Vec<OnlyInDestination> =
    destination_files
      .difference(&source_files)
      .map(|df| OnlyInDestination(df.clone()))
      .collect();

  let common: Vec<Common> =
    source_files
      .intersection(&destination_files)
      .map(|co| Common(co.clone()))
      .collect();

  Changes {
    only_in_source,
    only_in_destination,
    common
  }
}

fn categorise_files(dir_entry: &DirEntry, path: &Path) -> Option<FileType> {
  let string_path = path.to_string_lossy().to_string();
  let entry_file_type = dir_entry.file_type();
    if entry_file_type.is_file() {
      Some(FileType::File(string_path))
    } else if entry_file_type.is_dir() && !string_path.is_empty() { // we want to exclude the top level directory, that matches prefix
      Some(FileType::Dir(string_path))
    } else {
      None
    }
}

