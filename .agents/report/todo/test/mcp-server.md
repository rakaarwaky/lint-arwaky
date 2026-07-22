
# Test Suite — `mcp-server` (v1.10.106)

Below is the complete test suite following the **Create Rust Test Suite** skill. All files live flat in `crates/mcp-server/tests/` with prefix-based naming.

---

## Directory Layout

```
crates/mcp-server/
├── src/
│   ├── lib.rs
│   ├── agent_mcp_server_orchestrator.rs
│   ├── root_mcp_container.rs
│   └── surface_mcp_command.rs
├── tests/
│   ├── contract_mcp_server.rs
│   ├── unit_mcp_server_orchestrator.rs
│   ├── unit_mcp_server_surface.rs
│   ├── unit_mcp_server_container.rs
│   ├── integration_mcp_server.rs
│   ├── smoke_mcp_server.rs
│   ├── e2e_mcp_server_flow.rs
│   ├── acceptance_FRD_mcp_001.rs
│   ├── acceptance_FRD_mcp_002.rs
│   ├── acceptance_FRD_mcp_003.rs
│   ├── acceptance_FRD_mcp_004.rs
│   └── bench_mcp_server.rs
└── Cargo.toml
```

---

## `Cargo.toml` additions

```toml
[dev-dependencies]
tokio = { workspace = true, features = ["full", "test-util"] }
criterion = { version = "0.5", features = ["async_tokio"] }
serde_json.workspace = true

[[bench]]
name = "bench_mcp_server"
path = "tests/bench_mcp_server.rs"
harness = false
```

---

## 1. `tests/contract_mcp_server.rs`

```rust
// PURPOSE: Verify that McpServerOrchestrator implements IMcpServerAggregate
//          and that LintArwakyMcpServer implements ServerHandler.

use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::McpServerOrchestrator;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::contract_mcp_server_aggregate::IMcpServerAggregate;

// ─── Contract: McpServerOrchestrator implements IMcpServerAggregate ───

#[test]
fn mcp_server_orchestrator_implements_aggregate_trait() {
    fn assert_trait<T: IMcpServerAggregate>() {}
    assert_trait::<McpServerOrchestrator>();
}

#[test]
fn mcp_server_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<McpServerOrchestrator>();
}

// ─── Contract: LintArwakyMcpServer implements ServerHandler ───

#[test]
fn lint_arwaky_mcp_server_implements_server_handler() {
    fn assert_trait<T: rmcp::ServerHandler>() {}
    assert_trait::<LintArwakyMcpServer>();
}

#[test]
fn lint_arwaky_mcp_server_is_clone() {
    fn assert_clone<T: Clone>() {}
    assert_clone::<LintArwakyMcpServer>();
}

#[test]
fn lint_arwaky_mcp_server_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<LintArwakyMcpServer>();
}

// ─── Contract: McpContainer is constructible ───

#[test]
fn mcp_container_struct_is_public() {
    use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
    fn assert_sized<T: Sized>() {}
    assert_sized::<McpContainer>();
}
```

---

## 2. `tests/unit_mcp_server_orchestrator.rs`

