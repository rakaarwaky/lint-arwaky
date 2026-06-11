// PURPOSE: ILineCheckerProtocol — port trait for AES020/AES021: check file line count limits
use output_report::taxonomy_result_vo::LintResult;
use shared_common::taxonomy_definition_vo::LayerDefinition;

pub trait ILineCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    );
}
