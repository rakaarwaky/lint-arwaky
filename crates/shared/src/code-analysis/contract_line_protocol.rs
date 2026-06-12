// PURPOSE: ILineCheckerProtocol — port trait for AES020/AES021: check file line count limits
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait ILineCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    );
}
