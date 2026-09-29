// PURPOSE: external-lint business capability contracts (AES102 `_protocol`).
//
// Three protocols, one per FR in `crates/external-lint/FRD.md`.
// Utility concerns (subprocess execution, JS tool resolution) are handled
// internally by concrete types and are not exposed as protocols.

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_tool_name_vo::ToolName;
use crate::external_lint::taxonomy_external_lint_vo::ExternalLintContext;
use crate::quality_rules::taxonomy_operation_error::LinterOperationError;
use crate::quality_rules::taxonomy_quality_rules_vo::LintResultList;

/// FR-ExternalLint-001: run every selected adapter sequentially, aggregating
/// results and post-filtering them by the context's ignored paths. One adapter
/// failing never stops the remaining adapters.
pub trait IAdapterScanProtocol: Send + Sync {
    /// Run `path` against every adapter selected for `context`, in adapter-list
    /// order, and return the aggregated results.
    fn scan_all(&self, path: &FilePath, context: &ExternalLintContext) -> LintResultList;
}

/// FR-ExternalLint-002: each linter adapter exposes the canonical scanning and
/// auto-fix surface. Fix-capable adapters run the tool's native fix command;
/// non-fixing adapters (MyPy, Bandit, TSC, cargo-audit) return a no-op
/// `ComplianceStatus`.
///
/// `name()` is part of this seam so the orchestrator can log and dispatch
/// per-adapter using a stable identifier.
pub trait ILinterAdapterProtocol: Send + Sync {
    /// Stable name used as the adapter lookup key and log label.
    fn name(&self) -> AdapterName;

    /// Run the linter on `path`, returning normalized results.
    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError>;

    /// Run the tool's native fix command (or no-op for non-fixing tools).
    fn fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError>;
}

/// FR-ExternalLint-003: normalize each adapter's external tool output into
/// `LintResult` structs, preserving tool-native rule codes (e.g.
/// `clippy::needless_return`, `ruff::E501`) and applying the per-tool severity
/// mapping. File paths are canonicalized against `root`.
pub trait INormalizeProtocol: Send + Sync {
    /// Normalize raw subprocess output into a list of `LintResult`s plus any
    /// warnings raised while parsing.
    fn normalize(
        &self,
        tool_name: &ToolName,
        raw_output: &str,
        root: &FilePath,
    ) -> (LintResultList, Vec<String>);

    /// Map one tool-native severity or rule code to a lint-arwaky `Severity`.
    fn map_severity(
        &self,
        tool_name: &ToolName,
        code: &ErrorCode,
        tool_severity: &Severity,
    ) -> Severity;
}
