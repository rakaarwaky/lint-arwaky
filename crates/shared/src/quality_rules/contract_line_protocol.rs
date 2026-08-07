// PURPOSE: ILineCheckerProtocol — protocol trait for AES301/AES302: check file line count limits
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_lint_result_vo::LintResult;

pub trait ILineCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
}
