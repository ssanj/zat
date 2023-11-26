use std::path::Path;
use std::println;
use super::destination_file::DestinationFile;
use super::file_traverser::TemplateFile;
use crate::error::ZatResult;
use super::source_file::SourceFile;
use crate::config::{UserConfig, TemplateFilesDir};
use super::template_enricher::TemplateEnricher;
use super::enriched_template_file_processor::EnrichedTemplateFile;

pub struct DefaultTemplateEnricher {
  config: UserConfig
}

impl DefaultTemplateEnricher {
  pub fn new(config: UserConfig) -> Self {
    Self {
      config
    }
  }

  fn get_destination_file<D>(source_file: &SourceFile, source_root_path: &TemplateFilesDir, destination_root_path: D) -> ZatResult<DestinationFile>
    where D: AsRef<Path>
  {
    source_file
      .strip_prefix(source_root_path.as_ref())
      .map(|relative_source_path|{
        let destination_file_path = destination_root_path.as_ref().join(&relative_source_path);
        DestinationFile::new(&destination_file_path.to_string_lossy().to_string())
      })
  }
}

impl TemplateEnricher for DefaultTemplateEnricher {
  fn enrich(&self, template_file: TemplateFile) ->  ZatResult<EnrichedTemplateFile>  {

    let template_files_dir_path = &self.config.template_files_dir;
    let destination_dir_path = &self.config.target_dir;

    if self.config.verbose {
      println!("Enriching Template file: {:?}", &template_file);
    }

    match template_file {
      TemplateFile::File(file) => {
        let source_file = SourceFile(file);
        let destination_file = Self::get_destination_file(&source_file, &template_files_dir_path, &destination_dir_path)?;
        Ok(EnrichedTemplateFile::File(source_file, destination_file))
      },
      TemplateFile::Dir(dir) => {
        let source_file = SourceFile(dir);
        let destination_file = Self::get_destination_file(&source_file, &template_files_dir_path, &destination_dir_path)?;
        Ok(EnrichedTemplateFile::Dir(destination_file))
      }
    }
  }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::super::source_file::SourceFile;
    use super::super::destination_file::DestinationFile;

    use super::*;
    use crate::args::test_util::temp_dir_with_parent_child_pair;
    use crate::config::TEMPLATE_FILES_DIR;

    #[test]
    fn enriches_files() {
      let (template_directory, template_files_directory) = temp_dir_with_parent_child_pair(TEMPLATE_FILES_DIR);
      let relative_template_path = "relative/source/file.ext";
      let destination_dir = "/some/destination/path";
      let template_file_path = template_files_directory.join(&relative_template_path);

      let file_template = TemplateFile::new_file(&template_file_path.to_str().unwrap());

      let config: UserConfig =
        UserConfig::new(&template_directory.as_path().to_str().unwrap(), &destination_dir);


      let enricher = DefaultTemplateEnricher::new(config);
      let enriched_file = enricher.enrich(file_template).expect(&format!("Could not enrich file: {}", &template_file_path.to_string_lossy().to_string()));

      let expected_destination_path = "/some/destination/path/relative/source/file.ext";
      let expected_enriched_file =
        EnrichedTemplateFile::File(
          SourceFile::new(&template_file_path.to_str().unwrap()),
          DestinationFile::new(&expected_destination_path));

      assert_eq!(enriched_file, expected_enriched_file)
    }


    #[test]
    fn enriches_directories() {
      let (template_directory, template_files_directory) = temp_dir_with_parent_child_pair(TEMPLATE_FILES_DIR);
      let relative_template_path = "relative/source/path";
      let destination_dir = "/some/destination/path";
      let template_file_path = template_files_directory.join(&relative_template_path);

      let file_template = TemplateFile::new_dir(&template_file_path.to_str().unwrap());

      let config: UserConfig =
        UserConfig::new(&template_directory.as_path().to_str().unwrap(), &destination_dir);


      let enricher = DefaultTemplateEnricher::new(config);
      let enriched_dir = enricher.enrich(file_template).expect(&format!("Could not enrich file: {}", &template_file_path.to_string_lossy().to_string()));

      let expected_destination_path = "/some/destination/path/relative/source/path";
      let expected_enriched_dir =
        EnrichedTemplateFile::Dir(
          DestinationFile::new(&expected_destination_path)
        );

      assert_eq!(enriched_dir, expected_enriched_dir)
    }
}
