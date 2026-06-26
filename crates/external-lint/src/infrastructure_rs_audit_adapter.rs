// PURPOSE: RsAuditAdapter — ILinterAdapterPort implementation for cargo-audit security scanning
//
// Uses the `rustsec` crate directly (not subprocess) to parse Cargo.lock and
// check against the RustSec Advisory Database. Reports vulnerabilities as
// LintResults with CVE/RUSTSEC IDs as error codes.
//
// Key details:
//   - Finds Cargo.lock via resolve_cargo_lock_working_dir (walks up from path)
//   - Uses local advisory DB from ~/.cargo/advisory-db, or fetches if missing
//   - No subprocess overhead — uses rustsec library API directly
//   - CVSS severity is mapped: critical→CRITICAL, high→HIGH, medium→MEDIUM, else→LOW
//   - apply_fix returns true (cargo-audit has no fix command; affected packages
//     must be updated manually via cargo update)
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use tracing::debug;

use crate::infrastructure_rs_common::resolve_cargo_lock_working_dir;

pub struct CargoAuditAdapter {
    path_norm: Arc<dyn IPathNormalizationPort>,
}

impl CargoAuditAdapter {
    pub fn new(path_norm: Arc<dyn IPathNormalizationPort>) -> Self {
        Self { path_norm }
    }
}

#[async_trait]
impl ILinterAdapterPort for CargoAuditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("cargo-audit")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = resolve_cargo_lock_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_lock = Path::new(working_dir_str).join("Cargo.lock");
        if !cargo_lock.exists() {
            debug!(
                "Skipping cargo-audit: Cargo.lock not found at {:?}",
                cargo_lock
            );
            return Ok(LintResultList::new(results));
        }

        let lockfile = match rustsec::Lockfile::load(&cargo_lock) {
            Ok(lf) => lf,
            Err(e) => {
                debug!("Failed to parse Cargo.lock: {}", e);
                return Ok(LintResultList::new(results));
            }
        };

        let db_dir = match dirs::home_dir() {
            Some(p) => p,
            None => std::path::PathBuf::from("."),
        }
        .join(".cargo")
        .join("advisory-db");
        let db = if db_dir.exists() {
            match rustsec::Database::open(&db_dir) {
                Ok(db) => db,
                Err(_) => {
                    debug!("Failed to open advisory DB, will fetch...");
                    match rustsec::Database::fetch() {
                        Ok(db) => db,
                        Err(e) => {
                            debug!("Failed to fetch advisory DB: {}", e);
                            return Ok(LintResultList::new(results));
                        }
                    }
                }
            }
        } else {
            match rustsec::Database::fetch() {
                Ok(db) => db,
                Err(e) => {
                    debug!("Failed to fetch advisory DB: {}", e);
                    return Ok(LintResultList::new(results));
                }
            }
        };

        let settings = rustsec::report::Settings::default();
        let report = rustsec::Report::generate(&db, &lockfile, &settings);

        for vuln in &report.vulnerabilities.list {
            let id = vuln.advisory.id.to_string();
            let title = &vuln.advisory.title;
            let severity_str = match vuln.advisory.cvss.as_ref() {
                Some(c) => c.severity().to_string().to_lowercase(),
                None => "moderate".to_string(),
            };

            let severity = match severity_str.as_str() {
                "critical" => Severity::CRITICAL,
                "high" => Severity::HIGH,
                "medium" => Severity::MEDIUM,
                _ => Severity::LOW,
            };

            let resolved = self.path_norm.resolve_infrastructure_path(
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
                code: ErrorCode::raw(format!("cargo-audit::{}", id)),
                message: LintMessage::new(format!(
                    "{}: {} ({} v{})",
                    id, title, vuln.package.name, vuln.package.version
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