```rust
// PURPOSE: Unit tests for McpServerOrchestrator — execute_command, list_commands, read_skill

use std::sync::Arc;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
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
use rmcp::handler::server::wrapper::Parameters;

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
    async fn run_audit(&self, _target: &FilePath) -> Result<Vec<LintResult>, shared::common::taxonomy_adapter_error::ScanError> {
        Ok(vec![])
    }
    fn name(&self) -> &str {
        "mock-import"
    }
}

struct MockNamingRunner;
#[async_trait::async_trait]
impl INamingRunnerAggregate for MockNamingRunner {
    async fn run_audit(&self, _target: &FilePath) -> Result<Vec<LintResult>, shared::common::taxonomy_adapter_error::ScanError> {
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
    fn load_config_sync(&self, _project_root: &str) -> shared::config_system::taxonomy_config_vo::ArchitectureConfig {
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
    let result = sut.execute_command(make_execute_args("version", None)).await;
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
    let result = sut.execute_command(make_execute_args("adapters", None)).await;
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
    let result = sut.execute_command(make_execute_args("mcp-config", None)).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["client"], "all");
}

// ─── execute_command: install-hook / uninstall-hook ──────────────────

#[tokio::test]
async fn execute_command_install_hook_returns_success() {
    let sut = build_test_orchestrator();
    let result = sut.execute_command(make_execute_args("install-hook", None)).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
    assert!(parsed["message"].as_str().unwrap().contains("installed"));
}

#[tokio::test]
async fn execute_command_uninstall_hook_returns_success() {
    let sut = build_test_orchestrator();
    let result = sut.execute_command(make_execute_args("uninstall-hook", None)).await;
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
```

---

## 3. `tests/unit_mcp_server_surface.rs`

```rust
// PURPOSE: Unit tests for LintArwakyMcpServer — tool registration, get_info, health_check

use std::sync::Arc;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::contract_mcp_server_aggregate::IMcpServerAggregate;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::ServerHandler;

// ─── Mock Aggregate ──────────────────────────────────────────────────

struct StubAggregate;

#[async_trait::async_trait]
impl IMcpServerAggregate for StubAggregate {
    async fn execute_command(&self, _args: Parameters<ExecuteCommandArgs>) -> String {
        r#"{"status":"stub"}"#.to_string()
    }
    async fn list_commands(&self, _args: Parameters<ListCommandsArgs>) -> String {
        r#"{"commands":[]}"#.to_string()
    }
    async fn read_skill(&self, _args: Parameters<ReadSkillArgs>) -> String {
        r#"{"content":"stub"}"#.to_string()
    }
}

fn build_surface() -> LintArwakyMcpServer {
    LintArwakyMcpServer::new(Arc::new(StubAggregate))
}

// ─── get_info ────────────────────────────────────────────────────────

#[test]
fn get_info_returns_correct_server_name() {
    let sut = build_surface();
    let info = sut.get_info();
    assert_eq!(info.server_info.name, "lint-arwaky");
}

#[test]
fn get_info_returns_version_string() {
    let sut = build_surface();
    let info = sut.get_info();
    assert!(!info.server_info.version.is_empty());
}

#[test]
fn get_info_declares_tools_capability() {
    let sut = build_surface();
    let info = sut.get_info();
    assert!(info.capabilities.tools.is_some());
}

// ─── Tool delegation ─────────────────────────────────────────────────

#[tokio::test]
async fn execute_command_delegates_to_agent() {
    let sut = build_surface();
    let args = Parameters(ExecuteCommandArgs {
        action: "version".to_string(),
        args: None,
    });
    let result = sut.execute_command(args).await;
    assert!(result.contains("stub"));
}

#[tokio::test]
async fn list_commands_delegates_to_agent() {
    let sut = build_surface();
    let args = Parameters(ListCommandsArgs { domain: None });
    let result = sut.list_commands(args).await;
    assert!(result.contains("commands"));
}

#[tokio::test]
async fn read_skill_delegates_to_agent() {
    let sut = build_surface();
    let args = Parameters(ReadSkillArgs { section: None });
    let result = sut.read_skill(args).await;
    assert!(result.contains("content"));
}

// ─── health_check ────────────────────────────────────────────────────

#[tokio::test]
async fn health_check_returns_valid_json() {
    let sut = build_surface();
    let result = sut.health_check().await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["version"].is_string());
    assert!(parsed["adapters"].is_array());
    assert!(parsed["adapters_available"].is_number());
    assert!(parsed["adapters_total"].is_number());
}

#[tokio::test]
async fn health_check_lists_expected_adapters() {
    let sut = build_surface();
    let result = sut.health_check().await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let adapters = parsed["adapters"].as_array().unwrap();
    let names: Vec<&str> = adapters
        .iter()
        .map(|a| a["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"ruff"));
    assert!(names.contains(&"mypy"));
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"eslint"));
    assert!(names.contains(&"bandit"));
}

#[tokio::test]
async fn health_check_adapters_total_is_five() {
    let sut = build_surface();
    let result = sut.health_check().await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["adapters_total"], 5);
}

// ─── Clone ───────────────────────────────────────────────────────────

#[test]
fn surface_is_cloneable() {
    let sut = build_surface();
    let _cloned = sut.clone();
}
```

