// PURPOSE: External lint scan surface action
// AES406 NOTE: Uses subprocess approach (self-invocation) to run external lint scanning
// because IExternalLintAggregate::scan_all is async and no tokio runtime is available.
// This is a known gap — a sync scan method or async-aware surface layer should replace this.
// Adapted: uses subprocess approach since IExternalLintAggregate::scan_all is async
// and no tokio runtime is available in this crate.
use shared::common::ExitCode;
use std::process::Command;
use std::sync::Arc;

use shared::cli_commands::Format;
use shared::common::FilePath;
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use crate::surface_output_component::{ViolationItem, output_violations};

pub fn handle_scan_external(
    path: Option<FilePath>,
    format: Format,
    _external_lint: Arc<dyn IExternalLintAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    filter: Option<String>,
    filesystem: Arc<dyn IFilesystemAggregate>,
) -> ExitCode {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::RUNTIME_ERROR;
    }

    // Use subprocess approach — spawn external linter and parse JSON output
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => std::path::PathBuf::from("lint-arwaky-cli"),
    };

    let output = Command::new(&exe_path)
        .args(["external", &root, "--format", "json"])
        .output();

    let mut violations: Vec<ViolationItem> = match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
                if let Some(results) = val.get("results").and_then(|r| r.as_array()) {
                    results
                        .iter()
                        .filter_map(ViolationItem::from_json_obj)
                        .collect()
                } else if let Some(items) = val.as_array() {
                    items
                        .iter()
                        .filter_map(ViolationItem::from_json_obj)
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        }
        Err(e) => {
            eprintln!("[error] failed to run external linter: {e}");
            return ExitCode::RUNTIME_ERROR;
        }
    };

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    let has_violations = !violations.is_empty();
    let root_fp = FilePath::new(root.clone()).unwrap_or_default();
    output_violations(
        &violations,
        &root,
        format,
        filesystem.is_member_path(&root_fp),
    );
    if has_violations {
        ExitCode::POLICY_FAIL
    } else {
        ExitCode::OK
    }
}
