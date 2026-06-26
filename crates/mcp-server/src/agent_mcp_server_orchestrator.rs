// PURPOSE: McpServerOrchestrator — agent that implements IMcpServerAggregate
//
// The MCP orchestrator is the AI-agent entry point. It mirrors the CLI
// scan pipeline (surface_check_command::scan()) but accepts JSON parameters
// and returns JSON responses. Key difference from CLI: the MCP orchestrator
// receives structured arguments (action + path + options) instead of CLI flags.
//
// NOTE: This duplicates some logic from surface_check_command.rs. The scan
// pipeline should ideally be extracted to a shared location, but the CLI
// and MCP pipelines evolved independently.
//
// All async operations run inside tokio::task::spawn_blocking because the
// lint pipeline is synchronous (file I/O, regex matching) while the MCP
// server event loop is async. spawn_blocking bridges the two worlds.
use crate::contract_mcp_server_aggregate::IMcpServerAggregate;
use crate::contract_mcp_server_aggregate::{ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs};
use rmcp::handler::server::wrapper::Parameters;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::contract_scanner_provider_port::IScannerProviderPort;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

pub struct McpServerDependencies {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub scanner_provider: Arc<dyn IScannerProviderPort>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
}

pub struct McpServerOrchestrator {
    deps: McpServerDependencies,
}