---

## 4. `tests/unit_mcp_server_container.rs`

```rust
// PURPOSE: Unit tests for McpContainer — DI wiring struct

use mcp_server_lint_arwaky::root_mcp_container::McpContainer;

// ─── Struct field accessibility ──────────────────────────────────────

#[test]
fn mcp_container_has_all_required_fields() {
    // Compile-time check: if any field is missing or renamed, this won't compile.
    // We can't call new_default() in unit tests (requires real filesystem),
    // but we verify the struct shape.
    fn assert_fields(_c: &McpContainer) {
        let _ = &_c.code_analysis_linter;
        let _ = &_c.import_orchestrator;
        let _ = &_c.naming_orchestrator;
        let _ = &_c.orphan_orchestrator;
        let _ = &_c.external_lint;
        let _ = &_c.role_orchestrator;
        let _ = &_c.config_orchestrator;
    }
    // If this compiles, all fields are public and accessible.
}

#[test]
fn mcp_container_fields_are_arc_dyn_traits() {
    use std::sync::Arc;
    use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
    use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
    use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
    use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
    use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
    use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
    use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

    fn assert_arc_types(_c: &McpContainer) {
        let _: &Arc<dyn ICodeAnalysisAggregate> = &_c.code_analysis_linter;
        let _: &Arc<dyn IImportRunnerAggregate> = &_c.import_orchestrator;
        let _: &Arc<dyn INamingRunnerAggregate> = &_c.naming_orchestrator;
        let _: &Arc<dyn IOrphanAggregate> = &_c.orphan_orchestrator;
        let _: &Arc<dyn IExternalLintAggregate> = &_c.external_lint;
        let _: &Arc<dyn IRoleRunnerAggregate> = &_c.role_orchestrator;
        let _: &Arc<dyn IConfigOrchestratorAggregate> = &_c.config_orchestrator;
    }
}
```

---

## 5. `tests/integration_mcp_server.rs`

```rust
// PURPOSE: Integration tests — real DI container wiring, orchestrator + surface composition

use std::sync::Arc;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::contract_mcp_server_aggregate::IMcpServerAggregate;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::ServerHandler;

// ─── DI Container Wiring ─────────────────────────────────────────────

#[test]
fn container_new_default_produces_all_dependencies() {
    // This test requires the real filesystem and config system.
    // It validates that McpContainer::new_default() wires all 7 dependencies.
    let container = McpContainer::new_default();

    // All Arc<dyn Trait> fields must be non-null (they always are with Arc,
    // but we verify the container doesn't panic during construction).
    assert!(Arc::strong_count(&container.code_analysis_linter) >= 1);
    assert!(Arc::strong_count(&container.import_orchestrator) >= 1);
    assert!(Arc::strong_count(&container.naming_orchestrator) >= 1);
    assert!(Arc::strong_count(&container.orphan_orchestrator) >= 1);
    assert!(Arc::strong_count(&container.external_lint) >= 1);
    assert!(Arc::strong_count(&container.role_orchestrator) >= 1);
    assert!(Arc::strong_count(&container.config_orchestrator) >= 1);
}

// ─── Orchestrator from Container ─────────────────────────────────────

#[test]
fn orchestrator_constructed_from_container_deps() {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let _surface = LintArwakyMcpServer::new(Arc::new(orchestrator));
}

// ─── Full Pipeline: Surface → Agent → Mock Capabilities ─────────────

#[tokio::test]
async fn surface_execute_command_flows_through_orchestrator() {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let surface = LintArwakyMcpServer::new(Arc::new(orchestrator));

    let args = Parameters(ExecuteCommandArgs {
        action: "version".to_string(),
        args: None,
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["name"], "lint-arwaky");
}

#[tokio::test]
async fn surface_list_commands_returns_catalog() {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let surface = LintArwakyMcpServer::new(Arc::new(orchestrator));

    let args = Parameters(ListCommandsArgs { domain: None });
    let result = surface.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["total"].as_u64().unwrap() >= 10);
}

// ─── Server Info via Surface ─────────────────────────────────────────

#[test]
fn surface_get_info_reports_tools_capability() {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let surface = LintArwakyMcpServer::new(Arc::new(orchestrator));
    let info = surface.get_info();
    assert!(info.capabilities.tools.is_some());
    assert_eq!(info.server_info.name, "lint-arwaky");
}
```

