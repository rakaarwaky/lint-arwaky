// PURPOSE: PyBanditAdapter — ILinterAdapterProtocol implementation for Bandit security scanner integration
//
// Runs `bandit -r <path> --format json --exit-zero` to scan Python files for
// security vulnerabilities. Parses JSON output to extract findings (filename,
// line_range, test_id, issue_text, severity).
//
// Key details:
//   - `--exit-zero` ensures bandit always exits 0 regardless of findings
//   - JSON output avoids fragile regex parsing
//   - Severity is directly mapped: HIGH→HIGH, MEDIUM→MEDIUM, LOW→LOW
//   - apply_fix always returns false (Bandit is a scanner, not a fixer)

use serde_json::Value;
use shared_cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::{ComplianceStatus, LintMessage};
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_external_lint::utility_path_normalization::resolve_or_fallback_with_context;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_quality_rules::LinterOperationError;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct BanditAdapter {
    pub tool_resolution: Arc<dyn IToolResolutionProtocol>,
    pub io: Arc<dyn IFileSystemIOProtocol>,
    lint_executor: Arc<dyn ICommandExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for BanditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("bandit")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !self.tool_resolution.is_python_file_recursive(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let abs_path = self.io.canonicalize_path_str(path);
        // Exclude heavy dependency and worktree trees: vendor/ and .venv/
        // (dependency trees), .worktrees/ (may nest full venvs incl.
        // vendored bandit copies), site-packages, __pycache__, and
        // node_modules. Bandit exceeds its 120s timeout on these, and the
        // adapter silently reports zero findings (#993 follow-up). First-party
        // Python files remain in scope.
        let cmd = vec![
            executable,
            "-r".to_string(),
            abs_path.value.to_string(),
            "--exclude".to_string(),
            "tests,.venv,vendor,.worktrees,site-packages,__pycache__,node_modules".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--exit-zero".to_string(),
        ];
        let working_dir = self.tool_resolution.default_working_dir(path);

        let response = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 120.0, self.name())
            .map_err(crate::convert_executor_error)?;

        let stdout = &response.stdout;
        let parsed: Value = match serde_json::from_str(stdout) {
            Ok(v) => v,
            Err(_) => Value::Object(serde_json::Map::new()),
        };
        let findings = match parsed.get("results").and_then(|v| v.as_array()) {
            Some(arr) => arr.clone(),
            None => Vec::new(),
        };
        let mut results = Vec::new();

        for f in &findings {
            let filename = f
                .get("filename")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let line_number = f
                .get("line_number")
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let test_id = f.get("test_id").and_then(|v| v.as_str()).unwrap_or("B000");
            // FRD: tool-native codes carry a `bandit::` prefix.
            let code = format!("bandit::{}", test_id);
            let issue_text = f
                .get("issue_text")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let issue_severity = f
                .get("issue_severity")
                .and_then(|v| v.as_str())
                .unwrap_or("MEDIUM");
            let issue_confidence = f
                .get("issue_confidence")
                .and_then(|v| v.as_str())
                .unwrap_or("MEDIUM");

            let resolved =
                resolve_or_fallback_with_context(filename, path.clone(), Some(path.clone()));

            results.push(LintResult {
                file: resolved,
                line: LineNumber::new(line_number),
                // Bandit has no column data: use the 1-based column default
                // instead of the line_range span (which is a line number, not a
                // column — #913).
                column: ColumnNumber::new(1),
                code: ErrorCode::raw(code),
                message: LintMessage::new(issue_text),
                source: Some(self.name()),
                severity: self.map_severity(issue_severity, issue_confidence),
                enclosing_scope: None,
                related_locations: LocationList::new(),
                violation_name: String::new(),
                why: String::new(),
                fix: String::new(),
            });
        }
        Ok(LintResultList::new(results))
    }

    fn fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl BanditAdapter {
    pub fn new(
        lint_executor: Arc<dyn ICommandExecutorProtocol>,
        bin_path: Option<FilePath>,
        io: Arc<dyn IFileSystemIOProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
    ) -> Self {
        Self {
            lint_executor,
            bin_path,
            io,
            tool_resolution,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value().to_string(),
            None => "bandit".to_string(),
        }
    }

    pub fn map_severity(&self, severity: &str, confidence: &str) -> Severity {
        // FR-004: Bandit severity — HIGH confidence + HIGH severity → CRITICAL.
        match (severity, confidence) {
            ("HIGH", "HIGH") => Severity::CRITICAL,
            ("HIGH", _) => Severity::HIGH,
            ("MEDIUM", _) => Severity::MEDIUM,
            ("LOW", _) => Severity::LOW,
            _ => Severity::MEDIUM,
        }
    }
}
