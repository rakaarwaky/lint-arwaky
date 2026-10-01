// PURPOSE: quality-domain capability contracts (AES102 `_protocol`).
//
// One file for the quality feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::taxonomy_quality_rules_vo::AesCodeAnalysisViolation;
use shared_common::taxonomy_definition_vo::LayerDefinition;
use shared_common::taxonomy_lint_result_vo::LintResult;
use std::path::PathBuf;

pub trait IBypassCheckerProtocol: Send + Sync {
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>);
}

pub trait IMandatoryClassProtocol: Send + Sync {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
}

/// Protocol for analysing source-code metrics such as duplication.
///
/// Accepts pre-fetched (path, content) entries from the caller and
/// returns the resulting (file_path, violation) tuples.
pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    /// Run duplication analysis on pre-fetched (path, content) entries.
    /// The caller is responsible for discovering and reading files.
    fn handle_duplicates_entries(
        &self,
        entries: &[(PathBuf, String)],
    ) -> Vec<(String, AesCodeAnalysisViolation)>;
}

/// Protocol for detecting dead (empty) struct and impl blocks.
///
/// AES303 requires that every struct and impl block contain at least one
/// meaningful item. This protocol checks for violations and appends them
/// to the provided violations vector.
pub trait IDeadInheritanceProtocol: Send + Sync {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}

pub trait ILineCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
}