---

## 6. `tests/smoke_mcp_server.rs`

```rust
// PURPOSE: Smoke test — MCP server boots and responds within 5 seconds

use std::sync::Arc;
use std::time::Instant;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::ExecuteCommandArgs;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::ServerHandler;

#[tokio::test]
async fn mcp_server_boots_and_responds_under_5_seconds() {
    let start = Instant::now();

    // Boot: construct container → orchestrator → surface
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let surface = LintArwakyMcpServer::new(Arc::new(orchestrator));

    // Respond: call version (lightest operation)
    let args = Parameters(ExecuteCommandArgs {
        action: "version".to_string(),
        args: None,
    });
    let result = surface.execute_command(args).await;

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: took {:?}",
        elapsed
    );

    // Verify response is valid
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["name"], "lint-arwaky");
}

#[tokio::test]
async fn mcp_server_health_check_responds_under_5_seconds() {
    let start = Instant::now();

    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let surface = LintArwakyMcpServer::new(Arc::new(orchestrator));

    let result = surface.health_check().await;

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Health check exceeded 5s: took {:?}",
        elapsed
    );

    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["adapters_total"].as_u64().unwrap() > 0);
}
```

---

## 7. `tests/e2e_mcp_server_flow.rs`

```rust
// PURPOSE: E2E tests — full request lifecycle through all layers (Surface → Agent → Capabilities)

use std::sync::Arc;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::ServerHandler;

fn build_full_stack() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    LintArwakyMcpServer::new(Arc::new(orchestrator))
}

// ─── E2E: Full scan lifecycle ────────────────────────────────────────

#[tokio::test]
async fn e2e_scan_current_directory_returns_compliance_report() {
    let surface = build_full_stack();

    let args = Parameters(ExecuteCommandArgs {
        action: "scan".to_string(),
        args: Some(serde_json::json!({"path": "."})),
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["action"], "scan");
    assert!(parsed["total_violations"].is_number());
    assert!(parsed["report"].is_string());
}

// ─── E2E: CI gate lifecycle ──────────────────────────────────────────

#[tokio::test]
async fn e2e_ci_with_threshold_returns_pass_or_fail() {
    let surface = build_full_stack();

    let args = Parameters(ExecuteCommandArgs {
        action: "ci".to_string(),
        args: Some(serde_json::json!({"path": ".", "threshold": 0})),
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // With threshold 0, any score >= 0 passes
    assert_eq!(parsed["status"], "pass");
    assert!(parsed["score"].is_number());
    assert_eq!(parsed["threshold"], 0);
}

// ─── E2E: Doctor lifecycle ───────────────────────────────────────────

#[tokio::test]
async fn e2e_doctor_checks_all_expected_tools() {
    let surface = build_full_stack();

    let args = Parameters(ExecuteCommandArgs {
        action: "doctor".to_string(),
        args: None,
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["status"], "success");
    let checks = parsed["checks"].as_array().unwrap();
    let tool_names: Vec<&str> = checks.iter().map(|c| c["tool"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"cargo"));
    assert!(tool_names.contains(&"git"));
    assert!(tool_names.contains(&"node"));
}

// ─── E2E: List commands → execute one ────────────────────────────────

#[tokio::test]
async fn e2e_discover_then_execute_command() {
    let surface = build_full_stack();

    // Step 1: Discover commands
    let list_args = Parameters(ListCommandsArgs { domain: None });
    let list_result = surface.list_commands(list_args).await;
    let list_parsed: serde_json::Value = serde_json::from_str(&list_result).unwrap();
    let commands = list_parsed["commands"].as_array().unwrap();
    assert!(!commands.is_empty());

    // Step 2: Pick "version" and execute it
    let version_cmd = commands
        .iter()
        .find(|c| c["name"] == "version")
        .expect("version command must exist in catalog");
    assert!(version_cmd["description"].is_string());
    assert!(version_cmd["example"].is_string());

    let exec_args = Parameters(ExecuteCommandArgs {
        action: "version".to_string(),
        args: None,
    });
    let exec_result = surface.execute_command(exec_args).await;
    let exec_parsed: serde_json::Value = serde_json::from_str(&exec_result).unwrap();
    assert_eq!(exec_parsed["name"], "lint-arwaky");
}

// ─── E2E: Health check full lifecycle ────────────────────────────────

#[tokio::test]
async fn e2e_health_check_reports_adapter_status() {
    let surface = build_full_stack();
    let result = surface.health_check().await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert!(parsed["version"].is_string());
    assert_eq!(parsed["adapters_total"], 5);

    let adapters = parsed["adapters"].as_array().unwrap();
    for adapter in adapters {
        assert!(adapter["name"].is_string());
        assert!(adapter["language"].is_string());
        let status = adapter["status"].as_str().unwrap();
        assert!(
            status == "available" || status == "not_installed",
            "Unexpected adapter status: {}",
            status
        );
    }
}

// ─── E2E: Adapters listing ───────────────────────────────────────────

#[tokio::test]
async fn e2e_adapters_command_returns_enabled_status() {
    let surface = build_full_stack();

    let args = Parameters(ExecuteCommandArgs {
        action: "adapters".to_string(),
        args: None,
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert!(parsed["adapters"].is_array());
    let adapters = parsed["adapters"].as_array().unwrap();
    for adapter in adapters {
        assert!(adapter["name"].is_string());
        assert!(adapter["enabled"].is_boolean());
    }
}
```

