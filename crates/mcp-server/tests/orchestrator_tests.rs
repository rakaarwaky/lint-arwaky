use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::find_workspace_root;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::contract_mcp_server_aggregate::IMcpServerAggregate;
use mcp_server_lint_arwaky::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};
use rmcp::handler::server::wrapper::Parameters;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::contract_scanner_provider_port::IScannerProviderPort;
use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::taxonomy_diff_result_vo::GitDiffResultVO;
use shared::git_hooks::taxonomy_git_diff_data_vo::{GitDiffDataVO, HookIgnoreUpdateVO};
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::mcp_server::taxonomy_action_vo::JobId;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, SecurityScanReport, ToolchainDiagnostics,
};
use shared::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

struct MockDeps;

impl ICodeAnalysisAggregate for MockDeps {
    fn run_code_analysis(&self, _: &str) -> LintResultList {
        unreachable!()
    }
    fn run_code_analysis_dir(&self, _: &str) -> LintResultList {
        unreachable!()
    }
    fn run_code_analysis_path(&self, _: &str) -> Vec<LintResult> {
        unreachable!()
    }
    fn calc_score(&self, _: &[LintResult]) -> f64 {
        unreachable!()
    }
    fn check_critical(&self, _: &[LintResult]) -> bool {
        unreachable!()
    }
    fn format_report(&self, _: &LintResultList, _: &str) -> String {
        unreachable!()
    }
    fn active_rules(
        &self,
    ) -> Vec<shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO> {
        unreachable!()
    }
}

#[async_trait::async_trait]
impl IImportRunnerAggregate for MockDeps {
    async fn run_audit(&self, _: &FilePath) -> Vec<LintResult> {
        unreachable!()
    }
    fn name(&self) -> &str {
        unreachable!()
    }
}

#[async_trait::async_trait]
impl INamingRunnerAggregate for MockDeps {
    async fn run_audit(&self, _: &FilePath) -> Vec<LintResult> {
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
    ) -> Vec<LintResult> {
        Vec::new()
    }
}

impl ILayerDetectionAggregate for MockDeps {
    fn detect_layer(&self, _: &str, _: &str) -> Option<String> {
        None
    }
    fn get_layer_def(
        &self,
        _: &str,
    ) -> Option<shared::common::taxonomy_definition_vo::LayerDefinition> {
        None
    }
    fn get_orphan_entry_points(&self) -> Vec<String> {
        Vec::new()
    }
    fn config(&self) -> &shared::config_system::taxonomy_config_vo::ArchitectureConfig {
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
        Ok(shared::common::taxonomy_paths_vo::FilePathList { values: Vec::new() })
    }
    fn get_ignored_files(&self) -> shared::common::taxonomy_paths_vo::FilePathList {
        shared::common::taxonomy_paths_vo::FilePathList { values: Vec::new() }
    }
}

#[async_trait::async_trait]
impl IExternalLintAggregate for MockDeps {
    async fn scan_all(&self, _: &FilePath) -> LintResultList {
        unreachable!()
    }
    fn adapter_names(&self) -> Vec<String> {
        unreachable!()
    }
}

#[async_trait::async_trait]
impl IRoleRunnerAggregate for MockDeps {
    async fn run_audit(&self, _: &FilePath) -> Vec<LintResult> {
        unreachable!()
    }
    fn name(&self) -> &str {
        unreachable!()
    }
}

struct MockDiffProtocol;
#[async_trait::async_trait]
impl IDiffProtocol for MockDiffProtocol {
    async fn run_git_diff_check(&self, _: &FilePath) -> LintResultList {
        unreachable!()
    }
    async fn get_diff(&self, _: &FilePath) -> GitDiffResultVO {
        unreachable!()
    }
    async fn get_changed_files(&self, _: &FilePath, _: &str) -> FilePathList {
        unreachable!()
    }
    async fn get_default_branch(&self, _: &FilePath) -> String {
        unreachable!()
    }
}

