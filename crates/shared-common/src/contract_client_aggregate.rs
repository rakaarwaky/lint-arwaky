// PURPOSE: OutputClientAggregate — aggregate trait for output client (stdout, file, tee)
use output_report::taxonomy_score_vo::FileFormat;
use shared_common::taxonomy_layer_vo::Identity;
use shared_common::taxonomy_source_vo::ContentString;
use source_parsing::taxonomy_path_vo::FilePath;

pub trait OutputClientAggregate: Send + Sync {
    fn get_output_dir(&self) -> Option<&FilePath>;
    fn write_output(
        &self,
        output: &ContentString,
        command: &Identity,
        output_format: Option<&FileFormat>,
    ) -> Option<FilePath>;
}
