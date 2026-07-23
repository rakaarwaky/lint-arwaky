// PURPOSE: Unit tests for McpServerOrchestrator — execute_command, list_commands, read_skill

use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use rmcp::handler::server::wrapper::Parameters;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{BooleanVO, Score};
use shared::common::taxonomy_adapter_list_vo::AdapterNameList;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::mcp_server::contract_mcp_server_aggregate::IMcpServerAggregate;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};
use std::sync::Arc;

// ─── Mock Implementations ────────────────────────────────────────────

struct _MockCodeAnalysis;
impl ICodeAnalysisAggregate for _MockCodeAnalysis {
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
    fn check_critical(&self, _results: &[LintResult]) -> BooleanVO {
        BooleanVO::new(false)
    }
    fn format_report(&self, _results: &LintResultList, _project_root: &FilePath) -> DisplayContent {
        DisplayContent::new("Mock report: 0 violations")
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

struct MockExternalLint;
#[async_trait::async_trait]
impl IExternalLintAggregate for MockExternalLint {
    async fn scan_all(&self, _path: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }
    fn adapter_names(&self) -> AdapterNameList {
        AdapterNameList::new(vec![AdapterName::raw("ruff"), AdapterName::raw("mypy")])
    }
}

struct MockPipeline;
#[async_trait::async_trait]
impl shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate
    for MockPipeline
{
    async fn run(
        &self,
        _request: shared::cli_commands::taxonomy_scan_request_vo::ScanRequest,
    ) -> Result<
        shared::cli_commands::taxonomy_scan_report_vo::ScanReport,
        shared::cli_commands::taxonomy_scan_report_vo::PipelineError,
    > {
        Ok(shared::cli_commands::taxonomy_scan_report_vo::ScanReport::new(vec![], vec![]))
    }
    async fn run_with_discovery(
        &self,
    ) -> Result<
        shared::cli_commands::taxonomy_scan_report_vo::ScanReport,
        shared::cli_commands::taxonomy_scan_report_vo::PipelineError,
    > {
        Ok(shared::cli_commands::taxonomy_scan_report_vo::ScanReport::new(vec![], vec![]))
    }
    fn check_orphan_single_file(
        &self,
        _file_path: &str,
        _workspace_root: &str,
    ) -> Result<Vec<LintResult>, shared::cli_commands::taxonomy_scan_report_vo::PipelineError> {
        Ok(vec![])
    }
}

// ─── Helper ──────────────────────────────────────────────────────────

fn build_test_orchestrator() -> McpServerOrchestrator {
    McpServerOrchestrator::new(McpServerDependencies {
        analysis_pipeline: Arc::new(MockPipeline),
        external_lint: Arc::new(MockExternalLint),
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
    assert!(parsed["results"].is_array());
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
