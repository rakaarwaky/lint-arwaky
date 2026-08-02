// PURPOSE: Naming rules scan surface action
//
// Data Flow:
//   CLI → handle_scan_naming → filesystem.file_list() → naming_orchestrator.run_audit_with_entries → violations → output
//
// The surface layer is responsible for:
//   1. Path validation (delegated to filesystem aggregate)
//   2. Fetching pre-populated file entries from filesystem aggregate
//   3. Delegating audit to naming orchestrator (zero I/O in agent layer)
//   4. Formatting and printing violations
//   5. Returning ExitCode (OK or POLICY_FAIL)
//
// The naming-rules crate performs zero I/O — it receives &[FileEntry] and
// returns LintResult violations. All filesystem access is handled by the
// filesystem aggregate via the surface layer.
use shared::common::ExitCode;
use std::sync::Arc;

use shared::cli_commands::Format;
use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::naming_rules::INamingRunnerAggregate;

use crate::surface_output_component::{ViolationItem, output_violations};

pub fn handle_scan_naming(
    path: Option<FilePath>,
    format: Format,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    filter: Option<String>,
    fs_agg: Arc<dyn IFilesystemAggregate>,
) -> ExitCode {
    // 1. Resolve target path (default: current directory)
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };

    // 2. Validate path exists (delegated to filesystem aggregate)
    if !fs_agg.path_exists(std::path::Path::new(&root)) {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::RUNTIME_ERROR;
    }
    let root_fp = match FilePath::new(root.clone()) {
        Ok(fp) => fp,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };

    // 3. Run naming audit — surface fetches cached file entries from filesystem,
    //    passes them to orchestrator. Orchestrator does zero I/O, only delegates
    //    to naming_convention_checker (AES101) and suffix_prefix_checker (AES102).
    let results = naming_orchestrator.run_audit_with_entries(fs_agg.file_list());

    // 4. Convert LintResult to ViolationItem for output formatting
    let mut violations: Vec<ViolationItem> = results
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    // 5. Apply optional filter (by violation code)
    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    // 6. Output violations and return exit code
    output_violations(&violations, &root, format, fs_agg.is_member_path(&root_fp));
    if violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}
