// PURPOSE: Role rules scan surface action
// Adapted: IRoleRunnerAggregate no longer has run_audit(path) — only run_audit_with_entries.
// Uses subprocess approach to invoke the binary for role scanning.
use shared::common::ExitCode;
use std::process::Command;
use std::sync::Arc;

use shared::cli_commands::Format;
use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::role_rules::IRoleRunnerAggregate;

use crate::surface_output_component::{ViolationItem, output_violations};

pub fn handle_scan_role(
    path: Option<FilePath>,
    format: Format,
    _role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    filter: Option<String>,
    fs_agg: Arc<dyn IFilesystemAggregate>,
) -> ExitCode {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::RUNTIME_ERROR;
    }

    let root_fp = match FilePath::new(root.clone()) {
        Ok(fp) => fp,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };

    // Use subprocess approach since IRoleRunnerAggregate only has run_audit_with_entries
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => std::path::PathBuf::from("lint-arwaky-cli"),
    };

    let output = Command::new(&exe_path)
        .args(["role", &root, "--format", "json"])
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
            eprintln!("[error] failed to run role linter: {e}");
            return ExitCode::RUNTIME_ERROR;
        }
    };

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    output_violations(&violations, &root, format, fs_agg.is_member_path(&root_fp));
    if violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}
