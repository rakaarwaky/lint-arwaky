// PURPOSE: IBypassCheckerProtocol — port trait for AES304: detect bypass comments, unwrap/expect, panic
use crate::cli_commands::taxonomy_result_vo::LintResult;

pub trait IBypassCheckerProtocol: Send + Sync {
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>);
}
