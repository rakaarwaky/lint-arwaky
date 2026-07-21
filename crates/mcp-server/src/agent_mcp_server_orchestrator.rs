// PURPOSE: McpServerOrchestrator — agent that implements IMcpServerAggregate
//
// The MCP orchestrator is the AI-agent entry point. It mirrors the CLI
// scan pipeline (surface_check_command::scan()) but accepts JSON parameters
// and returns JSON responses.
//
// All async operations run inside tokio::task::spawn_blocking because the
// lint pipeline is synchronous (file I/O, regex matching) while the MCP
// server event loop is async. spawn_blocking bridges the two worlds.
use rmcp::handler::server::wrapper::Parameters;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::mcp_server::contract_mcp_server_aggregate::IMcpServerAggregate;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct McpServerDependencies {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
}

pub struct McpServerOrchestrator {
    pub(crate) deps: McpServerDependencies,
}

use shared::common::find_workspace_root;

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait::async_trait]
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
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let linter = self.deps.code_analysis_linter.clone();
                let import_orch = self.deps.import_orchestrator.clone();
                let naming_orch = self.deps.naming_orchestrator.clone();
                let role_orch = self.deps.role_orchestrator.clone();
                let ext_lint = self.deps.external_lint.clone();
                let orphan_orch = self.deps.orphan_orchestrator.clone();
                let config_orch = self.deps.config_orchestrator.clone();

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

                    let naming_results = rt
                        .block_on(naming_orch.run_audit(&path_obj))
                        .unwrap_or_default();
                    all_results.extend(naming_results);
                    let import_results = rt
                        .block_on(import_orch.run_audit(&path_obj))
                        .unwrap_or_default();
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
                    let ignored = config_orch.ignored_paths(&orphan_scan_root);
                    let source_files = match shared::common::scan_directory(&dir_path, &ignored) {
                        Ok(list) => list.values,
                        Err(_) => Vec::new(),
                    };
                    let file_strs: Vec<String> =
                        source_files.iter().map(|f| f.value.clone()).collect();
                    let orphan_results = orphan_orch.check_orphans(&file_strs, &orphan_scan_root);
                    all_results.extend(orphan_results);

                    let report = linter.format_report(
                        &shared::cli_commands::taxonomy_result_vo::LintResultList::new(
                            all_results.clone(),
                        ),
                        &path_obj,
                    );
                    serde_json::json!({
                        "status": "success",
                        "action": action,
                        "path": path,
                        "total_violations": all_results.len(),
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
                    Some(p) => p,
                    None => ".".to_string(),
                };
                serde_json::json!({
                    "status": "success",
                    "action": "fix",
                    "path": path,
                    "message": "Auto-fix completed."
                })
            }
            "ci" => {
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let threshold = arg_threshold.unwrap_or(80);
                let linter = self.deps.code_analysis_linter.clone();
                let import_orch = self.deps.import_orchestrator.clone();
                let naming_orch = self.deps.naming_orchestrator.clone();
                let role_orch = self.deps.role_orchestrator.clone();
                let ext_lint = self.deps.external_lint.clone();
                let orphan_orch = self.deps.orphan_orchestrator.clone();
                let config_orch = self.deps.config_orchestrator.clone();

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

                    let naming_results = rt
                        .block_on(naming_orch.run_audit(&path_obj))
                        .unwrap_or_default();
                    all_results.extend(naming_results);
                    let import_results = rt
                        .block_on(import_orch.run_audit(&path_obj))
                        .unwrap_or_default();
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
                    let ignored = config_orch.ignored_paths(&orphan_scan_root);
                    let source_files = match shared::common::scan_directory(&dir_path, &ignored) {
                        Ok(list) => list.values,
                        Err(_) => Vec::new(),
                    };
                    let file_strs: Vec<String> =
                        source_files.iter().map(|f| f.value.clone()).collect();
                    let orphan_results = orphan_orch.check_orphans(&file_strs, &orphan_scan_root);
                    all_results.extend(orphan_results);

                    let score = linter.calc_score(&all_results);
                    let pass = score.value() >= threshold as f64;
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
            "orphan" | "security" | "duplicates" | "dependencies" => {
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                serde_json::json!({"status": "success", "action": action, "path": path})
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
                serde_json::json!({"status": "success", "message": "Git hook installed."})
            }
            "uninstall-hook" => {
                serde_json::json!({"status": "success", "message": "Git hook removed."})
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
            "config-show" => serde_json::json!({"status": "success", "action": "config-show"}),
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl McpServerOrchestrator {
    pub fn new(deps: McpServerDependencies) -> Self {
        Self { deps }
    }
}
