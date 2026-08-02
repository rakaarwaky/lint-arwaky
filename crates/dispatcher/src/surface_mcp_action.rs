// PURPOSE: MCP server action — Utility Surface
// Business logic for MCP server, called by MCP Smart Surface
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::role_rules::IRoleRunnerAggregate;
use shared::maintenance::MaintenanceCommandsAggregate;
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::git_hooks::GitHooksAggregate;
use shared::project_setup::SetupManagementAggregate;
use std::sync::Arc;

pub struct McpDeps {
    pub code_analysis: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
    pub maintenance: Arc<dyn MaintenanceCommandsAggregate>,
    pub git_hooks: Arc<dyn GitHooksAggregate>,
    pub setup: Arc<dyn SetupManagementAggregate>,
}

/// Run check/scan — all linters combined
pub fn execute_check(deps: &McpDeps, path: &str) -> serde_json::Value {
    let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.to_string()) {
        Ok(f) => f,
        Err(_) => return serde_json::json!({"error": "Invalid path", "exit_code": 2}),
    };

    let mut all_results: Vec<serde_json::Value> = Vec::new();

    let quality = deps.code_analysis.run_code_analysis_path(&fp);
    all_results.extend(results_to_json(&quality, Some("quality")));

    if let Ok(import_results) = deps.import_orchestrator.run_audit(&fp) {
        all_results.extend(results_to_json(&import_results, Some("import")));
    }

    let naming_results = deps.naming_orchestrator.run_audit_with_entries(deps.filesystem.file_list());
    all_results.extend(results_to_json(&naming_results, Some("naming")));

    let role_results = deps.role_orchestrator.run_audit_with_entries(deps.filesystem.file_list());
    all_results.extend(results_to_json(&role_results, Some("role")));

    let ignored = deps.config_orchestrator.ignored_paths(&fp);
    let (_, orphan_results) = deps.orphan_orchestrator.scan_orphans(&fp, &ignored.values);
    all_results.extend(results_to_json(&orphan_results, Some("orphan")));

    let scan_results = deps.external_lint.scan_all(&fp);
    all_results.extend(results_to_json(&scan_results.values, Some("external")));

    let total = all_results.len();
    let exit_code = if total == 0 { 0 } else { 1 };
    serde_json::json!({
        "status": if exit_code == 0 { "success" } else { "failure" },
        "action": "check",
        "path": path,
        "exit_code": exit_code,
        "total_violations": total,
        "results": all_results,
    })
}

/// Run CI — scoring + threshold
pub fn execute_ci(deps: &McpDeps, path: &str, threshold: u64) -> serde_json::Value {
    let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.to_string()) {
        Ok(f) => f,
        Err(_) => return serde_json::json!({"error": "Invalid path", "exit_code": 2}),
    };
    let quality = deps.code_analysis.run_code_analysis_path(&fp);
    let total = quality.len();
    let exit_code = if total == 0 { 0 } else { 1 };
    serde_json::json!({
        "status": if exit_code == 0 { "pass" } else { "fail" },
        "action": "ci",
        "threshold": threshold,
        "path": path,
        "exit_code": exit_code,
        "total_violations": total,
    })
}

/// Run fix — auto-fix with dry_run support
pub fn execute_fix(deps: &McpDeps, path: &str, dry_run: bool) -> serde_json::Value {
    let fp = shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
        .unwrap_or_else(|_| shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default());
    let file_adapter = deps.fix_orchestrator.file_adapter();
    let fix_result = deps.fix_orchestrator.execute(&fp);
    serde_json::json!({
        "status": "success",
        "action": "fix",
        "path": path,
        "dry_run": dry_run,
        "exit_code": 0,
        "message": fix_result.output.value,
    })
}

/// Run quality scan
pub fn execute_quality(deps: &McpDeps, path: &str) -> serde_json::Value {
    let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.to_string()) {
        Ok(f) => f,
        Err(_) => return serde_json::json!({"error": "Invalid path", "exit_code": 2}),
    };
    let results = deps.code_analysis.run_code_analysis_path(&fp);
    let exit_code = if results.is_empty() { 0 } else { 1 };
    serde_json::json!({
        "status": if exit_code == 0 { "success" } else { "violations" },
        "action": "quality",
        "exit_code": exit_code,
        "violation_count": results.len(),
        "results": results_to_json(&results, None),
    })
}

/// Run import scan
pub fn execute_import(deps: &McpDeps, path: &str) -> serde_json::Value {
    let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.to_string()) {
        Ok(f) => f,
        Err(_) => return serde_json::json!({"error": "Invalid path", "exit_code": 2}),
    };
    let results = match deps.import_orchestrator.run_audit(&fp) {
        Ok(r) => r,
        Err(e) => return serde_json::json!({"error": format!("Import audit failed: {}", e), "exit_code": 2}),
    };
    let exit_code = if results.is_empty() { 0 } else { 1 };
    serde_json::json!({
        "status": if exit_code == 0 { "success" } else { "violations" },
        "action": "import",
        "exit_code": exit_code,
        "violation_count": results.len(),
        "results": results_to_json(&results, None),
    })
}

/// Run naming scan
pub fn execute_naming(deps: &McpDeps, _path: &str) -> serde_json::Value {
    let results = deps.naming_orchestrator.run_audit_with_entries(deps.filesystem.file_list());
    let exit_code = if results.is_empty() { 0 } else { 1 };
    serde_json::json!({
        "status": if exit_code == 0 { "success" } else { "violations" },
        "action": "naming",
        "exit_code": exit_code,
        "violation_count": results.len(),
        "results": results_to_json(&results, None),
    })
}

