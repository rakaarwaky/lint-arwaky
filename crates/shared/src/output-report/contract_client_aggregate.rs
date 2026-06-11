// PURPOSE: OutputClientAggregate — aggregate trait for output client (stdout, file, tee)
use crate::common::taxonomy_layer_vo::Identity;
use crate::config_system::taxonomy_source_vo::ContentString;
use crate::output_report::taxonomy_score_vo::FileFormat;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait OutputClientAggregate: Send + Sync {
    fn get_output_dir(&self) -> Option<&FilePath>;
    fn write_output(
        &self,
        output: &ContentString,
        command: &Identity,
        output_format: Option<&FileFormat>,
    ) -> Option<FilePath>;
}
