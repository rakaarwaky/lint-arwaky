// PURPOSE: report command — counts per member plus what changed since the last
// run. Runs the same scan as `scan`, then prints a table instead of violations.
use dispatcher::surface_check_action::{FilesystemSeam, ScanOptions};
use shared_cli_commands::ReportSnapshot;
use shared_cli_commands::utility_report_delta::compare;
use shared_cli_commands::utility_report_renderer::render_report;
use shared_cli_commands::utility_report_snapshot::{read_store, target_key, write_store};
use shared_common::{ExitCode, FilePath};
use std::collections::BTreeMap;
use std::sync::Arc;
use tracing::error;

use shared_config_system::IConfigOrchestratorAggregate;
use shared_quality_rules::ICodeAnalysisAggregate;
use shared_structure_rules::IStructureAggregate;

/// Parameters for the `report` command.
pub struct ReportCommandParams {
    pub path: Option<FilePath>,
    pub config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filesystem_seam: FilesystemSeam,
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub structure_aggregate: Arc<dyn IStructureAggregate>,
    pub scan_aggregates: Option<dispatcher::surface_check_action::ScanAggregates>,
}

/// Count violations per member.
///
/// The key is the same `{top}/{member}` path the scan report groups by, so a
/// member counts identically whether the scan covered one member or the whole
/// workspace — otherwise a baseline would compare two different definitions.
pub fn count_by_member(violations: &[shared_common::ViolationItem]) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for v in violations {
        let member = member_key(&v.file.value);
        *counts.entry(member).or_default() += 1;
    }
    counts
}

/// The report's member key for a file: its member dir, or the file itself when
/// it sits directly under a member dir with no folder of its own.
///
/// This mirrors the scan report's own grouping, kept here rather than imported
/// because the formatter's key is private to that module and the rule is two
/// lines — the duplication is cheaper than exporting a formatter internal for
/// a caller that only needs the count.
fn member_key(file_path: &str) -> String {
    const MEMBER_DIRS: [&str; 3] = ["crates", "packages", "modules"];
    let path = std::path::Path::new(file_path);
    let mut segments: Vec<String> = Vec::new();
    let mut seen_member_dir = false;
    for part in path.components() {
        let name = part.as_os_str().to_string_lossy().into_owned();
        if MEMBER_DIRS.contains(&name.as_str()) {
            // Restart at the member dir: everything above it is the machine's
            // path to the workspace, not part of the member's own name.
            seen_member_dir = true;
            segments.clear();
            segments.push(name);
        } else if seen_member_dir {
            segments.push(name);
        }
    }
    match segments.len() {
        0 => path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| file_path.to_string()),
        1 => segments[0].clone(),
        _ => format!("{}/{}", segments[0], segments[1]),
    }
}

/// Seconds since the epoch, for the snapshot's timestamp.
fn now_seconds() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// `report` — a count table with deltas, not a violation list.
///
/// Exit code is success whenever the report prints: it is not a gate. A report
/// that failed on findings would make it useless for looking at progress.
pub fn handle_report(params: ReportCommandParams) -> ExitCode {
    let opts = ScanOptions {
        path: params.path.clone(),
        multi_project_orchestrator: params.config_orchestrator,
        filter: None,
        member: None,
        filesystem: Arc::new(params.filesystem_seam.clone()),
        scan_aggregates: params.scan_aggregates.clone(),
    };
    let target = params
        .path
        .as_ref()
        .map(|p| p.value.clone())
        .unwrap_or_else(|| ".".to_string());

    let violations = match dispatcher::surface_check_action::collect_scan(opts) {
        Ok(v) => v,
        Err(e) => {
            error!(error = %e, "scan failed");
            return ExitCode::RUNTIME_ERROR;
        }
    };

    let snapshot = ReportSnapshot {
        taken_at: now_seconds(),
        members: count_by_member(&violations),
    };
    let store = read_store();
    let previous = store.entries.get(&target_key(&target));
    let delta = compare(&snapshot, previous);

    render_report(&target, &delta, &snapshot);

    if let Err(e) = write_store(&target, snapshot) {
        // The report already printed; the next run simply has no baseline.
        error!(error = %e, "could not save the snapshot for the next comparison");
    }
    ExitCode::OK
}