struct MockHookProtocol;
#[async_trait::async_trait]
impl IHookProtocol for MockHookProtocol {
    async fn install_pre_commit(&self, _: &FilePath) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
    async fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
    fn get_hook_manager_identity(&self) -> Identity {
        unreachable!()
    }
    async fn initialize_config(&self, _: &str) -> DescriptionVO {
        unreachable!()
    }
    fn update_ignore_rule(&self, _: HookIgnoreUpdateVO) -> DescriptionVO {
        unreachable!()
    }
    async fn get_diff_data(&self, _: &str, _: &str) -> GitDiffDataVO {
        unreachable!()
    }
}

#[async_trait::async_trait]
impl MaintenanceCommandsAggregate for MockDeps {
    async fn stats(&self, _: &FilePath) -> MaintenanceStatsVO {
        unreachable!()
    }
    async fn clean(&self) {
        unreachable!()
    }
    async fn update(&self) {
        unreachable!()
    }
    async fn doctor(&self) -> DoctorResultVO {
        unreachable!()
    }
    async fn cancel(&self, _: JobId) {
        unreachable!()
    }
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        unreachable!()
    }
    async fn run_security_scan(&self, _: &FilePath) -> SecurityScanReport {
        unreachable!()
    }
    async fn run_dependency_report(&self, _: &FilePath) -> Result<DependencyReport, String> {
        unreachable!()
    }
}

#[async_trait::async_trait]
impl GitHooksAggregate for MockDeps {
    fn diff_protocol(&self) -> &dyn IDiffProtocol {
        &MockDiffProtocol
    }
    fn hook_protocol(&self) -> &dyn IHookProtocol {
        &MockHookProtocol
    }
}

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
        maintenance_orchestrator: Arc::new(MockDeps),
        git_hooks_aggregate: Arc::new(MockDeps),
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

