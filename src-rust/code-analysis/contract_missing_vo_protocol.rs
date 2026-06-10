// PURPOSE: IMissingVoProtocol — port trait for AES0303/AES0304: detect raw literals without Value Object wrapper
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IMissingVoProtocol: Send + Sync {
    fn check_missing_vo(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}
