// PURPOSE: IInlineUnusedProtocol — port trait for AES023: detect cross-language unused imports (Rust/Python/JS)
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IInlineUnusedProtocol: Send + Sync {
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