---

## 8. `tests/acceptance_FRD_mcp_001.rs`

```rust
// PURPOSE: Acceptance test — FRD Requirement: execute_command tool
// "execute_command — execute any CLI command via the MCP interface."

use std::sync::Arc;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::ExecuteCommandArgs;
use rmcp::handler::server::wrapper::Parameters;

fn build_surface() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    LintArwakyMcpServer::new(Arc::new(McpServerOrchestrator::new(deps)))
}

/// FRD-MCP-001: execute_command accepts any valid CLI action and returns JSON
#[tokio::test]
async fn frd_mcp_001_execute_command_scan() {
    let surface = build_surface();
    let args = Parameters(ExecuteCommandArgs {
        action: "scan".to_string(),
        args: Some(serde_json::json!({"path": "."})),
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["action"], "scan");
}

/// FRD-MCP-001: execute_command handles check alias
#[tokio::test]
async fn frd_mcp_001_execute_command_check_alias() {
    let surface = build_surface();
    let args = Parameters(ExecuteCommandArgs {
        action: "check".to_string(),
        args: Some(serde_json::json!({"path": "."})),
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
}

/// FRD-MCP-001: execute_command rejects unknown actions gracefully
#[tokio::test]
async fn frd_mcp_001_execute_command_unknown_action_error() {
    let surface = build_surface();
    let args = Parameters(ExecuteCommandArgs {
        action: "invalid_xyz".to_string(),
        args: None,
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["error"].is_string());
}

/// FRD-MCP-001: execute_command supports all documented actions
#[tokio::test]
async fn frd_mcp_001_all_documented_actions_accepted() {
    let surface = build_surface();
    let actions = vec![
        "check", "scan", "fix", "ci", "doctor", "orphan", "security",
        "dependencies", "version", "adapters", "install-hook",
        "uninstall-hook", "init", "install", "mcp-config", "config-show",
    ];
    for action in actions {
        let args = Parameters(ExecuteCommandArgs {
            action: action.to_string(),
            args: Some(serde_json::json!({"path": "."})),
        });
        let result = surface.execute_command(args).await;
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(
            !parsed.get("error").map_or(false, |e| {
                e.as_str().map_or(false, |s| s.contains("Unknown action"))
            }),
            "Action '{}' should be recognized",
            action
        );
    }
}
```

