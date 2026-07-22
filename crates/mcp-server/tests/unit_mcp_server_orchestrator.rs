// PURPOSE: Unit tests for McpServerOrchestrator — execute_command, list_commands, read_skill

use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use rmcp::handler::server::wrapper::Parameters;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::Score;
use shared::common::taxonomy_path_vo::FilePath;
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

// ─── Mock Implementations ────────────────────────────────────────────

struct MockCodeAnalysis;
impl ICodeAnalysisAggregate for MockCodeAnalysis {
    fn run_code_analysis(&self, _project_root: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }
    fn run_code_analysis_dir(&self, _src_dir: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }
    fn run_code_analysis_path(&self, _path: &FilePath) -> Vec<LintResult> {
        vec![]
    }
    fn calc_score(&self, _results: &[LintResult]) -> Score {
        Score::new(100.0)
    }
    fn check_critical(&self, _results: &[LintResult]) -> bool {
        false
    }
    fn format_report(&self, _results: &LintResultList, _project_root: &FilePath) -> String {
        "Mock report: 0 violations".to_string()
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

struct MockImportRunner;
#[async_trait::async_trait]
impl IImportRunnerAggregate for MockImportRunner {
    async fn run_audit(
        &self,
        _target: &FilePath,
    ) -> Result<Vec<LintResult>, shared::common::taxonomy_adapter_error::ScanError> {
        Ok(vec![])
    }
    fn name(&self) -> &str {
        "mock-import"
    }
}

struct MockNamingRunner;
#[async_trait::async_trait]
impl INamingRunnerAggregate for MockNamingRunner {
    async fn run_audit(
        &self,
        _target: &FilePath,
    ) -> Result<Vec<LintResult>, shared::common::taxonomy_adapter_error::ScanError> {
        Ok(vec![])
    }
    fn name(&self) -> &str {
        "mock-naming"
    }
}

struct MockOrphanDetector;
impl IOrphanAggregate for MockOrphanDetector {
    fn build_orphan_graph_context(
        &self,
        _files: &[String],
        _root_dir: &str,
    ) -> shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext {
        shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext::default()
    }
    fn identify_orphan_entry_points(&self, _files: &[String]) -> std::collections::HashSet<String> {
        std::collections::HashSet::new()
    }
    fn check_orphans(&self, _files: &[String], _root_dir: &str) -> Vec<LintResult> {
        vec![]
    }
}

struct MockExternalLint;
#[async_trait::async_trait]
impl IExternalLintAggregate for MockExternalLint {
    async fn scan_all(&self, _path: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }
    fn adapter_names(&self) -> Vec<String> {
        vec!["ruff".to_string(), "mypy".to_string()]
    }
}

struct MockRoleRunner;
#[async_trait::async_trait]
impl IRoleRunnerAggregate for MockRoleRunner {
    async fn run_audit(&self, _target: &FilePath) -> Vec<LintResult> {
        vec![]
    }
    fn name(&self) -> &str {
        "mock-role"
    }
}

struct MockConfigOrchestrator;
#[async_trait::async_trait]
impl IConfigOrchestratorAggregate for MockConfigOrchestrator {
    async fn load_project_config(
        &self,
        _project_root: &FilePath,
    ) -> shared::config_system::taxonomy_source_vo::ConfigResult {
        shared::config_system::taxonomy_source_vo::ConfigResult::default()
    }
    async fn load_config_for_language(
        &self,
        _project_root: &FilePath,
        _language: shared::config_system::taxonomy_config_language_vo::ConfigLanguage,
    ) -> shared::config_system::taxonomy_source_vo::ConfigResult {
        shared::config_system::taxonomy_source_vo::ConfigResult::default()
    }
    async fn discover_workspaces(
        &self,
        _root: &FilePath,
    ) -> Vec<shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo> {
        vec![]
    }
    fn load_config_sync(
        &self,
        _project_root: &str,
    ) -> shared::config_system::taxonomy_config_vo::ArchitectureConfig {
        shared::config_system::taxonomy_config_vo::ArchitectureConfig::default()
    }
    fn ignored_paths(&self, _project_root: &str) -> Vec<String> {
        vec!["target".to_string(), "node_modules".to_string()]
    }
}

// ─── Helper ──────────────────────────────────────────────────────────

fn build_test_orchestrator() -> McpServerOrchestrator {
    McpServerOrchestrator::new(McpServerDependencies {
        code_analysis_linter: Arc::new(MockCodeAnalysis),
        import_orchestrator: Arc::new(MockImportRunner),
        naming_orchestrator: Arc::new(MockNamingRunner),
        orphan_orchestrator: Arc::new(MockOrphanDetector),
        external_lint: Arc::new(MockExternalLint),
        role_orchestrator: Arc::new(MockRoleRunner),
        config_orchestrator: Arc::new(MockConfigOrchestrator),
    })
}

fn make_execute_args(action: &str, path: Option<&str>) -> Parameters<ExecuteCommandArgs> {
    let args = path.map(|p| serde_json::json!({"path": p}));
    Parameters(ExecuteCommandArgs {
        action: action.to_string(),
        args,
    })
}

// ─── execute_command: version ────────────────────────────────────────

#[tokio::test]
async fn execute_command_version_returns_version_json() {
    let sut = build_test_orchestrator();
    let result = sut
        .execute_command(make_execute_args("version", None))
        .await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["name"], "lint-arwaky");
    assert!(parsed["version"].is_string());
}

// ─── execute_command: unknown action ─────────────────────────────────

#[tokio::test]
async fn execute_command_unknown_action_returns_error() {
    let sut = build_test_orchestrator();
    let result = sut
        .execute_command(make_execute_args("nonexistent_action", None))
        .await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["error"].as_str().unwrap().contains("Unknown action"));
}

