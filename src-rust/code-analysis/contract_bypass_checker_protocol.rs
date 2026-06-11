// PURPOSE: IBypassCheckerProtocol — port trait for AES022: detect bypass comments, unwrap/expect, panic
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IBypassCheckerProtocol: Send + Sync {
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
