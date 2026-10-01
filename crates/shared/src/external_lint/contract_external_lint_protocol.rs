// PURPOSE: external-lint-domain capability contracts (AES102 `_protocol`).
//
// One file for the external-lint feature. One trait per functional requirement
// in `crates/external-lint/FRD.md`. The aggregate seam
// (`IExternalLintAggregate`) is the entry point and is not counted toward the
// 1:1 FR-to-protocol mapping.
//
// FR-ExternalLint-001 (project language detection) deliberately has no trait
// here: language detection belongs to the `filesystem` aggregate, which serves
// it as `FilesystemRequest::DetectProjectLanguages`. A feature does not own a
// protocol for behaviour another feature already owns.

use crate::taxonomy_duration_vo::Timeout;
use shared_common::taxonomy_adapter_list_vo::AdapterNameList;
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_message_vo::ComplianceStatus;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_response_data_vo::ResponseData;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_quality_rules::taxonomy_operation_error::LinterOperationError;
use shared_quality_rules::taxonomy_quality_rules_vo::LintResultList;

/// FR-ExternalLint-001: select the set of adapters to run given the detected
/// language booleans and whether Markdown files are present. Returns the ordered
/// list of adapter names in language-group order (Rust → Python → JS →
/// markdown). When `has_markdown` is true, `markdownlint` is appended after the
/// JS group.
///
/// The language booleans are pre-computed by the caller — either from the
/// surface, or from the `filesystem` aggregate's
/// `FilesystemRequest::DetectProjectLanguages`. This trait maps flags to
/// adapter names and nothing else.
pub trait IExternalLintSelectorProtocol: Send + Sync {
    fn select_adapters(
        &self,
        has_rs: bool,
        has_py: bool,
        has_js: bool,
        has_md: bool,
    ) -> AdapterNameList;
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

// ─── Utility Protocols ──────────────────────────────────────
// These protocol traits define the technical seams for utility implementations
// (StdioClient, JS/Cargo path resolvers). They live in the contract module
// because AES404 forbids trait definitions in the utility layer, and the
// `_protocol` suffix is contract-layer-only (AES102). Capability adapters
// depend on them via dependency injection.

/// Utility: execute an external linter tool as a subprocess with timeout,
/// stdout/stderr capture, and error mapping.
///
/// Implemented by StdioClient (utility layer).
pub trait ICommandExecutorProtocol: Send + Sync {
    /// Run a command and return stdout, stderr, and return code.
    fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData>;

    /// Check the health of the execution transport.
    fn health_check(&self) -> anyhow::Result<ResponseData>;

    /// Execute a command, mapping failures to `LinterOperationError::Scan`.
    fn exec_cmd_scan(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError>;

    /// Execute a command, mapping failures to `LinterOperationError::Adapter`.
    fn exec_cmd_adapter(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError>;
}

/// Utility: resolve JS/TS tool paths, preferring local
/// `node_modules/.bin/<tool>` binaries over global PATH installations. Also
/// resolves the working directory by walking up for the nearest config file,
/// and runs a JS tool's native fix command over the resolved path.
pub trait IJsToolResolutionProtocol: Send + Sync {
    /// Resolve the full command for a JS tool, falling back to global PATH if
    /// a local binary is not found.
    fn resolve_js_cmd(
        &self,
        tool_name: &ToolName,
        args: Vec<String>,
        working_dir: &FilePath,
    ) -> Option<Vec<String>>;

    /// Walk up to 10 parent directories for the nearest JS/TS config file and
    /// return the directory containing it.
    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath;

    /// Apply a JS tool's fix command via `node_modules/.bin/` resolution.
    /// Returns a no-op status when the tool cannot be resolved.
    fn js_apply_fix(
        &self,
        path: &FilePath,
        tool: &ToolName,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError>;
}

/// Utility: find the directory containing `Cargo.toml` or
/// `Cargo.lock` for a given target path, used by Rust adapters.
pub trait ICargoDirProtocol: Send + Sync {
    /// Resolve the directory containing the nearest `Cargo.toml` (or
    /// `Cargo.lock` for audit targets).
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath;
}
