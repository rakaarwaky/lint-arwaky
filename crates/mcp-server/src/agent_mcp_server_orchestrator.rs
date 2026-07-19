// PURPOSE: McpServerOrchestrator — agent that implements IMcpServerAggregate
//
// The MCP orchestrator is the AI-agent entry point. It mirrors the CLI
// scan pipeline (surface_check_command::scan()) but accepts JSON parameters
// and returns JSON responses.
//
// All async operations run inside tokio::task::spawn_blocking because the
// lint pipeline is synchronous (file I/O, regex matching) while the MCP
// server event loop is async. spawn_blocking bridges the two worlds.
use crate::contract_mcp_server_aggregate::IMcpServerAggregate;
use crate::{ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs};
use rmcp::handler::server::wrapper::Parameters;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::contract_scanner_provider_port::IScannerProviderPort;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

pub fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    let mut dir = std::path::Path::new(path).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    if dir.is_file() {
        dir.pop();
    }
    loop {
        if dir.join("Cargo.toml").exists()
            || dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

// ─── Block 1: Struct Definition ───────────────────────────
pub struct McpServerDependencies {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub scanner_provider: Arc<dyn IScannerProviderPort>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    pub git_hooks_aggregate: Arc<dyn GitHooksAggregate>,
}

pub struct McpServerOrchestrator {
    pub(crate) deps: McpServerDependencies,
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl McpServerOrchestrator {
    pub fn new(deps: McpServerDependencies) -> Self {
        Self { deps }
    }
}

/// Resolve a path to an absolute path, using current_dir as fallback.
/// Returns None if the path doesn't exist.
fn resolve_path(path: &str) -> Option<String> {
    let p = std::path::Path::new(path);
    let abs = if p.is_absolute() {
        p.to_path_buf()
    } else {
        let cwd = std::env::current_dir().ok()?;
        cwd.join(p)
    };
    if abs.exists() {
        Some(abs.to_string_lossy().to_string())
    } else {
        None
    }
}

#[async_trait::async_trait]
// ─── Block 2: Public Contract ─────────────────────────────
impl IMcpServerAggregate for McpServerOrchestrator {
    async fn execute_command(&self, Parameters(args): Parameters<ExecuteCommandArgs>) -> String {
        let action = args.action.clone();
        let arg_path = args
            .args
            .as_ref()
            .and_then(|a| a.get("path"))
            .and_then(|v| v.as_str())
            .map(String::from);
        let arg_threshold = args
            .args
            .as_ref()
            .and_then(|a| a.get("threshold"))
            .and_then(|v| v.as_u64())
            .map(|n| n as u32);
        let arg_client = args
            .args
            .as_ref()
            .and_then(|a| a.get("client"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let result = match action.as_str() {
            "check" | "scan" => {
                let path = match arg_path {
                    Some(p) => match resolve_path(&p) {
                        Some(resolved) => resolved,
                        None => {
                            return serde_json::to_string_pretty(&serde_json::json!({
                                "status": "error",
                                "action": action,
                                "error": format!("Path does not exist: {}", p),
                                "hint": "Pass an absolute path to the project directory, e.g. {\"path\": \"/home/user/myproject\"}"
                            }))
                            .unwrap_or_default();
                        }
                    },
                    None => std::env::current_dir()
                        .map(|c| c.to_string_lossy().to_string())
                        .unwrap_or_else(|_| ".".to_string()),
                };
                let ext_lint = self.deps.external_lint.clone();
                let orphan_orch = self.deps.orphan_orchestrator.clone();
                let scanner = self.deps.scanner_provider.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let mut all_results = Vec::new();
                    let path_obj = FilePath::new(path.clone()).unwrap_or_default();
                    let rt = match tokio::runtime::Runtime::new() {
                        Ok(r) => r,
                        Err(_) => {
                            return serde_json::json!({"error": "failed to create runtime"});
                        }
                    };

                    let config_container = config_system::root_config_system_container::ConfigContainer::new();
                    let config_orchestrator = config_container.orchestrator();
                    let config_result = rt.block_on(config_orchestrator.load_project_config(&path_obj));
                    let loaded_config = config_result.config;

                    let import_container =
                        import_rules::root_import_rules_container::ImportContainer::new_with_config(
                            loaded_config.clone(),
                        );
                    let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
                        import_container.analyzer(),
                    );
                    let role_container =
                        role_rules::root_role_rules_container::RoleContainer::new_with_config(loaded_config.clone());
                    let analyzer = import_container.analyzer();
                    let arch_linter =
                        code_analysis::root_code_analysis_container::CodeAnalysisContainer::new_with_analyzer(
                            analyzer.clone(),
                        )
                        .code_analysis_linter();

                    let layer_det: Arc<dyn ILayerDetectionAggregate> =
                        Arc::new(import_rules::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer::new(
                            loaded_config.clone(),
                        ));

                    let aes_results = arch_linter.run_code_analysis_path(&path_obj);
                    all_results.extend(aes_results);

                    let naming_results = rt.block_on(naming_container.orchestrator().run_audit(&path_obj));
                    all_results.extend(naming_results);
                    let import_results = rt.block_on(import_container.orchestrator().run_audit(&path_obj));
                    all_results.extend(import_results);
                    let external_results = rt.block_on(ext_lint.scan_all(&path_obj));
                    all_results.extend(external_results.values);
                    let role_results = rt.block_on(role_container.orchestrator().run_audit(&path_obj));
                    all_results.extend(role_results);

                    let scan_root = find_workspace_root(&path);
                    let orphan_scan_root = match scan_root.as_ref().and_then(|r| r.to_str()) {
                        Some(s) => s.to_string(),
                        None => ".".to_string(),
                    };
                    let dir_path = DirectoryPath::new(orphan_scan_root.clone()).unwrap_or_default();
                    let source_files = match scanner.scan_directory(&dir_path) {
                        Ok(list) => list.values,
                        Err(_) => Vec::new(),
                    };
                    let file_paths: Vec<FilePath> = source_files;
                    let orphan_scan_root_fp = FilePath::new(orphan_scan_root).unwrap_or_default();
                    let orphan_results = orphan_orch.check_orphans(
                        layer_det.as_ref(),
                        &file_paths,
                        &orphan_scan_root_fp,
                    );
                    all_results.extend(orphan_results);

                    let final_results: Vec<_> = all_results
                        .into_iter()
                        .filter(|r| {
                            !loaded_config.ignored_rules.values.contains(&r.code.to_string())
                        })
                        .collect();

                    let report = arch_linter.format_report(
                        &shared::cli_commands::taxonomy_result_vo::LintResultList::new(
                            final_results.clone(),
                        ),
                        &path_obj,
                    );
                    serde_json::json!({
                        "status": "success",
                        "action": action,
                        "path": path,
                        "total_violations": final_results.len(),
                        "report": report
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "fix" => {
                let path = match arg_path {
                    Some(p) => match resolve_path(&p) {
                        Some(resolved) => resolved,
                        None => {
                            return serde_json::to_string_pretty(&serde_json::json!({
                                "status": "error",
                                "action": "fix",
                                "error": format!("Path does not exist: {}", p),
                                "hint": "Pass an absolute path to the project directory"
                            }))
                            .unwrap_or_default();
                        }
                    },
                    None => std::env::current_dir()
                        .map(|c| c.to_string_lossy().to_string())
                        .unwrap_or_else(|_| ".".to_string()),
                };
                let dry_run = args
                    .args
                    .as_ref()
                    .and_then(|a| a.get("dry_run"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let linter = self.deps.code_analysis_linter.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let path_obj = FilePath::new(path.clone()).unwrap_or_default();
                    let results_before = linter.run_code_analysis(&path_obj);
                    let violations_before = results_before.len();

                    let auto_fix_container =
                        auto_fix::root_auto_fix_container::AutoFixContainer::new(linter.clone());
                    let fix_orch = auto_fix_container.orchestrator(dry_run);
                    let fix_result = fix_orch.execute(&path_obj);

                    let violations_after = if dry_run {
                        violations_before
                    } else {
                        let after = linter.run_code_analysis(&path_obj);
                        after.len()
                    };
                    let fixed_count = violations_before.saturating_sub(violations_after);

                    serde_json::json!({
                        "status": "success",
                        "action": "fix",
                        "path": path,
                        "dry_run": dry_run,
                        "violations_before": violations_before,
                        "violations_after": violations_after,
                        "fixed": fixed_count,
                        "output": fix_result.output.value
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "ci" => {
                let path = match arg_path {
                    Some(p) => match resolve_path(&p) {
                        Some(resolved) => resolved,
                        None => {
                            return serde_json::to_string_pretty(&serde_json::json!({
                                "status": "error",
                                "action": "ci",
                                "error": format!("Path does not exist: {}", p),
                                "hint": "Pass an absolute path to the project directory"
                            }))
                            .unwrap_or_default();
                        }
                    },
                    None => std::env::current_dir()
                        .map(|c| c.to_string_lossy().to_string())
                        .unwrap_or_else(|_| ".".to_string()),
                };
                let threshold = arg_threshold.unwrap_or(80);
                let linter = self.deps.code_analysis_linter.clone();
                let import_orch = self.deps.import_orchestrator.clone();
                let naming_orch = self.deps.naming_orchestrator.clone();
                let role_orch = self.deps.role_orchestrator.clone();
                let ext_lint = self.deps.external_lint.clone();
                let orphan_orch = self.deps.orphan_orchestrator.clone();
                let layer_det = self.deps.layer_detector.clone();
                let scanner = self.deps.scanner_provider.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let mut all_results = Vec::new();
                    let path_obj = FilePath::new(path.clone()).unwrap_or_default();
                    let aes_results = linter.run_code_analysis_path(&path_obj);
                    all_results.extend(aes_results);

                    let rt = match tokio::runtime::Runtime::new() {
                        Ok(r) => r,
                        Err(_) => {
                            return serde_json::json!({"error": "failed to create runtime"});
                        }
                    };

                    let naming_results = rt.block_on(naming_orch.run_audit(&path_obj));
                    all_results.extend(naming_results);
                    let import_results = rt.block_on(import_orch.run_audit(&path_obj));
                    all_results.extend(import_results);
                    let external_results = rt.block_on(ext_lint.scan_all(&path_obj));
                    all_results.extend(external_results.values);
                    let role_results = rt.block_on(role_orch.run_audit(&path_obj));
                    all_results.extend(role_results);

                    let scan_root = find_workspace_root(&path);
                    let orphan_scan_root = match scan_root.as_ref().and_then(|r| r.to_str()) {
                        Some(s) => s.to_string(),
                        None => ".".to_string(),
                    };
                    let dir_path = DirectoryPath::new(orphan_scan_root.clone()).unwrap_or_default();
                    let source_files = match scanner.scan_directory(&dir_path) {
                        Ok(list) => list.values,
                        Err(_) => Vec::new(),
                    };
                    let file_paths: Vec<FilePath> = source_files;
                    let orphan_scan_root_fp = FilePath::new(orphan_scan_root).unwrap_or_default();
                    let orphan_results = orphan_orch.check_orphans(
                        layer_det.as_ref(),
                        &file_paths,
                        &orphan_scan_root_fp,
                    );
                    all_results.extend(orphan_results);

                    let score = linter.calc_score(&all_results);
                    let pass = score >= threshold as f64;
                    serde_json::json!({
                        "status": if pass { "pass" } else { "fail" },
                        "action": "ci",
                        "score": score,
                        "threshold": threshold,
                        "violations": all_results.len()
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "doctor" => {
                let mut checks = Vec::new();
                for tool in &["cargo", "python3", "ruff", "mypy", "bandit", "node", "git"] {
                    let found = match std::process::Command::new("which").arg(tool).output() {
                        Ok(o) => o.status.success(),
                        Err(_) => false,
                    };
                    checks.push(serde_json::json!({
                        "tool": tool,
                        "status": if found { "ok" } else { "not_found" }
                    }));
                }
                serde_json::json!({"status": "success", "action": "doctor", "checks": checks})
            }
            "orphan" => {
                let path = match arg_path {
                    Some(p) => match resolve_path(&p) {
                        Some(resolved) => resolved,
                        None => {
                            return serde_json::to_string_pretty(&serde_json::json!({
                                "status": "error",
                                "action": "orphan",
                                "error": format!("Path does not exist: {}", p),
                                "hint": "Pass an absolute path to the project directory"
                            }))
                            .unwrap_or_default();
                        }
                    },
                    None => std::env::current_dir()
                        .map(|c| c.to_string_lossy().to_string())
                        .unwrap_or_else(|_| ".".to_string()),
                };
                let orphan_orch = self.deps.orphan_orchestrator.clone();
                let layer_det = self.deps.layer_detector.clone();
                let scanner = self.deps.scanner_provider.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let scan_root = find_workspace_root(&path);
                    let orphan_scan_root = match scan_root.as_ref().and_then(|r| r.to_str()) {
                        Some(s) => s.to_string(),
                        None => ".".to_string(),
                    };
                    let dir_path = DirectoryPath::new(orphan_scan_root.clone()).unwrap_or_default();
                    let source_files = match scanner.scan_directory(&dir_path) {
                        Ok(list) => list.values,
                        Err(_) => Vec::new(),
                    };
                    let file_paths: Vec<FilePath> = source_files;
                    let orphan_scan_root_fp = FilePath::new(orphan_scan_root).unwrap_or_default();
                    let orphan_results = orphan_orch.check_orphans(
                        layer_det.as_ref(),
                        &file_paths,
                        &orphan_scan_root_fp,
                    );

                    let path_canonical = std::path::Path::new(&path)
                        .canonicalize()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| path.clone());

                    let filtered: Vec<_> = orphan_results
                        .into_iter()
                        .filter(|r| {
                            let f = &r.file.value;
                            let f_canonical = std::path::Path::new(f)
                                .canonicalize()
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_else(|_| f.clone());
                            f_canonical == path_canonical
                        })
                        .collect();

                    let is_orphan = !filtered.is_empty();
                    let report: Vec<_> = filtered
                        .iter()
                        .map(|v| {
                            serde_json::json!({
                                "code": v.code.code(),
                                "message": v.message.value,
                                "file": v.file.value,
                                "line": v.line.value,
                            })
                        })
                        .collect();

                    serde_json::json!({
                        "status": "success",
                        "action": "orphan",
                        "path": path,
                        "is_orphan": is_orphan,
                        "violations": report.len(),
                        "details": report
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "security" => {
                let path = match arg_path {
                    Some(p) => match resolve_path(&p) {
                        Some(resolved) => resolved,
                        None => {
                            return serde_json::to_string_pretty(&serde_json::json!({
                                "status": "error",
                                "action": "security",
                                "error": format!("Path does not exist: {}", p),
                                "hint": "Pass an absolute path to the project directory"
                            }))
                            .unwrap_or_default();
                        }
                    },
                    None => std::env::current_dir()
                        .map(|c| c.to_string_lossy().to_string())
                        .unwrap_or_else(|_| ".".to_string()),
                };
                let maint = self.deps.maintenance_orchestrator.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let rt = match tokio::runtime::Runtime::new() {
                        Ok(r) => r,
                        Err(_) => {
                            return serde_json::json!({"error": "failed to create runtime"});
                        }
                    };
                    let path_obj = FilePath::new(path.clone()).unwrap_or_default();
                    let report = rt.block_on(maint.run_security_scan(&path_obj));

                    let findings: Vec<_> = report
                        .findings
                        .iter()
                        .map(|f| {
                            serde_json::json!({
                                "severity": f.severity,
                                "test_id": f.test_id,
                                "file": f.file,
                                "line": f.line,
                                "issue": f.issue,
                            })
                        })
                        .collect();

                    serde_json::json!({
                        "status": "success",
                        "action": "security",
                        "path": path,
                        "language": report.language,
                        "tool": report.tool_name,
                        "tool_installed": report.tool_installed,
                        "findings_count": findings.len(),
                        "findings": findings
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "duplicates" => {
                let path = match arg_path {
                    Some(p) => match resolve_path(&p) {
                        Some(resolved) => resolved,
                        None => {
                            return serde_json::to_string_pretty(&serde_json::json!({
                                "status": "error",
                                "action": "duplicates",
                                "error": format!("Path does not exist: {}", p),
                                "hint": "Pass an absolute path to the project directory"
                            }))
                            .unwrap_or_default();
                        }
                    },
                    None => std::env::current_dir()
                        .map(|c| c.to_string_lossy().to_string())
                        .unwrap_or_else(|_| ".".to_string()),
                };

                let join_result = tokio::task::spawn_blocking(move || {
                    let dup_analyzer = code_analysis::CodeDuplicationAnalyzer::new();
                    let fs_adapter =
                        import_rules::infrastructure_filesystem_adapter::OSFileSystemAdapter::new();
                    let violations = dup_analyzer.handle_duplicates(
                        Some(FilePath::new(path.clone()).unwrap_or_default()),
                        &fs_adapter,
                    );

                    let findings: Vec<_> = violations
                        .iter()
                        .map(|v| {
                            let s: String = v.clone().into();
                            serde_json::json!({
                                "issue": s,
                            })
                        })
                        .collect();

                    serde_json::json!({
                        "status": "success",
                        "action": "duplicates",
                        "path": path,
                        "duplicates_found": findings.len(),
                        "details": findings
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "dependencies" => {
                let path = match arg_path {
                    Some(p) => match resolve_path(&p) {
                        Some(resolved) => resolved,
                        None => {
                            return serde_json::to_string_pretty(&serde_json::json!({
                                "status": "error",
                                "action": "dependencies",
                                "error": format!("Path does not exist: {}", p),
                                "hint": "Pass an absolute path to the project directory"
                            }))
                            .unwrap_or_default();
                        }
                    },
                    None => std::env::current_dir()
                        .map(|c| c.to_string_lossy().to_string())
                        .unwrap_or_else(|_| ".".to_string()),
                };
                let maint = self.deps.maintenance_orchestrator.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let rt = match tokio::runtime::Runtime::new() {
                        Ok(r) => r,
                        Err(_) => {
                            return serde_json::json!({"error": "failed to create runtime"});
                        }
                    };
                    let path_obj = FilePath::new(path.clone()).unwrap_or_default();
                    match rt.block_on(maint.run_dependency_report(&path_obj)) {
                        Ok(report) => {
                            let deps: Vec<_> = report
                                .dependencies
                                .iter()
                                .take(50)
                                .map(|d| {
                                    serde_json::json!({
                                        "name": d.name,
                                        "version": d.version,
                                        "dep_type": d.dep_type,
                                    })
                                })
                                .collect();
                            serde_json::json!({
                                "status": "success",
                                "action": "dependencies",
                                "path": path,
                                "language": report.language,
                                "total": report.dependencies.len(),
                                "dependencies": deps
                            })
                        }
                        Err(e) => serde_json::json!({
                            "status": "error",
                            "action": "dependencies",
                            "path": path,
                            "error": e
                        }),
                    }
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "version" => {
                serde_json::json!({"version": env!("CARGO_PKG_VERSION"), "name": "lint-arwaky"})
            }
            "adapters" => {
                let ext = self.deps.external_lint.clone();
                let adapter_names = ext.adapter_names();
                let mut adapters = Vec::new();
                for name in &adapter_names {
                    let found = match std::process::Command::new("which").arg(name).output() {
                        Ok(o) => o.status.success(),
                        Err(_) => false,
                    };
                    adapters.push(serde_json::json!({"name": name, "enabled": found}));
                }
                serde_json::json!({"adapters": adapters})
            }
            "install-hook" => {
                let git_hooks = self.deps.git_hooks_aggregate.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let rt = match tokio::runtime::Runtime::new() {
                        Ok(r) => r,
                        Err(_) => {
                            return serde_json::json!({"error": "failed to create runtime"});
                        }
                    };
                    let exe_path =
                        shared::common::taxonomy_path_vo::FilePath::new("lint-arwaky".to_string())
                            .unwrap_or_default();
                    match rt.block_on(git_hooks.install_hook(&exe_path)) {
                        Ok(status) if status.value => {
                            serde_json::json!({
                                "status": "success",
                                "message": "Git pre-commit hook installed successfully."
                            })
                        }
                        Ok(_) => {
                            serde_json::json!({
                                "status": "success",
                                "message": "Not a git repository, hook installation skipped."
                            })
                        }
                        Err(e) => {
                            serde_json::json!({
                                "status": "error",
                                "message": format!("Failed to install hook: {:?}", e)
                            })
                        }
                    }
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "uninstall-hook" => {
                let git_hooks = self.deps.git_hooks_aggregate.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let rt = match tokio::runtime::Runtime::new() {
                        Ok(r) => r,
                        Err(_) => {
                            return serde_json::json!({"error": "failed to create runtime"});
                        }
                    };
                    match rt.block_on(git_hooks.uninstall_hook()) {
                        Ok(status) if status.value => {
                            serde_json::json!({
                                "status": "success",
                                "message": "Git pre-commit hook removed successfully."
                            })
                        }
                        Ok(_) => {
                            serde_json::json!({
                                "status": "success",
                                "message": "Not a git repository, hook removal skipped."
                            })
                        }
                        Err(e) => {
                            serde_json::json!({
                                "status": "error",
                                "message": format!("Failed to remove hook: {:?}", e)
                            })
                        }
                    }
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "init" => serde_json::json!({"status": "success", "action": "init"}),
            "install" => serde_json::json!({"status": "success", "action": "install"}),
            "mcp-config" => {
                let client = match arg_client {
                    Some(c) => c,
                    None => "all".to_string(),
                };
                serde_json::json!({"status": "success", "action": "mcp-config", "client": client})
            }
            "config-show" => {
                let join_result = tokio::task::spawn_blocking(move || {
                    let rt = match tokio::runtime::Runtime::new() {
                        Ok(r) => r,
                        Err(_) => {
                            return serde_json::json!({"error": "failed to create runtime"});
                        }
                    };
                    let config_container =
                        config_system::root_config_system_container::ConfigContainer::new();
                    let config_orchestrator = config_container.orchestrator();
                    let config_reader = config_orchestrator.config_reader();
                    let project_root = FilePath::new(".".to_string()).unwrap_or_default();
                    let config_files = rt.block_on(config_reader.list_config_files(&project_root));

                    if config_files.is_empty() {
                        return serde_json::json!({
                            "status": "success",
                            "action": "config-show",
                            "message": "No config files found. Run `lint-arwaky init` to create one.",
                            "config_files": []
                        });
                    }

                    let mut configs = Vec::new();
                    for (lang, path_str) in &config_files {
                        if let Some(source) = rt.block_on(config_reader.read_config(&project_root, lang)) {
                            configs.push(serde_json::json!({
                                "language": lang,
                                "path": path_str,
                                "content": source.raw_content,
                            }));
                        }
                    }

                    serde_json::json!({
                        "status": "success",
                        "action": "config-show",
                        "config_files": configs
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            _ => serde_json::json!({"error": format!("Unknown action: {}", action)}),
        };
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }

    async fn list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        let catalog = shared::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
        let commands: Vec<serde_json::Value> = catalog
            .iter()
            .filter(|(name, _desc, _ex)| match args.domain.as_deref() {
                Some(d) if !d.is_empty() => name.contains(d),
                _ => true,
            })
            .map(|(name, desc, example)| {
                serde_json::json!({
                    "name": name,
                    "description": desc,
                    "example": example
                })
            })
            .collect();
        let result = serde_json::json!({ "commands": commands, "total": commands.len() });
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }

    async fn read_skill(&self, Parameters(args): Parameters<ReadSkillArgs>) -> String {
        let mut candidates = vec![
            env!("CARGO_MANIFEST_DIR").to_string() + "/../SKILL.md",
            env!("CARGO_MANIFEST_DIR").to_string() + "/SKILL.md",
            "SKILL.md".to_string(),
            "./SKILL.md".to_string(),
        ];
        // XDG config fallback: ~/.config/lint-arwaky/SKILL.md
        if let Some(config_dir) = dirs::config_dir() {
            let xdg_skill = config_dir.join("lint-arwaky").join("SKILL.md");
            candidates.push(xdg_skill.to_string_lossy().to_string());
        }
        let content = candidates
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.exists())
            .and_then(|p| std::fs::read_to_string(p).ok());
        let content = match content {
            Some(c) => c,
            None => {
                return serde_json::json!({
                    "error": "SKILL.md not found",
                    "searched": candidates
                })
                .to_string();
            }
        };
        match args.section.as_deref() {
            Some(s) if !s.is_empty() => {
                let header = format!("## {}", s);
                if let Some(start) = content.find(&header) {
                    let remaining = &content[start..];
                    let end = match remaining[1..].find("\n## ") {
                        Some(i) => i + 1,
                        None => remaining.len(),
                    };
                    serde_json::json!({"section": s, "content": &remaining[..end]}).to_string()
                } else {
                    serde_json::json!({"error": format!("Section '{}' not found", s)}).to_string()
                }
            }
            _ => serde_json::json!({"content": content}).to_string(),
        }
    }
}