#[test]
fn find_workspace_root_finds_cargo_toml() {
    let root = find_workspace_root("crates/mcp-server/src/lib.rs");
    assert!(root.is_some(), "should find workspace root from subpath");
    let path = root.unwrap();
    assert!(
        path.join("Cargo.toml").exists(),
        "root should contain Cargo.toml"
    );
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
    assert!(root.is_none(), "no marker dirs → should be None");

    std::fs::create_dir_all(&tmp.join("crates")).unwrap();
    let root2 = find_workspace_root(tmp.join("sub/a/b").to_str().unwrap());
    assert_eq!(
        root2.as_deref(),
        Some(tmp.as_path()),
        "should find crates/ dir"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn find_workspace_root_finds_packages_dir() {
    let tmp = std::env::temp_dir().join("test_find_root_packages");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp.join("packages")).unwrap();
    std::fs::create_dir_all(&tmp.join("sub/deep")).unwrap();
    let root = find_workspace_root(tmp.join("sub/deep").to_str().unwrap());
    assert_eq!(
        root.as_deref(),
        Some(tmp.as_path()),
        "should find packages/ dir"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn find_workspace_root_finds_modules_dir() {
    let tmp = std::env::temp_dir().join("test_find_root_modules");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp.join("modules")).unwrap();
    let root = find_workspace_root(tmp.to_str().unwrap());
    assert_eq!(
        root.as_deref(),
        Some(tmp.as_path()),
        "modules/ at root itself"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn find_workspace_root_absolute_path_does_not_call_current_dir() {
    let tmp = std::env::temp_dir().join("test_find_root_abs");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let root = find_workspace_root(tmp.to_str().unwrap());
    assert!(root.is_none(), "absolute path without markers → None");
    let _ = std::fs::remove_dir_all(&tmp);
}

#[tokio::test]
async fn list_commands_returns_all_without_domain() {
    let orch = make_orchestrator();
    let params = make_list_params(None);
    let output = orch.list_commands(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
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
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    let commands = parsed["commands"].as_array().unwrap();
    assert!(!commands.is_empty());
    for cmd in commands {
        let name = cmd["name"].as_str().unwrap();
        assert!(name.contains("check"));
    }
}

#[tokio::test]
async fn list_commands_unknown_domain_returns_empty() {
    let orch = make_orchestrator();
    let params = make_list_params(Some("zzz_nonexistent_999"));
    let output = orch.list_commands(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["total"].as_u64().unwrap(), 0);
}

#[tokio::test]
async fn list_commands_empty_domain_equals_all() {
    let orch = make_orchestrator();
    let params = make_list_params(Some(""));
    let output = orch.list_commands(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    let total = parsed["total"].as_u64().unwrap();
    assert!(total > 0);
}

static SKILL_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();

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
        Self {
            path,
            _guard: guard,
        }
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
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    let content = parsed["content"].as_str().unwrap();
    assert!(content.contains("Test Skill"));
}

#[tokio::test]
async fn read_skill_finds_existing_section() {
    let _skill = TempSkill::new("# Title\n\n## my-section\nSection content.\n\n## another\nOther.");
    let orch = make_orchestrator();
    let params = make_read_params(Some("my-section"));
    let output = orch.read_skill(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["section"].as_str(), Some("my-section"));
    assert!(parsed["content"]
        .as_str()
        .unwrap()
        .contains("Section content"));
}

#[tokio::test]
async fn read_skill_returns_error_for_nonexistent_section() {
    let _skill = TempSkill::new("# Title\n\n## real-section\ncontent");
    let orch = make_orchestrator();
    let params = make_read_params(Some("NonexistentSectionXYZ"));
    let output = orch.read_skill(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert!(parsed["error"].as_str().unwrap().contains("not found"));
}

#[tokio::test]
async fn read_skill_empty_section_equals_full() {
    let _skill = TempSkill::new("# Title\n\nsome content here");
    let orch = make_orchestrator();
    let params = make_read_params(Some(""));
    let output = orch.read_skill(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert!(parsed["content"].as_str().unwrap().contains("Title"));
}

#[tokio::test]
async fn execute_version_returns_pkg_version() {
    let orch = make_orchestrator();
    let params = make_exec_params("version", None);
    let output = orch.execute_command(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["name"].as_str(), Some("lint-arwaky"));
    assert!(!parsed["version"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn execute_unknown_action_returns_error() {
    let orch = make_orchestrator();
    let params = make_exec_params("__does_not_exist__", None);
    let output = orch.execute_command(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert!(parsed["error"].as_str().unwrap().contains("Unknown action"));
}

#[tokio::test]
async fn execute_doctor_runs_which_checks() {
    let orch = make_orchestrator();
    let params = make_exec_params("doctor", None);
    let output = orch.execute_command(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["status"].as_str(), Some("success"));
    let checks = parsed["checks"].as_array().unwrap();
    assert!(!checks.is_empty());
}

#[tokio::test]
async fn execute_init_returns_success() {
    let orch = make_orchestrator();
    let params = make_exec_params("init", None);
    let output = orch.execute_command(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["status"].as_str(), Some("success"));
}

#[tokio::test]
async fn execute_install_hook_returns_success() {
    let orch = make_orchestrator();
    let params = make_exec_params("install-hook", None);
    let output = orch.execute_command(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["status"].as_str(), Some("success"));
}

#[tokio::test]
async fn execute_uninstall_hook_returns_success() {
    let orch = make_orchestrator();
    let params = make_exec_params("uninstall-hook", None);
    let output = orch.execute_command(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["status"].as_str(), Some("success"));
}

#[tokio::test]
async fn execute_install_returns_success() {
    let orch = make_orchestrator();
    let params = make_exec_params("install", None);
    let output = orch.execute_command(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["status"].as_str(), Some("success"));
}

#[tokio::test]
async fn execute_config_show_returns_success() {
    let orch = make_orchestrator();
    let params = make_exec_params("config-show", None);
    let output = orch.execute_command(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["status"].as_str(), Some("success"));
}

#[tokio::test]
async fn execute_orphan_with_path() {
    let orch = make_orchestrator();
    let cwd = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let params = make_exec_params("orphan", Some(&cwd));
    let output = orch.execute_command(params).await;
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["status"].as_str(), Some("success"));
    assert_eq!(parsed["action"].as_str(), Some("orphan"));
    assert_eq!(parsed["path"].as_str(), Some(cwd.as_str()));
}