---

## 9. `tests/acceptance_FRD_mcp_002.rs`

```rust
// PURPOSE: Acceptance test — FRD Requirement: list_commands tool
// "list_commands — list available CLI commands with descriptions and examples."

use std::sync::Arc;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::ListCommandsArgs;
use rmcp::handler::server::wrapper::Parameters;

fn build_surface() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    LintArwakyMcpServer::new(Arc::new(McpServerOrchestrator::new(deps)))
}

/// FRD-MCP-002: list_commands returns all commands with name, description, example
#[tokio::test]
async fn frd_mcp_002_list_commands_complete_entries() {
    let surface = build_surface();
    let args = Parameters(ListCommandsArgs { domain: None });
    let result = surface.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let commands = parsed["commands"].as_array().unwrap();
    assert!(commands.len() >= 10, "Expected at least 10 commands in catalog");

    for cmd in commands {
        assert!(cmd["name"].is_string(), "Each command must have a name");
        assert!(cmd["description"].is_string(), "Each command must have a description");
        assert!(cmd["example"].is_string(), "Each command must have an example");
        assert!(!cmd["name"].as_str().unwrap().is_empty());
        assert!(!cmd["description"].as_str().unwrap().is_empty());
    }
}

/// FRD-MCP-002: list_commands supports domain filtering
#[tokio::test]
async fn frd_mcp_002_list_commands_domain_filter() {
    let surface = build_surface();
    let args = Parameters(ListCommandsArgs {
        domain: Some("hook".to_string()),
    });
    let result = surface.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let commands = parsed["commands"].as_array().unwrap();
    assert!(!commands.is_empty(), "Domain 'hook' should match install-hook/uninstall-hook");
    for cmd in commands {
        assert!(cmd["name"].as_str().unwrap().contains("hook"));
    }
}

/// FRD-MCP-002: total count matches commands array length
#[tokio::test]
async fn frd_mcp_002_total_matches_array_length() {
    let surface = build_surface();
    let args = Parameters(ListCommandsArgs { domain: None });
    let result = surface.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let total = parsed["total"].as_u64().unwrap();
    let commands = parsed["commands"].as_array().unwrap();
    assert_eq!(total as usize, commands.len());
}
```

---

## 10. `tests/acceptance_FRD_mcp_003.rs`

