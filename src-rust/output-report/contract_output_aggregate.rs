// PURPOSE: OutputAggregate — aggregate trait for output orchestration (format, write, display)
use crate::output_report::contract_client_aggregate::OutputClientAggregate as BaseOutputClientAggregate;
use crate::output_report::taxonomy_score_vo::FileFormat;
use crate::shared_common::taxonomy_layer_vo::Identity;
use crate::shared_common::taxonomy_source_vo::ContentString;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub type OutputClientDyn = Box<dyn BaseOutputClientAggregate>;

pub trait OutputClientAggregate: Send + Sync {
    fn get_output_dir(&self) -> Option<&FilePath>;
    fn write_output(
        &self,
        output: &ContentString,
        command: &Identity,
        output_format: Option<&FileFormat>,
    ) -> Option<FilePath>;
}

use crate::output_report::taxonomy_result_vo::LintResultList;

pub trait IReportFormatterProtocol: Send + Sync {
    fn format_text(&self, results: &LintResultList, path: &str) -> String;
    fn format_json(&self, results: &LintResultList, path: &str) -> String;
}