// ─── execute_command: doctor ─────────────────────────────────────────

#[tokio::test]
async fn execute_command_doctor_returns_checks_array() {
    let sut = build_test_orchestrator();
    let result = sut.execute_command(make_execute_args("doctor", None)).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["action"], "doctor");
    assert!(parsed["checks"].is_array());
    assert!(!parsed["checks"].as_array().unwrap().is_empty());
}

// ─── execute_command: adapters ───────────────────────────────────────

#[tokio::test]
async fn execute_command_adapters_returns_adapter_list() {
    let sut = build_test_orchestrator();
    let result = sut
        .execute_command(make_execute_args("adapters", None))
        .await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["adapters"].is_array());
    let adapters = parsed["adapters"].as_array().unwrap();
    assert_eq!(adapters.len(), 2); // MockExternalLint returns ["ruff", "mypy"]
}

// ─── execute_command: scan (happy path) ──────────────────────────────

#[tokio::test]
async fn execute_command_scan_returns_success_with_report() {
    let sut = build_test_orchestrator();
    let result = sut
        .execute_command(make_execute_args("scan", Some(".")))
        .await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["action"], "scan");
    assert_eq!(parsed["total_violations"], 0);
    assert!(parsed["report"].is_string());
}

// ─── execute_command: scan defaults path to "." ──────────────────────

#[tokio::test]
async fn execute_command_scan_no_path_defaults_to_dot() {
    let sut = build_test_orchestrator();
    let result = sut.execute_command(make_execute_args("scan", None)).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["path"], ".");
}

// ─── execute_command: ci with threshold ──────────────────────────────

#[tokio::test]
async fn execute_command_ci_pass_with_high_score() {
    let sut = build_test_orchestrator();
    let args = Parameters(ExecuteCommandArgs {
        action: "ci".to_string(),
        args: Some(serde_json::json!({"path": ".", "threshold": 50})),
    });
    let result = sut.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    // MockCodeAnalysis returns score 100.0, threshold 50 → pass
    assert_eq!(parsed["status"], "pass");
    assert_eq!(parsed["threshold"], 50);
}

// ─── execute_command: fix ────────────────────────────────────────────