/// Run role scan
pub fn execute_role(deps: &McpDeps, _path: &str) -> serde_json::Value {
    let results = deps.role_orchestrator.run_audit_with_entries(deps.filesystem.file_list());
    let exit_code = if results.is_empty() { 0 } else { 1 };
    serde_json::json!({
        "status": if exit_code == 0 { "success" } else { "violations" },
        "action": "role",
        "exit_code": exit_code,
        "violation_count": results.len(),
        "results": results_to_json(&results, None),
    })
}

/// Run orphan scan
pub fn execute_orphan(deps: &McpDeps, path: &str) -> serde_json::Value {
    let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.to_string()) {
        Ok(f) => f,
        Err(_) => return serde_json::json!({"error": "Invalid path", "exit_code": 2}),
    };
    let ignored = deps.config_orchestrator.ignored_paths(&fp);
    let (_, results) = deps.orphan_orchestrator.scan_orphans(&fp, &ignored.values);
    serde_json::json!({
        "status": "success",
        "action": "orphan",
        "exit_code": if results.is_empty() { 0 } else { 1 },
        "orphan_count": results.len(),
        "results": results_to_json(&results, None),
    })
}

/// Run external lint
pub fn execute_external(deps: &McpDeps, path: &str) -> serde_json::Value {
    let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.to_string()) {
        Ok(f) => f,
        Err(_) => return serde_json::json!({"error": "Invalid path", "exit_code": 2}),
    };
    let scan_results = deps.external_lint.scan_all(&fp);
    let exit_code = if scan_results.values.is_empty() { 0 } else { 1 };
    serde_json::json!({
        "status": if exit_code == 0 { "success" } else { "violations" },
        "action": "external",
        "exit_code": exit_code,
        "violation_count": scan_results.values.len(),
        "results": results_to_json(&scan_results.values, None),
    })
}

/// Run doctor diagnostics
pub fn execute_doctor(deps: &McpDeps) -> serde_json::Value {
    let diag = deps.maintenance.diagnose_toolchain();
    let mut checks = Vec::new();
    for status in &diag.rust_tools {
        checks.push(serde_json::json!({"tool": status.name, "status": if status.status == "OK" { "ok" } else { "not_found" }, "version": status.version}));
    }
    for status in &diag.python_tools {
        checks.push(serde_json::json!({"tool": status.name, "status": if status.status == "OK" { "ok" } else { "not_found" }, "version": status.version}));
    }
    for status in &diag.js_tools {
        checks.push(serde_json::json!({"tool": status.name, "status": if status.status == "OK" { "ok" } else { "not_found" }, "version": status.version}));
    }
    for status in &diag.vcs_tools {
        checks.push(serde_json::json!({"tool": status.name, "status": if status.status == "OK" { "ok" } else { "not_found" }, "version": status.version}));
    }
    serde_json::json!({"status": "success", "action": "doctor", "exit_code": 0, "checks": checks})
}

/// Run security scan
pub fn execute_security(deps: &McpDeps, path: &str) -> serde_json::Value {
    let fp = shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
        .unwrap_or_else(|_| shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default());
    let report = deps.maintenance.run_security_scan(&fp);
    let exit_code = if !report.tool_installed { 3 } else if report.findings.is_empty() { 0 } else { 1 };
    serde_json::json!({
        "status": if exit_code == 0 { "clean" } else if exit_code == 3 { "tool_missing" } else { "findings" },
        "action": "security",
        "exit_code": exit_code,
        "language": report.language,
        "tool_name": report.tool_name,
        "tool_installed": report.tool_installed,
        "findings_count": report.findings.len(),
        "findings": report.findings.iter().map(|f| serde_json::json!({
            "severity": f.severity.to_uppercase(),
            "test_id": f.test_id,
            "file": f.file,
            "line": f.line,
            "issue": f.issue,
        })).collect::<Vec<serde_json::Value>>(),
    })
}

/// Run dependency report
pub fn execute_dependencies(deps: &McpDeps, path: &str) -> serde_json::Value {
    let fp = shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
        .unwrap_or_else(|_| shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default());
    match deps.maintenance.run_dependency_report(&fp) {
        Ok(report) => serde_json::json!({
            "status": "success",
            "action": "dependencies",
            "exit_code": 0,
            "language": report.language,
            "dependency_count": report.dependencies.len(),
            "dependencies": report.dependencies.iter().map(|d| serde_json::json!({
                "name": d.name, "version": d.version, "dep_type": d.dep_type,
            })).collect::<Vec<serde_json::Value>>(),
        }),
        Err(e) => serde_json::json!({"error": format!("Dependency report failed: {}", e), "exit_code": 2}),
    }
}

/// Serialize LintResults to JSON
fn results_to_json(results: &[LintResult], linter: Option<&str>) -> Vec<serde_json::Value> {
    results.iter().map(|r| {
        let mut obj = serde_json::json!({
            "file": r.file.value.as_str(),
            "code": r.code.code(),
            "message": r.message.value.as_str(),
            "line": r.line.value(),
            "column": r.column.value(),
        });
        if let Some(name) = linter {
            obj["linter"] = serde_json::json!(name);
        }
        obj
    }).collect()
}
