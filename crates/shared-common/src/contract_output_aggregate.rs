// PURPOSE: OutputAggregate — aggregate trait for output orchestration (format, write, display)
use output_report::contract_client_aggregate::OutputClientAggregate as BaseOutputClientAggregate;
use output_report::taxonomy_score_vo::FileFormat;
use shared_common::taxonomy_layer_vo::Identity;
use shared_common::taxonomy_source_vo::ContentString;
use source_parsing::taxonomy_path_vo::FilePath;

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

use output_report::taxonomy_result_vo::LintResultList;

pub trait IReportFormatterProtocol: Send + Sync {
    fn format_text(&self, results: &LintResultList, path: &str) -> String;
    fn format_json(&self, results: &LintResultList, path: &str) -> String;
}
