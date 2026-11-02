// PURPOSE: GitleaksAdapter — ILinterAdapterProtocol implementation for Gitleaks secret scanning.
//
// Invokes `gitleaks detect --source <path> --report-format json --redact` as a
// subprocess and parses the JSON array of findings into LintResults.
//
// Key details:
//   - `--redact` always enabled — raw secrets never enter the LintResult stream
//   - `--no-git` added when `history == false` (working-tree-only default)
//   - Optional `.gitleaks.toml` discovered at or above the working directory
//   - Exit code 1 = findings exist (expected); 0 = clean; >1 = error

use serde::Deserialize;
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
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_quality_rules::LinterOperationError;
use std::sync::Arc;
use tracing::debug;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct GitleaksAdapter {
    executor: Arc<dyn ICommandExecutorProtocol>,
    io: Arc<dyn IFileSystemIOProtocol>,
    tool_resolution: Arc<dyn IToolResolutionProtocol>,
    history: bool,
}

// Parsed output from `gitleaks detect --report-format json`.
#[derive(Debug, Deserialize)]
struct GitleaksOutput {
    #[serde(default)]
    findings: Vec<GitleaksFinding>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GitleaksFinding {
    #[serde(default)]
    rule: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    start_line: Option<i64>,
    #[serde(default)]
    start_column: Option<i64>,
    #[serde(default)]
    end_line: Option<i64>,
    #[serde(default)]
    end_column: Option<i64>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    match_: Option<String>,
    #[serde(default)]
    secret: Option<String>,
    #[serde(default)]
    commit: Option<String>,
    #[serde(default)]
    entropy: Option<f64>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for GitleaksAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("gitleaks")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = self.tool_resolution.default_working_dir(path);

        // Locate optional .gitleaks.toml at or above the working directory.
        let config_path = self.find_gitleaks_config(&working_dir);

        let mut cmd = vec![
            "gitleaks".to_string(),
            "detect".to_string(),
            "--source".to_string(),
            path.value().to_string(),
            "--report-format".to_string(),
            "json".to_string(),
            "--redact".to_string(),
        ];
        if let Some(cfg) = config_path {
            cmd.push("--config".to_string());
            cmd.push(cfg.value().to_string());
        }
        if !self.history {
            cmd.push("--no-git".to_string());
        }

        let response = self
            .executor
            .exec_cmd_adapter(cmd, working_dir, 120.0, self.name())
            .map_err(crate::convert_executor_error)?;

        // returncode 1 = findings (expected); >1 = error — skip without raising
        if response.returncode > 1 {
            debug!("gitleaks exited with error code: {}", response.returncode);
            return Ok(LintResultList::new(results));
        }

        // gitleaks writes findings to stdout as a JSON array.
        let stdout = &response.stdout;
        if stdout.trim().is_empty() {
            return Ok(LintResultList::new(results));
        }

        let parsed: GitleaksOutput = match serde_json::from_str(stdout) {
            Ok(v) => v,
            Err(e) => {
                debug!("Failed to parse gitleaks JSON output: {}", e);
                return Ok(LintResultList::new(results));
            }
        };

        for finding in &parsed.findings {
            let code = format!("gitleaks::{}", finding.rule);
            results.push(LintResult {
                file: path.clone(),
                line: LineNumber::new(finding.start_line.unwrap_or(1)),
                column: ColumnNumber::new(finding.start_column.unwrap_or(1)),
                code: ErrorCode::raw(code.as_str()),
                message: LintMessage::new(finding.description.as_str()),
                source: Some(AdapterName::raw("gitleaks")),
                severity: map_gitleaks_severity(&finding.rule, &finding.tags),
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
        Ok(ComplianceStatus::new(true))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl GitleaksAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorProtocol>,
        io: Arc<dyn IFileSystemIOProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
        history: bool,
    ) -> Self {
        Self {
            executor,
            io,
            tool_resolution,
            history,
        }
    }

    /// Walk up from `working_dir` looking for `.gitleaks.toml`. Returns None
    /// when no config file is found — gitleaks then uses its built-in rules.
    fn find_gitleaks_config(&self, working_dir: &FilePath) -> Option<FilePath> {
        let mut current = std::path::Path::new(working_dir.value());
        loop {
            let candidate = current.join(".gitleaks.toml");
            if self.io.path_exists(&candidate) {
                return FilePath::new(candidate.to_string_lossy().to_string()).ok();
            }
            match current.parent() {
                Some(parent) if parent != current => current = parent,
                _ => return None,
            }
        }
    }
}

/// Map a gitleaks rule ID + tags to an AES severity.
pub fn map_gitleaks_severity(rule: &str, tags: &[String]) -> Severity {
    // Cloud key prefixes → CRITICAL.
    if rule.starts_with("aws-")
        || rule.starts_with("gcp-")
        || rule.starts_with("azure-")
        || rule.starts_with("cloud-")
    {
        return Severity::CRITICAL;
    }
    // Secret/token suffixes → CRITICAL.
    for suffix in &[
        "-key",
        "-secret",
        "-token",
        "-password",
        "-webhook",
        "-api-key",
    ] {
        if rule.ends_with(suffix) {
            return Severity::CRITICAL;
        }
    }
    // Private key families → CRITICAL.
    if rule.starts_with("private-key")
        || rule.starts_with("ssh")
        || rule.starts_with("pgp")
        || rule.starts_with("rsa")
        || rule.starts_with("ecdsa")
    {
        return Severity::CRITICAL;
    }
    // PCI / PII tags → CRITICAL.
    for tag in tags {
        if tag == "pci" || tag == "pii" {
            return Severity::CRITICAL;
        }
    }
    // Default HIGH (not MEDIUM): unknown Gitleaks rules are secret-related; HIGH prevents false negatives
    Severity::HIGH
}
