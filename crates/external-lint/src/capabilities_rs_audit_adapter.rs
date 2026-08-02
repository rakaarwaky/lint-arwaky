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

use async_trait::async_trait;
use serde::Deserialize;
use shared::cli_commands::{LintResult, LintResultList};

use shared::code_analysis::{ILinterAdapterProtocol, LinterOperationError};

use shared::common::{FilePath, Severity};

use shared::common::{
    AdapterName, ColumnNumber, ComplianceStatus, ErrorCode, LineNumber, LintMessage, LocationList,
};

use std::path::Path;
use tracing::debug;
use std::sync::Arc;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CargoAuditAdapter {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
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

#[async_trait]
impl ILinterAdapterProtocol for CargoAuditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("cargo-audit")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = self.filesystem.resolve_cargo_lock_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_lock = Path::new(working_dir_str).join("Cargo.lock");
        if !cargo_lock.exists() {
            debug!(
                "Skipping cargo-audit: Cargo.lock not found at {:?}",
                cargo_lock
            );
            return Ok(LintResultList::new(results));
        }

        // Run cargo-audit as subprocess (same pattern as clippy/rustfmt adapters)
        let output = match tokio::process::Command::new("cargo")
            .arg("audit")
            .arg("--json")
            .current_dir(working_dir_str)
            .output()
            .await
        {
            Ok(o) => o,
            Err(e) => {
                debug!("Failed to run cargo-audit: {}", e);
                return Ok(LintResultList::new(results));
            }
        };

        if !output.status.success() {
            debug!("cargo-audit exited with status: {:?}", output.status.code());
            // cargo-audit exits non-zero when vulnerabilities are found — that's OK
        }

        // Parse the JSON output
        let stdout = String::from_utf8_lossy(&output.stdout);
        let parsed: CargoAuditOutput = match serde_json::from_str(&stdout) {
            Ok(v) => v,
            Err(e) => {
                debug!("Failed to parse cargo-audit JSON: {}", e);
                return Ok(LintResultList::new(results));
            }
        };

        for vuln in &parsed.vulnerabilities {
            // FR-004: cargo-audit severity — case-insensitive match.
            // Tool outputs title-case ("Critical", "High", "Medium", "Low", "Unknown").
            let severity = match vuln.severity.as_deref().map(str::to_lowercase).as_deref() {
                Some("critical") => Severity::CRITICAL,
                Some("high") => Severity::HIGH,
                Some("medium") => Severity::MEDIUM,
                Some("low") | Some("unknown") | None => Severity::LOW,
                _ => Severity::LOW,
            };

            let resolved = shared::common::utility_path_normalization::resolve_capabilities_path(
                match FilePath::new("Cargo.lock".to_string()) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                },
                Some(path.clone()),
            );
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
            });
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(true))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl CargoAuditAdapter {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self { filesystem }
    }
}

impl Default for CargoAuditAdapter {
    fn default() -> Self {
        Self::new(Arc::new(filesystem::FilesystemOrchestrator::new()))
    }
}