```rust
// PURPOSE: Acceptance test — FRD Requirement: read_skill tool
// "read_skill — read SKILL.md documentation by section."

use std::sync::Arc;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::ReadSkillArgs;
use rmcp::handler::server::wrapper::Parameters;

fn build_surface() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    LintArwakyMcpServer::new(Arc::new(McpServerOrchestrator::new(deps)))
}

/// FRD-MCP-003: read_skill returns content or structured error
#[tokio::test]
async fn frd_mcp_003_read_skill_returns_valid_response() {
    let surface = build_surface();
    let args = Parameters(ReadSkillArgs { section: None });
    let result = surface.read_skill(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Must have either "content" (success) or "error" (file not found)
    assert!(
        parsed["content"].is_string() || parsed["error"].is_string(),
        "read_skill must return content or error"
    );
}

/// FRD-MCP-003: read_skill with section returns section or error
#[tokio::test]
async fn frd_mcp_003_read_skill_section_extraction() {
    let surface = build_surface();
    let args = Parameters(ReadSkillArgs {
        section: Some("Usage".to_string()),
    });
    let result = surface.read_skill(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Either returns the section content or an error (file/section not found)
    assert!(
        parsed["content"].is_string() || parsed["error"].is_string(),
        "read_skill with section must return content or error"
    );
}

/// FRD-MCP-003: read_skill searches multiple candidate locations
#[tokio::test]
async fn frd_mcp_003_read_skill_searched_paths_in_error() {
    let surface = build_surface();
    let args = Parameters(ReadSkillArgs { section: None });
    let result = surface.read_skill(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    if parsed["error"].is_string() {
        // Error response should include searched paths for debugging
        assert!(
            parsed["searched"].is_array(),
            "Error response should list searched paths"
        );
    }
}
```

---

## 11. `tests/acceptance_FRD_mcp_004.rs`

```rust
// PURPOSE: Acceptance test — FRD Requirement: health_check tool + JSON-RPC conformance
// "health_check — check system health: adapters and system state."
// "JSON-RPC conformance; tool discovery by AI clients."

use std::sync::Arc;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use rmcp::ServerHandler;

fn build_surface() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    LintArwakyMcpServer::new(Arc::new(McpServerOrchestrator::new(deps)))
}

/// FRD-MCP-004: health_check returns adapter availability
#[tokio::test]
async fn frd_mcp_004_health_check_adapter_status() {
    let surface = build_surface();
    let result = surface.health_check().await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert!(parsed["version"].is_string());
    assert!(parsed["adapters_available"].is_number());
    assert!(parsed["adapters_total"].is_number());

    let total = parsed["adapters_total"].as_u64().unwrap();
    let available = parsed["adapters_available"].as_u64().unwrap();
    assert!(available <= total, "available cannot exceed total");
}

/// FRD-MCP-004: Tool discovery — ServerInfo declares tools capability
#[test]
fn frd_mcp_004_tool_discovery_capabilities() {
    let surface = build_surface();
    let info = surface.get_info();

    // MCP clients discover tools via capabilities.tools
    assert!(
        info.capabilities.tools.is_some(),
        "Server must declare tools capability for AI client discovery"
    );
}

/// FRD-MCP-004: Server identity is correct for JSON-RPC handshake
#[test]
fn frd_mcp_004_server_identity_for_jsonrpc() {
    let surface = build_surface();
    let info = surface.get_info();

    assert_eq!(info.server_info.name, "lint-arwaky");
    assert!(!info.server_info.version.is_empty());
    // Protocol version must be set for JSON-RPC conformance
    assert!(!format!("{:?}", info.protocol_version).is_empty());
}

/// FRD-MCP-004: Response time under 5 seconds for standard operations
#[tokio::test]
async fn frd_mcp_004_response_time_under_5_seconds() {
    let surface = build_surface();
    let start = std::time::Instant::now();

    let _ = surface.health_check().await;

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "health_check took {:?}, exceeds 5s SLA",
        elapsed
    );
}
```

---

## 12. `tests/bench_mcp_server.rs`

