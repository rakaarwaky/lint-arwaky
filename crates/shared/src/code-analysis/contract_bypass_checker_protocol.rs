// PURPOSE: IBypassCheckerProtocol — port trait for AES304: detect bypass comments, unwrap/expect, panic
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;

pub trait IBypassCheckerProtocol: Send + Sync {
    fn check_bypass_comments(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
    );
    fn check_cargo_toml(&self, content: &ContentString, violations: &mut Vec<LintResult>);
}
