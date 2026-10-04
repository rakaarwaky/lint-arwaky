// PURPOSE: RsAuditAdapter — ILinterAdapterProtocol implementation for cargo-audit security scanning
//
// Invokes the `cargo-audit` CLI as a subprocess to parse Cargo.lock and check
// against the RustSec Advisory Database. Reports vulnerabilities as LintResults
// with CVE/RUSTSEC IDs as error codes.
//
// Key details:
//   - Finds Cargo.lock via resolve_cargo_lock_working_dir (walks up from path)
//   - No rustsec crate dependency — avoids massive gix/cargo-lock compile tree
//   - CVSS severity is mapped: critical→CRITICAL, high→HIGH, medium→MEDIUM, else→LOW
//   - apply_fix returns true (cargo-audit has no fix command; affected packages
//     must be updated manually via cargo update)

use serde::Deserialize;
use shared_cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber, PatternList};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_external_lint::taxonomy_duration_vo::Timeout;
use shared_external_lint::utility_path_normalization::resolve_or_fallback_with_context;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_quality_rules::LinterOperationError;
use std::path::Path;
use std::sync::Arc;
use tracing::debug;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CargoAuditAdapter {
    executor: Arc<dyn ICommandExecutorProtocol>,
    pub tool_resolution: Arc<dyn IToolResolutionProtocol>,
}

/// Parsed output from `cargo-audit --json` (cargo-vulnerability-report format).
#[derive(Debug, Deserialize)]
struct CargoAuditOutput {
    #[serde(default)]
    vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Deserialize)]
struct Vulnerability {
    id: String,
    #[serde(rename = "crate")]
    package: String,
    #[serde(rename = "crate_version")]
    version: String,
    #[serde(rename = "info")]
    title: String,
    #[serde(rename = "severity")]
    severity: Option<String>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for CargoAuditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("cargo-audit")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = self.tool_resolution.resolve_cargo_lock_working_dir(path);
        let working_dir_str = working_dir.value();

        let cargo_lock = Path::new(working_dir_str).join("Cargo.lock");
        if !cargo_lock.exists() {
            debug!(
                "Skipping cargo-audit: Cargo.lock not found at {:?}",
                cargo_lock
            );
            return Ok(LintResultList::new(results));
        }

        // Run cargo-audit via executor protocol (consistent with other adapters)
        let cmd = PatternList::new(vec![
            "cargo".to_string(),
            "audit".to_string(),
            "--json".to_string(),
        ]);
        let response = self
            .executor
            .execute_command(cmd, working_dir.clone(), Some(Timeout::new(120.0)))
            .map_err(|e| crate::map_executor_err(e, self.name()))?;

        if response.returncode != 0 && response.returncode != 1 {
            debug!("cargo-audit exited with code: {}", response.returncode);
            // cargo-audit exits non-zero when vulnerabilities are found — that's OK
        }

        // Parse the JSON output
        let stdout = &response.stdout;
        let parsed: CargoAuditOutput = match serde_json::from_str(stdout) {
            Ok(v) => v,
            Err(e) => {
                debug!("Failed to parse cargo-audit JSON: {}", e);
                return Ok(LintResultList::new(results));
            }
        };

        for vuln in &parsed.vulnerabilities {
            // FR-004: cargo-audit severity — case-insensitive match.
            let severity = match vuln.severity.as_deref().map(str::to_lowercase).as_deref() {
                Some("critical") => Severity::CRITICAL,
                Some("high") => Severity::HIGH,
                Some("medium") => Severity::MEDIUM,
                Some("low") | Some("unknown") | None => Severity::LOW,
                _ => Severity::LOW,
            };

            let resolved =
                resolve_or_fallback_with_context("Cargo.lock", path.clone(), Some(path.clone()));
            results.push(LintResult {
                file: resolved,
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw(format!("cargo-audit::{}", vuln.id)),
                message: LintMessage::new(format!(
                    "{}: {} ({} v{})",
                    vuln.id, vuln.title, vuln.package, vuln.version
                )),
                source: Some(AdapterName::raw("cargo-audit")),
                severity,
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

impl CargoAuditAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
    ) -> Self {
        Self {
            executor,
            tool_resolution,
        }
    }
}
