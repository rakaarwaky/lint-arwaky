// PURPOSE: Protocol: Contract trait for Line
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;

pub trait ILineCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    );
}
