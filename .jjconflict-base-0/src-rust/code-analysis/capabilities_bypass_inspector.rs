//! Inline bypass and agent-wildcard checks.

use std::path::Path;

use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violationrs_constant::{
    AES022_BYPASS_COMMENT, AES022_PANIC, AES022_UNWRAP_EXPECT,
};

/// Create a LintResult — shared by all inline checkers.
fn mk_result(file: &str, line: usize, code: &str, sev: Severity, msg: &str) -> LintResult {
    LintResult::new_arch(file, line, code, sev, msg)
}
