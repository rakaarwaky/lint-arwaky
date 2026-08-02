// PURPOSE: CI entry point — Utility Surface action
// CLASSIFICATION: Utility Surface
//   - Centralizes business logic for CI pipeline
//   - Called by Smart surfaces (CLI, MCP, TUI)
use shared::common::ExitCode;
use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;

pub fn handle_ci(
    path: &FilePath,
    code_analysis: &dyn ICodeAnalysisAggregate,
    import_orchestrator: &dyn IImportRunnerAggregate,
    naming_orchestrator: &dyn INamingRunnerAggregate,
    orphan_orchestrator: &dyn IOrphanAggregate,
    filesystem: &dyn IFilesystemAggregate,
    threshold: f64,
) -> ExitCode {
    // 1. Validate path exists
    if !filesystem.path_exists(std::path::Path::new(path.value())) {
        eprintln!("Error: path '{}' does not exist", path.value());
        return ExitCode::RUNTIME_ERROR;
    }

    // 2. Run all linters
    let mut results = code_analysis.run_code_analysis_path(path);

    if let Ok(import_res) = import_orchestrator.run_audit(path) {
        results.extend(import_res);
    }

    let naming_res = naming_orchestrator.run_audit_with_entries(filesystem.file_list());
    results.extend(naming_res);

    let (_, orphan_res) = orphan_orchestrator.scan_orphans(path, &[]);
    results.extend(orphan_res);

    // 3. Calculate score
    let score = code_analysis.calc_score(&results);
    let has_critical = code_analysis.check_critical(&results);
    let below_threshold = score.value() < threshold;

    // 4. Return exit code
    if has_critical.value() || below_threshold {
        ExitCode::POLICY_FAIL
    } else {
        ExitCode::OK
    }
}