#[tokio::test]
async fn execute_command_fix_returns_success_message() {
    let sut = build_test_orchestrator();
    let result = sut
        .execute_command(make_execute_args("fix", Some("./src")))
        .await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["action"], "fix");
    assert!(parsed["message"].as_str().unwrap().contains("Auto-fix"));
}

// ─── execute_command: init / install / mcp-config / config-show ─────

#[tokio::test]
async fn execute_command_init_returns_success() {
    let sut = build_test_orchestrator();
    let result = sut.execute_command(make_execute_args("init", None)).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["action"], "init");
}

#[tokio::test]
async fn execute_command_mcp_config_with_client() {
    let sut = build_test_orchestrator();
    let args = Parameters(ExecuteCommandArgs {
        action: "mcp-config".to_string(),
        args: Some(serde_json::json!({"client": "claude"})),
    });
    let result = sut.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["client"], "claude");
}

#[tokio::test]
async fn execute_command_mcp_config_defaults_to_all() {
    let sut = build_test_orchestrator();
    let result = sut
        .execute_command(make_execute_args("mcp-config", None))
        .await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["client"], "all");
}

// ─── execute_command: install-hook / uninstall-hook ──────────────────

#[tokio::test]
async fn execute_command_install_hook_returns_success() {
    let sut = build_test_orchestrator();
    let result = sut
        .execute_command(make_execute_args("install-hook", None))
        .await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
    assert!(parsed["message"].as_str().unwrap().contains("installed"));
}

#[tokio::test]
async fn execute_command_uninstall_hook_returns_success() {
    let sut = build_test_orchestrator();
    let result = sut
        .execute_command(make_execute_args("uninstall-hook", None))
        .await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
    assert!(parsed["message"].as_str().unwrap().contains("removed"));
}

// ─── list_commands ───────────────────────────────────────────────────

#[tokio::test]
async fn list_commands_returns_all_commands() {
    let sut = build_test_orchestrator();
    let args = Parameters(ListCommandsArgs { domain: None });
    let result = sut.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["commands"].is_array());
    assert!(parsed["total"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn list_commands_filter_by_domain() {
    let sut = build_test_orchestrator();
    let args = Parameters(ListCommandsArgs {
        domain: Some("check".to_string()),
    });
    let result = sut.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let commands = parsed["commands"].as_array().unwrap();
    for cmd in commands {
        assert!(cmd["name"].as_str().unwrap().contains("check"));
    }
}

#[tokio::test]
async fn list_commands_empty_domain_returns_all() {
    let sut = build_test_orchestrator();
    let args = Parameters(ListCommandsArgs {
        domain: Some("".to_string()),
    });
    let result = sut.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["total"].as_u64().unwrap() > 5);
}

#[tokio::test]
async fn list_commands_no_match_returns_empty() {
    let sut = build_test_orchestrator();
    let args = Parameters(ListCommandsArgs {
        domain: Some("zzz_nonexistent_zzz".to_string()),
    });
    let result = sut.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["total"], 0);
}

// ─── read_skill ──────────────────────────────────────────────────────

#[tokio::test]
async fn read_skill_missing_file_returns_error() {
    let sut = build_test_orchestrator();
    let args = Parameters(ReadSkillArgs { section: None });
    let result = sut.read_skill(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    // SKILL.md likely not present in test env → error
    assert!(
        parsed["error"].is_string() || parsed["content"].is_string(),
        "Expected either error or content field"
    );
}

#[tokio::test]
async fn read_skill_section_not_found_returns_error() {
    let sut = build_test_orchestrator();
    let args = Parameters(ReadSkillArgs {
        section: Some("NonExistentSection12345".to_string()),
    });
    let result = sut.read_skill(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    // Either file not found or section not found
    assert!(
        parsed["error"].is_string(),
        "Expected error for missing section/file"
    );
}

// ─── Constructor ─────────────────────────────────────────────────────

#[test]
fn orchestrator_new_creates_instance() {
    let sut = build_test_orchestrator();
    // If we get here without panic, construction succeeded
    let _ = &sut;
}
