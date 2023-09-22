use std::path::Path;
use crate::destination_file::DestinationFile;
use super::file_traverser::TemplateFile;
use crate::shared_models::ZatResultX;
use crate::source_file::SourceFile;
use crate::config::UserConfigX;
use super::template_enricher::TemplateEnricher;
use super::enriched_template_file_processor::EnrichedTemplateFile;

pub struct DefaultTemplateEnricher {
  config: UserConfigX
}

impl DefaultTemplateEnricher {
  pub fn new(config: UserConfigX) -> Self {
    Self {
      config
    }
  }

  fn get_destination_file<P1, P2>(source_file: &SourceFile, source_root_path: P1, destination_root_path: P2) -> ZatResultX<DestinationFile>
    where P1: AsRef<Path>,
          P2: AsRef<Path>
  {
    source_file
      .strip_prefix(&source_root_path)
      .map(|relative_source_path|{
        let destination_file_path = destination_root_path.as_ref().join(&relative_source_path);
        DestinationFile::new(&destination_file_path.to_string_lossy().to_string())
      })
  }
}

impl TemplateEnricher for DefaultTemplateEnricher {
  fn enrich(&self, template_file: TemplateFile) ->  ZatResultX<EnrichedTemplateFile>  {

    let template_dir_path = &self.config.template_dir;
    let destination_dir_path = &self.config.target_dir;

    match template_file {
      TemplateFile::File(file) => {
        let source_file = SourceFile(file);
        let destination_file = Self::get_destination_file(&source_file, &template_dir_path, &destination_dir_path)?;
        Ok(EnrichedTemplateFile::File(source_file, destination_file))
      },
      TemplateFile::Dir(dir) => {
        let source_file = SourceFile(dir);
        let destination_file = Self::get_destination_file(&source_file, &template_dir_path, &destination_dir_path)?;
        Ok(EnrichedTemplateFile::Dir(destination_file))
      }
    }
  }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::source_file::SourceFile;
    use crate::destination_file::DestinationFile;

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn enriches_files() {
      let source_dir = tempdir().unwrap();
      let source_relative_path = "relative/source/file.ext";
      let destination_dir = "/some/destination/path";
      let source_file_path = source_dir.path().join(&source_relative_path);

      let file_template = TemplateFile::new_file(&source_file_path.to_str().unwrap());

      let config: UserConfigX =
        UserConfigX::new(& source_dir.path().to_str().unwrap(), &destination_dir);


      let enricher = DefaultTemplateEnricher::new(config);
      let enriched_file = enricher.enrich(file_template).expect(&format!("Could not enrich file: {}", &source_file_path.to_string_lossy().to_string()));

      let expected_destination_path = "/some/destination/path/relative/source/file.ext";
      let expected_enriched_file =
        EnrichedTemplateFile::File(
          SourceFile::new(&source_file_path.to_str().unwrap()),
          DestinationFile::new(&expected_destination_path));

      assert_eq!(enriched_file, expected_enriched_file)
    }


    #[test]
    fn enriches_directories() {
      let source_dir = tempdir().unwrap();
      let source_relative_path = "relative/source/path";
      let destination_dir = "/some/destination/path";
      let source_file_path = source_dir.path().join(&source_relative_path);

      let file_template = TemplateFile::new_dir(&source_file_path.to_str().unwrap());

      let config: UserConfigX =
        UserConfigX::new(& source_dir.path().to_str().unwrap(), &destination_dir);


      let enricher = DefaultTemplateEnricher::new(config);
      let enriched_dir = enricher.enrich(file_template).expect(&format!("Could not enrich file: {}", &source_file_path.to_string_lossy().to_string()));

      let expected_destination_path = "/some/destination/path/relative/source/path";
      let expected_enriched_dir =
        EnrichedTemplateFile::Dir(
          DestinationFile::new(&expected_destination_path)
        );

      assert_eq!(enriched_dir, expected_enriched_dir)
    }
}
