use crate::code_analysis::contract_checker_aggregate::ICheckerAggregate;
use crate::output_report::taxonomy_score_vo::FileFormat;
use crate::shared_common::taxonomy_layer_vo::Identity;
use crate::shared_common::taxonomy_source_vo::ContentString;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub type CheckerAggregateRef = Box<dyn ICheckerAggregate>;

pub trait OutputClientAggregate: Send + Sync {
    fn get_output_dir(&self) -> Option<&FilePath>;
    fn write_output(
        &self,
        output: &ContentString,
        command: &Identity,
        output_format: Option<&FileFormat>,
    ) -> Option<FilePath>;
}
