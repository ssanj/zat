use std::fs::create_dir;

use walkdir::WalkDir;
use std::fmt;

#[derive(Debug, Clone)]
enum FileTypes {
  File(String),
  Dir(String),
}

impl fmt::Display for FileTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let path = match self {
        FileTypes::File(p) => p,
        FileTypes::Dir(p) => p
      };

      write!(f, "{}", path)
    }
}


fn main() {
  let template_dir =  "/Users/sanj/ziptemp/st-template";
  let target_dir =  "/Users/sanj/ziptemp/template-expansion";

  let _: () = create_dir(target_dir).expect("Could not created target dir");

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
            .expect(&format!("could not retrieve metadata for file: {}", file_path))
            .is_file() {
              FileTypes::File(target_path)
        } else {
          FileTypes::Dir(target_path)
        }

      });

  for target_file in target_files_it {
      // remove template dir from path
      println!("{}", target_file);
      // create_dir(format!("{}/{}", target_dir))
  }
}