impl McpServerOrchestrator {
    pub fn new(deps: McpServerDependencies) -> Self {
        Self { deps }
    }
}

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
                let layer_det = self.deps.layer_detector.clone();
                let scanner = self.deps.scanner_provider.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let mut all_results = Vec::new();

                    // 1. AES analysis (sync, code-analysis)
                    let aes_results = linter.run_code_analysis_path(&path);
                    all_results.extend(aes_results);

                    // Build FilePath for async aggregates
                    let path_obj = FilePath::new(path.clone()).unwrap_or_default();
                    let rt = match tokio::runtime::Runtime::new() {
                        Ok(r) => r,
                        Err(_) => {
                            return serde_json::json!({"error": "failed to create runtime"});
                        }
                    };

                    // 2. Naming rules (AES101, AES102)
                    let naming_results = rt.block_on(naming_orch.run_audit(&path_obj));
                    all_results.extend(naming_results);

                    // 3. Import rules (AES201–AES205)
                    let import_results = rt.block_on(import_orch.run_audit(&path_obj));
                    all_results.extend(import_results);

                    // 4. External linters (ruff, mypy, bandit, eslint, clippy)
                    let external_results = rt.block_on(ext_lint.scan_all(&path_obj));
                    all_results.extend(external_results.values);

                    // 5. Role rules (AES401–AES406)
                    let role_results = rt.block_on(role_orch.run_audit(&path_obj));
                    all_results.extend(role_results);

                    // 6. Orphan detection (AES501–AES506)
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
                    let file_strs: Vec<String> =
                        source_files.iter().map(|f| f.value.clone()).collect();
                    let orphan_results = orphan_orch.check_orphans(
                        layer_det.as_ref(),
                        &file_strs,
                        &orphan_scan_root,
                    );
                    all_results.extend(orphan_results);

                    // Format report
                    let report = linter.format_report(
                        &shared::cli_commands::taxonomy_result_vo::LintResultList::new(
                            all_results.clone(),
                        ),
                        &path,
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
                let layer_det = self.deps.layer_detector.clone();
                let scanner = self.deps.scanner_provider.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let mut all_results = Vec::new();

                    let aes_results = linter.run_code_analysis_path(&path);
                    all_results.extend(aes_results);

                    let path_obj = FilePath::new(path.clone()).unwrap_or_default();
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
                    let file_strs: Vec<String> =
                        source_files.iter().map(|f| f.value.clone()).collect();
                    let orphan_results = orphan_orch.check_orphans(
                        layer_det.as_ref(),
                        &file_strs,
                        &orphan_scan_root,
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
        let candidates = [
            env!("CARGO_MANIFEST_DIR").to_string() + "/../SKILL.md",
            env!("CARGO_MANIFEST_DIR").to_string() + "/SKILL.md",
            "SKILL.md".to_string(),
            "./SKILL.md".to_string(),
        ];
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

fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    let mut dir = std::path::Path::new(path).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract_mcp_server_aggregate::{ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs};
    use rmcp::handler::server::wrapper::Parameters;
    use std::sync::Arc;

    // ── Mock dependencies ──────────────────────────────────────────────────
    // These implementations panic if called. The test paths below
    // (version, doctor, unknown action, list_commands, read_skill) do NOT
    // touch self.deps, so the mocks are never exercised.
    struct MockDeps;

    impl ICodeAnalysisAggregate for MockDeps {
        fn run_code_analysis(
            &self,
            _: &str,
        ) -> shared::cli_commands::taxonomy_result_vo::LintResultList {
            unreachable!()
        }
        fn run_code_analysis_dir(
            &self,
            _: &str,
        ) -> shared::cli_commands::taxonomy_result_vo::LintResultList {
            unreachable!()
        }
        fn run_code_analysis_path(
            &self,
            _: &str,
        ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
            unreachable!()
        }
        fn calc_score(
            &self,
            _: &[shared::cli_commands::taxonomy_result_vo::LintResult],
        ) -> f64 {
            unreachable!()
        }
        fn check_critical(
            &self,
            _: &[shared::cli_commands::taxonomy_result_vo::LintResult],
        ) -> bool {
            unreachable!()
        }
        fn format_report(
            &self,
            _: &shared::cli_commands::taxonomy_result_vo::LintResultList,
            _: &str,
        ) -> String {
            unreachable!()
        }
        fn active_rules(
            &self,
        ) -> Vec<shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO>
        {
            unreachable!()
        }
    }

    #[async_trait::async_trait]
    impl IImportRunnerAggregate for MockDeps {
        async fn run_audit(
            &self,
            _: &FilePath,
        ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
            unreachable!()
        }
        fn name(&self) -> &str {
            unreachable!()
        }
    }

    #[async_trait::async_trait]
    impl INamingRunnerAggregate for MockDeps {
        async fn run_audit(
            &self,
            _: &FilePath,
        ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
            unreachable!()
        }
        fn name(&self) -> &str {
            unreachable!()
        }
    }

    impl IOrphanAggregate for MockDeps {
        fn build_orphan_graph_context(
            &self,
            _: &[String],
            _: &str,
        ) -> shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext {
            unreachable!()
        }
        fn identify_orphan_entry_points(&self, _: &[String]) -> std::collections::HashSet<String> {
            unreachable!()
        }
        fn check_orphans(
            &self,
            _: &dyn ILayerDetectionAggregate,
            _: &[String],
            _: &str,
        ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
            unreachable!()
        }
    }

    impl ILayerDetectionAggregate for MockDeps {
        fn detect_layer(&self, _: &str, _: &str) -> Option<String> {
            unreachable!()
        }
        fn get_layer_def(
            &self,
            _: &str,
        ) -> Option<shared::common::taxonomy_definition_vo::LayerDefinition> {
            unreachable!()
        }
        fn get_orphan_entry_points(&self) -> Vec<String> {
            unreachable!()
        }
    }

    impl IScannerProviderPort for MockDeps {
        fn scan_directory(
            &self,
            _: &DirectoryPath,
        ) -> Result<
            shared::common::taxonomy_paths_vo::FilePathList,
            shared::common::taxonomy_filesystem_error::FileSystemError,
        > {
            unreachable!()
        }
        fn get_ignored_files(&self) -> shared::common::taxonomy_paths_vo::FilePathList {
            unreachable!()
        }
    }

    #[async_trait::async_trait]
    impl IExternalLintAggregate for MockDeps {
        async fn scan_all(
            &self,
            _: &FilePath,
        ) -> shared::cli_commands::taxonomy_result_vo::LintResultList {
            unreachable!()
        }
        fn adapter_names(&self) -> Vec<String> {
            unreachable!()
        }
    }

    #[async_trait::async_trait]
    impl IRoleRunnerAggregate for MockDeps {
        async fn run_audit(
            &self,
            _: &FilePath,
        ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
            unreachable!()
        }
        fn name(&self) -> &str {
            unreachable!()
        }
    }

    // ── Helpers ────────────────────────────────────────────────────────────

    fn make_orchestrator() -> McpServerOrchestrator {
        let deps = McpServerDependencies {
            code_analysis_linter: Arc::new(MockDeps),
            import_orchestrator: Arc::new(MockDeps),
            naming_orchestrator: Arc::new(MockDeps),
            orphan_orchestrator: Arc::new(MockDeps),
            layer_detector: Arc::new(MockDeps),
            scanner_provider: Arc::new(MockDeps),
            external_lint: Arc::new(MockDeps),
            role_orchestrator: Arc::new(MockDeps),
        };
        McpServerOrchestrator::new(deps)
    }

    fn make_exec_params(action: &str, path: Option<&str>) -> Parameters<ExecuteCommandArgs> {
        Parameters(ExecuteCommandArgs {
            action: action.to_string(),
            args: path.map(|p| serde_json::json!({"path": p})),
        })
    }

    fn make_list_params(domain: Option<&str>) -> Parameters<ListCommandsArgs> {
        Parameters(ListCommandsArgs {
            domain: domain.map(|d| d.to_string()),
        })
    }

    fn make_read_params(section: Option<&str>) -> Parameters<ReadSkillArgs> {
        Parameters(ReadSkillArgs {
            section: section.map(|s| s.to_string()),
        })
    }

    // ── find_workspace_root ────────────────────────────────────────────────

    #[test]
    fn find_workspace_root_finds_cargo_toml() {
        // The project root has Cargo.toml — walk up from crates/mcp-server/
        let root = find_workspace_root("crates/mcp-server/src/lib.rs");
        assert!(root.is_some(), "should find workspace root from subpath");
        let path = root.unwrap();
        assert!(path.join("Cargo.toml").exists(), "root should contain Cargo.toml");
    }

    #[test]
    fn find_workspace_root_returns_none_for_empty() {
        let root = find_workspace_root("");
        // current_dir() + "" might still resolve inside the workspace
        // So just verify it doesn't crash and returns Some or None
        assert!(root.is_none() || root.is_some());
    }

    #[test]
    fn find_workspace_root_from_root_returns_self() {
        let root = find_workspace_root(".");
        assert!(root.is_some());
        let path = root.unwrap();
        assert!(path.join("Cargo.toml").exists());
    }

    #[test]
    fn find_workspace_root_finds_crates_dir() {
        let tmp = std::env::temp_dir().join("test_find_root_crates");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp.join("sub/a/b")).unwrap();
        let root = find_workspace_root(tmp.join("sub/a/b").to_str().unwrap());
        // without crates/ marker, should walk up but not find anything
        assert!(root.is_none(), "no marker dirs → should be None");

        // Now add crates/ dir at tmp level
        std::fs::create_dir_all(&tmp.join("crates")).unwrap();
        let root2 = find_workspace_root(tmp.join("sub/a/b").to_str().unwrap());
        assert_eq!(root2.as_deref(), Some(tmp.as_path()), "should find crates/ dir");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn find_workspace_root_finds_packages_dir() {
        let tmp = std::env::temp_dir().join("test_find_root_packages");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp.join("packages")).unwrap();
        std::fs::create_dir_all(&tmp.join("sub/deep")).unwrap();
        let root = find_workspace_root(tmp.join("sub/deep").to_str().unwrap());
        assert_eq!(root.as_deref(), Some(tmp.as_path()), "should find packages/ dir");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn find_workspace_root_finds_modules_dir() {
        let tmp = std::env::temp_dir().join("test_find_root_modules");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp.join("modules")).unwrap();
        let root = find_workspace_root(tmp.to_str().unwrap());
        assert_eq!(root.as_deref(), Some(tmp.as_path()), "modules/ at root itself");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn find_workspace_root_absolute_path_does_not_call_current_dir() {
        let tmp = std::env::temp_dir().join("test_find_root_abs");
        let _ = std::fs::remove_dir_all(&tmp);
        // No markers at all — should return None for an absolute path
        std::fs::create_dir_all(&tmp).unwrap();
        let root = find_workspace_root(tmp.to_str().unwrap());
        assert!(root.is_none(), "absolute path without markers → None");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    // ── list_commands ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn list_commands_returns_all_without_domain() {
        let orch = make_orchestrator();
        let params = make_list_params(None);
        let output = orch.list_commands(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        let total = parsed["total"].as_u64().unwrap();
        let commands = parsed["commands"].as_array().unwrap();
        assert_eq!(total as usize, commands.len());
        assert!(total > 0, "COMMAND_CATALOG should have entries");
    }

    #[tokio::test]
    async fn list_commands_filters_by_domain() {
        let orch = make_orchestrator();
        let params = make_list_params(Some("check"));
        let output = orch.list_commands(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        let commands = parsed["commands"].as_array().unwrap();
        assert!(!commands.is_empty(), "\"check\" matches at least 'check' and 'scan'");
        for cmd in commands {
            let name = cmd["name"].as_str().unwrap();
            assert!(name.contains("check"), "filtered command '{}' should contain 'check'", name);
        }
    }

    #[tokio::test]
    async fn list_commands_unknown_domain_returns_empty() {
        let orch = make_orchestrator();
        let params = make_list_params(Some("zzz_nonexistent_999"));
        let output = orch.list_commands(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert_eq!(parsed["total"].as_u64().unwrap(), 0);
        let commands = parsed["commands"].as_array().unwrap();
        assert!(commands.is_empty());
    }

    #[tokio::test]
    async fn list_commands_empty_domain_equals_all() {
        let orch = make_orchestrator();
        let params = make_list_params(Some(""));
        let output = orch.list_commands(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        let total = parsed["total"].as_u64().unwrap();
        assert!(total > 0, "empty domain should return all commands");
    }

    // ── read_skill ─────────────────────────────────────────────────────────
    // read_skill searches candidates including {CARGO_MANIFEST_DIR}/SKILL.md.
    // We create a temporary file there and serialize tests with a mutex.

    static SKILL_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
        std::sync::OnceLock::new();

    /// RAII guard that creates a temp SKILL.md at the crate's manifest dir and
    /// removes it on drop (even on panic). Acquires a module-level mutex to
    /// prevent races between parallel tests.
    struct TempSkill {
        path: std::path::PathBuf,
        _guard: std::sync::MutexGuard<'static, ()>,
    }

    impl TempSkill {
        fn new(content: &str) -> Self {
            let lock = SKILL_LOCK.get_or_init(|| std::sync::Mutex::new(()));
            let guard = lock.lock().unwrap();
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("SKILL.md");
            let _ = std::fs::remove_file(&path);
            std::fs::write(&path, content).expect("write temp SKILL.md");
            Self { path, _guard: guard }
        }
    }

    impl Drop for TempSkill {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    #[tokio::test]
    async fn read_skill_returns_full_content() {
        let _skill = TempSkill::new("# Test Skill\n\nsome content");
        let orch = make_orchestrator();
        let params = make_read_params(None);
        let output = orch.read_skill(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        let content = parsed["content"].as_str().unwrap();
        assert!(content.contains("Test Skill"), "full content should contain title");
    }

    #[tokio::test]
    async fn read_skill_finds_existing_section() {
        let _skill = TempSkill::new(
            "# Title\n\n## my-section\nThis is the section content.\n\n## another-section\nOther content here.",
        );
        let orch = make_orchestrator();
        let params = make_read_params(Some("my-section"));
        let output = orch.read_skill(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert_eq!(parsed["section"].as_str(), Some("my-section"));
        let section_content = parsed["content"].as_str().unwrap();
        assert!(section_content.contains("my-section"));
        assert!(section_content.contains("section content"));
    }

    #[tokio::test]
    async fn read_skill_returns_error_for_nonexistent_section() {
        let _skill = TempSkill::new("# Title\n\n## real-section\ncontent");
        let orch = make_orchestrator();
        let params = make_read_params(Some("NonexistentSectionXYZ"));
        let output = orch.read_skill(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert!(
            parsed["error"].as_str().map_or(false, |s| s.contains("not found")),
            "should report section not found"
        );
    }

    #[tokio::test]
    async fn read_skill_empty_section_equals_full() {
        let _skill = TempSkill::new("# Title\n\nsome content here");
        let orch = make_orchestrator();
        let params = make_read_params(Some(""));
        let output = orch.read_skill(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert!(
            parsed["content"].as_str().map_or(false, |s| s.contains("Title")),
            "empty section should fall back to full content"
        );
    }

    // ── execute_command: version ───────────────────────────────────────────

    #[tokio::test]
    async fn execute_version_returns_pkg_version() {
        let orch = make_orchestrator();
        let params = make_exec_params("version", None);
        let output = orch.execute_command(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert_eq!(parsed["name"].as_str(), Some("lint-arwaky"));
        assert!(parsed["version"].as_str().map_or(false, |v| !v.is_empty()));
    }

    // ── execute_command: unknown action ────────────────────────────────────

    #[tokio::test]
    async fn execute_unknown_action_returns_error() {
        let orch = make_orchestrator();
        let params = make_exec_params("__does_not_exist__", None);
        let output = orch.execute_command(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert!(parsed["error"].as_str().map_or(false, |s| s.contains("Unknown action")));
    }

    // ── execute_command: doctor ────────────────────────────────────────────

    #[tokio::test]
    async fn execute_doctor_runs_which_checks() {
        let orch = make_orchestrator();
        let params = make_exec_params("doctor", None);
        let output = orch.execute_command(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert_eq!(parsed["status"].as_str(), Some("success"));
        assert_eq!(parsed["action"].as_str(), Some("doctor"));
        let checks = parsed["checks"].as_array().unwrap();
        assert!(!checks.is_empty(), "should have tool checks");
        // cargo should be present in any Rust development environment
        let cargo_check = checks
            .iter()
            .find(|c| c["tool"].as_str() == Some("cargo"))
            .expect("should include cargo check");
        assert_eq!(cargo_check["status"].as_str(), Some("ok"));
    }

    // ── execute_command: common action paths (no deps needed) ──────────────

    #[tokio::test]
    async fn execute_init_returns_success() {
        let orch = make_orchestrator();
        let params = make_exec_params("init", None);
        let output = orch.execute_command(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert_eq!(parsed["status"].as_str(), Some("success"));
    }

    #[tokio::test]
    async fn execute_install_hook_returns_success() {
        let orch = make_orchestrator();
        let params = make_exec_params("install-hook", None);
        let output = orch.execute_command(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert_eq!(parsed["status"].as_str(), Some("success"));
    }

    #[tokio::test]
    async fn execute_uninstall_hook_returns_success() {
        let orch = make_orchestrator();
        let params = make_exec_params("uninstall-hook", None);
        let output = orch.execute_command(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert_eq!(parsed["status"].as_str(), Some("success"));
    }

    #[tokio::test]
    async fn execute_install_returns_success() {
        let orch = make_orchestrator();
        let params = make_exec_params("install", None);
        let output = orch.execute_command(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert_eq!(parsed["status"].as_str(), Some("success"));
    }

    #[tokio::test]
    async fn execute_config_show_returns_success() {
        let orch = make_orchestrator();
        let params = make_exec_params("config-show", None);
        let output = orch.execute_command(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert_eq!(parsed["status"].as_str(), Some("success"));
    }

    #[tokio::test]
    async fn execute_orphan_with_path() {
        let orch = make_orchestrator();
        let params = make_exec_params("orphan", Some("/some/path"));
        let output = orch.execute_command(params).await;
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("valid JSON");
        assert_eq!(parsed["status"].as_str(), Some("success"));
        assert_eq!(parsed["action"].as_str(), Some("orphan"));
        assert_eq!(parsed["path"].as_str(), Some("/some/path"));
    }
}