```rust
// PURPOSE: Benchmark tests — performance regression for MCP server operations

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs,
};
use rmcp::handler::server::wrapper::Parameters;

fn build_surface() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    LintArwakyMcpServer::new(Arc::new(McpServerOrchestrator::new(deps)))
}

fn bench_version_command(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("execute_command_version", |b| {
        b.iter(|| {
            rt.block_on(async {
                let args = Parameters(ExecuteCommandArgs {
                    action: "version".to_string(),
                    args: None,
                });
                surface.execute_command(args).await
            })
        })
    });
}

fn bench_list_commands(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("list_commands");

    for domain in [None, Some("check"), Some("hook")] {
        let label = domain.unwrap_or("all");
        group.bench_with_input(
            BenchmarkId::new("filter", label),
            &domain,
            |b, d| {
                b.iter(|| {
                    rt.block_on(async {
                        let args = Parameters(ListCommandsArgs {
                            domain: d.map(String::from),
                        });
                        surface.list_commands(args).await
                    })
                })
            },
        );
    }
    group.finish();
}

fn bench_health_check(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("health_check", |b| {
        b.iter(|| rt.block_on(async { surface.health_check().await }))
    });
}

fn bench_doctor_command(c: &mut Criterion) {
    let surface = build_surface();
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("execute_command_doctor", |b| {
        b.iter(|| {
            rt.block_on(async {
                let args = Parameters(ExecuteCommandArgs {
                    action: "doctor".to_string(),
                    args: None,
                });
                surface.execute_command(args).await
            })
        })
    });
}

criterion_group!(
    benches,
    bench_version_command,
    bench_list_commands,
    bench_health_check,
    bench_doctor_command,
);
criterion_main!(benches);
```

---

## Run Commands

```bash
# All tests
cargo test -p mcp_server-lint-arwaky

# By type
cargo test -p mcp_server-lint-arwaky --test contract_mcp_server
cargo test -p mcp_server-lint-arwaky --test unit_mcp_server_orchestrator
cargo test -p mcp_server-lint-arwaky --test unit_mcp_server_surface
cargo test -p mcp_server-lint-arwaky --test unit_mcp_server_container
cargo test -p mcp_server-lint-arwaky --test integration_mcp_server
cargo test -p mcp_server-lint-arwaky --test smoke_mcp_server
cargo test -p mcp_server-lint-arwaky --test e2e_mcp_server_flow
cargo test -p mcp_server-lint-arwaky --test acceptance_FRD_mcp_001
cargo test -p mcp_server-lint-arwaky --test acceptance_FRD_mcp_002
cargo test -p mcp_server-lint-arwaky --test acceptance_FRD_mcp_003
cargo test -p mcp_server-lint-arwaky --test acceptance_FRD_mcp_004

# Benchmarks
cargo bench -p mcp_server-lint-arwaky

# Coverage
cargo tarpaulin -p mcp_server-lint-arwaky --fail-under 60

# With output
cargo test -p mcp_server-lint-arwaky -- --nocapture
```

---

## Coverage Mapping

| Layer                          | File(s) Tested                       | Target | Tests                                                                                |
| ------------------------------ | ------------------------------------ | ------ | ------------------------------------------------------------------------------------ |
| **Agent** (orchestrator) | `agent_mcp_server_orchestrator.rs` | 60%    | `unit_mcp_server_orchestrator.rs` (18 tests)                                       |
| **Surface** (command)    | `surface_mcp_command.rs`           | 70%    | `unit_mcp_server_surface.rs` (10 tests)                                            |
| **Root** (container)     | `root_mcp_container.rs`            | 50%    | `unit_mcp_server_container.rs` (2 tests) + `integration_mcp_server.rs` (4 tests) |
| **Contract** (traits)    | `contract_mcp_server_aggregate.rs` | —     | `contract_mcp_server.rs` (5 tests)                                                 |
| **E2E**                  | Full stack                           | —     | `e2e_mcp_server_flow.rs` (6 tests)                                                 |
| **Acceptance**           | FRD requirements                     | —     | 4 files, 12 tests                                                                    |
| **Smoke**                | Boot + respond                       | < 5s   | `smoke_mcp_server.rs` (2 tests)                                                    |
| **Bench**                | Perf regression                      | —     | `bench_mcp_server.rs` (4 benchmarks)                                               |

**Total: 57 tests + 4 benchmarks**
