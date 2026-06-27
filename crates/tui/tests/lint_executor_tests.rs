use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::code_analysis::taxonomy_analysis_vo::{
    FileDefinitionMap, GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap,
};
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::contract_scanner_provider_port::IScannerProviderPort;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_vo::ScopeRef;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use shared::mcp_server::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, ProjectLanguageVO, WriteConfigResult,
};
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_action_flags_vo::ActionFlags;
use std::sync::Arc;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

struct MockCodeAnalysis {
    results: LintResultList,
    score: f64,
    critical: bool,
}
impl MockCodeAnalysis {
    fn empty() -> Self {
        Self {
            results: LintResultList::new(vec![]),
            score: 95.0,
            critical: false,
        }
    }
    fn with_violations(count: usize, critical: bool) -> Self {
        let results: Vec<LintResult> = (0..count)
            .map(|i| LintResult {
                file: FilePath::new(format!("file_{}.rs", i)).unwrap_or_default(),
                line: LineNumber::new(1),
                column: Default::default(),
                code: ErrorCode::raw("TEST001"),
                message: LintMessage::new(format!("violation {}", i)),
                source: None,
                severity: if critical && i == 0 {
                    Severity::CRITICAL
                } else {
                    Severity::LOW
                },
                enclosing_scope: Some(ScopeRef {
                    name: DescriptionVO::new(String::new()),
                    kind: DescriptionVO::new(String::new()),
                    file: None,
                    start_line: None,
                    end_line: None,
                }),
                related_locations: Default::default(),
            })
            .collect();
        Self {
            results: LintResultList::new(results),
            score: if critical { 50.0 } else { 85.0 },
            critical,
        }
    }
}
impl ICodeAnalysisAggregate for MockCodeAnalysis {
    fn run_code_analysis(&self, _path: &str) -> LintResultList {
        self.results.clone()
    }
    fn run_code_analysis_dir(&self, _path: &str) -> LintResultList {
        self.results.clone()
    }
    fn run_code_analysis_path(&self, _path: &str) -> Vec<LintResult> {
        self.results.values.clone()
    }
    fn calc_score(&self, _results: &[LintResult]) -> f64 {
        self.score
    }
    fn check_critical(&self, _results: &[LintResult]) -> bool {
        self.critical
    }
    fn format_report(&self, _results: &LintResultList, _root: &str) -> String {
        "mock report".to_string()
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

fn make_executor(mock: MockCodeAnalysis) -> LintExecutor {
    LintExecutor::new(Arc::new(mock))
}

struct MockFixOrchestrator {
    output: String,
}
impl MockFixOrchestrator {
    fn new(output: &str) -> Self {
        Self {
            output: output.to_string(),
        }
    }
}
impl LintFixOrchestratorAggregate for MockFixOrchestrator {
    fn execute(&self, _path: &FilePath) -> FixResult {
        FixResult::new(DescriptionVO::new(self.output.clone()), None)
    }
}

fn make_executor_with_fix(mock: MockCodeAnalysis, fix_mock: MockFixOrchestrator) -> LintExecutor {
    LintExecutor::new(Arc::new(mock)).with_fix(Arc::new(fix_mock))
}

// ---------------------------------------------------------------------------
// MockSetupAggregate for testing install()
// ---------------------------------------------------------------------------
struct MockSetupAggregate {
    language: String,
    py_success: bool,
    js_success: bool,
}

impl MockSetupAggregate {
    fn new(language: &str) -> Self {
        Self {
            language: language.to_string(),
            py_success: true,
            js_success: true,
        }
    }
}

#[async_trait::async_trait]
impl SetupManagementAggregate for MockSetupAggregate {
    fn check_http(
        &self,
        _url: &shared::cli_commands::taxonomy_protocol_vo::TransportUrlVO,
    ) -> SuccessStatus {
        SuccessStatus::new(true)
    }
    fn generate_env(
        &self,
        _transport: &shared::cli_commands::taxonomy_protocol_vo::TransportProtocol,
        _home: &shared::common::taxonomy_path_vo::DirectoryPath,
    ) -> EnvContentVO {
        EnvContentVO::new("mock env".to_string())
    }
    fn generate_mcp_config(
        &self,
        _transport: &shared::cli_commands::taxonomy_protocol_vo::TransportProtocol,
    ) -> McpConfigVO {
        McpConfigVO::new(std::collections::HashMap::new())
    }
    fn mcp_config_claude(
        &self,
        _transport: &shared::cli_commands::taxonomy_protocol_vo::TransportProtocol,
    ) -> McpConfigVO {
        McpConfigVO::new(std::collections::HashMap::new())
    }
    fn mcp_config_hermes(
        &self,
        _transport: &shared::cli_commands::taxonomy_protocol_vo::TransportProtocol,
    ) -> McpConfigVO {
        McpConfigVO::new(std::collections::HashMap::new())
    }
    fn mcp_config_vscode(
        &self,
        _transport: &shared::cli_commands::taxonomy_protocol_vo::TransportProtocol,
    ) -> McpConfigVO {
        McpConfigVO::new(std::collections::HashMap::new())
    }
    async fn install_python_adapters(&self) -> SuccessStatus {
        SuccessStatus::new(self.py_success)
    }
    async fn install_javascript_adapters(&self, _sudo: bool) -> SuccessStatus {
        SuccessStatus::new(self.js_success)
    }
    fn detect_language(&self) -> ProjectLanguageVO {
        ProjectLanguageVO::new(&self.language)
    }
    fn get_config_template(&self, _language: &str) -> &'static str {
        "mock template"
    }
    fn write_config_file(&self, _filename: &str, _content: &str) -> WriteConfigResult {
        Ok(DescriptionVO::new("wrote mock".to_string()))
    }
    fn create_global_config_dir(&self) -> CreateConfigDirResult {
        Ok(std::path::PathBuf::from("/tmp/mock"))
    }
    fn file_exists(&self, _path: &str) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// Basic executor tests
// ---------------------------------------------------------------------------
#[test]
fn test_check_with_no_violations() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let flags = ActionFlags::default();
    let result = executor.check("/root", &flags);
    assert!(result.success);
    assert_eq!(result.violation_count, 0);
    assert!(result.output.contains("No violations found"));
}

#[test]
fn test_check_with_violations() {
    let executor = make_executor(MockCodeAnalysis::with_violations(3, false));
    let flags = ActionFlags::default();
    let result = executor.check("/root", &flags);
    assert!(result.success);
    assert_eq!(result.violation_count, 3);
    assert!(result.output.contains("3 violation"));
}

#[test]
fn test_scan() {
    let executor = make_executor(MockCodeAnalysis::with_violations(2, false));
    let result = executor.scan("/root");
    assert!(result.success);
    assert_eq!(result.violation_count, 2);
}

#[test]
fn test_fix_dry_run() {
    let executor = make_executor(MockCodeAnalysis::with_violations(5, false));
    let mut flags = ActionFlags::default();
    flags.dry_run = true;
    let result = executor.fix("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("DRY-RUN"));
}

#[test]
fn test_fix_live() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let flags = ActionFlags::default();
    let result = executor.fix("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("LIVE"));
}

#[test]
fn test_ci_pass() {
    let executor = make_executor(MockCodeAnalysis {
        score: 90.0,
        critical: false,
        ..MockCodeAnalysis::empty()
    });
    let mut flags = ActionFlags::default();
    flags.threshold = 80;
    let result = executor.ci("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("PASS"));
}

#[test]
fn test_ci_fail_low_score() {
    let executor = make_executor(MockCodeAnalysis {
        score: 50.0,
        critical: false,
        ..MockCodeAnalysis::empty()
    });
    let mut flags = ActionFlags::default();
    flags.threshold = 80;
    let result = executor.ci("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("FAIL"));
}

#[test]
fn test_ci_fail_critical() {
    let executor = make_executor(MockCodeAnalysis {
        score: 95.0,
        critical: true,
        ..MockCodeAnalysis::empty()
    });
    let mut flags = ActionFlags::default();
    flags.threshold = 80;
    let result = executor.ci("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("FAIL"));
}

#[test]
fn test_orphan() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.orphan("/root");
    assert!(result.success);
    assert!(result.output.contains("Orphan detection"));
}

#[test]
fn test_security() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.security("/root");
    assert!(result.success);
    assert!(result.output.contains("Security scan"));
}

#[test]
fn test_doctor() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.doctor();
    assert!(result.success);
    assert!(result.output.contains("Environment Diagnostics"));
}

#[test]
fn test_version() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.version();
    assert!(result.success);
    assert!(result.output.contains("Lint Arwaky"));
}

#[test]
fn test_format_results_empty() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let results = LintResultList::new(vec![]);
    let output = executor.format_results(&results);
    assert_eq!(output, "No violations found.");
}

#[test]
fn test_format_results_with_violations() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let results = LintResultList::new(vec![LintResult {
        file: FilePath::new("test.rs".to_string()).unwrap_or_default(),
        line: LineNumber::new(10),
        column: Default::default(),
        code: ErrorCode::raw("E001"),
        message: LintMessage::new("test message"),
        source: None,
        severity: Severity::LOW,
        enclosing_scope: None,
        related_locations: Default::default(),
    }]);
    let output = executor.format_results(&results);
    assert!(output.contains("1 violation"));
    assert!(output.contains("test.rs:10"));
    assert!(output.contains("test message"));
}

#[test]
fn test_fix_with_orchestrator_live() {
    let executor = make_executor_with_fix(
        MockCodeAnalysis::with_violations(3, false),
        MockFixOrchestrator::new("Fixed 2 violations automatically (1 remaining)"),
    );
    let flags = ActionFlags::default();
    let result = executor.fix("/some/path", &flags);
    assert!(result.success);
    assert!(result.output.contains("[LIVE]"));
    assert!(result.output.contains("Fixed 2 violations"));
}

#[test]
fn test_fix_with_orchestrator_dry_run() {
    let executor = make_executor_with_fix(
        MockCodeAnalysis::with_violations(5, false),
        MockFixOrchestrator::new("Dry-run: would fix 3 violations"),
    );
    let mut flags = ActionFlags::default();
    flags.dry_run = true;
    let result = executor.fix("/some/path", &flags);
    assert!(result.success);
    assert!(result.output.contains("[DRY-RUN]"));
    assert!(result.output.contains("Dry-run: would fix 3 violations"));
}

#[test]
fn test_fix_without_orchestrator_shows_stub() {
    let executor = make_executor(MockCodeAnalysis::with_violations(2, false));
    let flags = ActionFlags::default();
    let result = executor.fix("/root", &flags);
    assert!(result.success);
    assert!(result
        .output
        .contains("Fix application requires FixOrchestrator"));
    assert!(result.output.contains("lint-arwaky-cli fix"));
}

#[test]
fn test_fix_without_orchestrator_dry_run_shows_stub() {
    let executor = make_executor(MockCodeAnalysis::with_violations(1, false));
    let mut flags = ActionFlags::default();
    flags.dry_run = true;
    let result = executor.fix("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("[DRY-RUN]"));
    assert!(result
        .output
        .contains("Fix application requires FixOrchestrator"));
}

#[test]
fn test_with_fix_preserves_code_analysis() {
    let executor = make_executor_with_fix(
        MockCodeAnalysis::with_violations(3, false),
        MockFixOrchestrator::new("ok"),
    );
    let flags = ActionFlags::default();
    let result = executor.check("/root", &flags);
    assert!(result.success);
    assert_eq!(result.violation_count, 3);
}

#[test]
fn test_orphan_stub_without_container() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.orphan("/nonexistent");
    assert!(result.success);
    assert!(result.output.contains("Use CLI"));
    assert!(result.output.contains("lint-arwaky-cli orphan"));
}

#[test]
fn test_dependencies_stub() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.dependencies("/nonexistent");
    assert!(result.success);
    assert!(result.output.contains("Use CLI"));
    assert!(result.output.contains("lint-arwaky-cli dependencies"));
}

#[test]
fn test_doctor_stub() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.doctor();
    assert!(result.success);
    assert!(result.output.contains("Use CLI"));
    assert!(result.output.contains("lint-arwaky-cli maintenance doctor"));
}

// ---------------------------------------------------------------------------
// Install tests
// ---------------------------------------------------------------------------

#[test]
fn test_install_stub_without_setup_aggregate() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let flags = ActionFlags::default();
    let result = executor.install(&flags);
    assert!(result.success);
    assert!(result.output.contains("Adapter dependency installation"));
    assert!(result.output.contains("lint-arwaky-cli setup install"));
}

#[test]
fn test_install_with_setup_aggregate_python_project() {
    let mock = MockSetupAggregate::new("rust");
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis::empty()))
        .with_fix(Arc::new(MockFixOrchestrator::new("ok")))
        .with_setup(Arc::new(mock));
    let flags = ActionFlags::default();
    let result = executor.install(&flags);
    assert!(result.success, "Expected success, got: {}", result.output);
    assert!(result.output.contains("Adapter dependency installation"));
    assert!(result.output.contains("Detected language: rust"));
    assert!(result.output.contains("Python (ruff, mypy, bandit)"));
    assert!(result.output.contains("[OK]"));
    assert!(result
        .output
        .contains("JavaScript (eslint, prettier, typescript)"));
    assert!(result.output.contains("All adapter dependencies installed"));
}

#[test]
fn test_install_with_setup_aggregate_js_project() {
    let mock = MockSetupAggregate::new("javascript");
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis::empty()))
        .with_fix(Arc::new(MockFixOrchestrator::new("ok")))
        .with_setup(Arc::new(mock));
    let flags = ActionFlags::default();
    let result = executor.install(&flags);
    assert!(result.success, "Expected success, got: {}", result.output);
    assert!(result.output.contains("Detected language: javascript"));
    assert!(result.output.contains("Python (ruff, mypy, bandit)"));
    assert!(result
        .output
        .contains("JavaScript (eslint, prettier, typescript)"));
    assert!(result.output.contains("All adapter dependencies installed"));
}

#[test]
fn test_install_with_setup_aggregate_python_failure() {
    let mut mock = MockSetupAggregate::new("rust");
    mock.py_success = false;
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis::empty()))
        .with_fix(Arc::new(MockFixOrchestrator::new("ok")))
        .with_setup(Arc::new(mock));
    let flags = ActionFlags::default();
    let result = executor.install(&flags);
    assert!(
        !result.success,
        "Expected failure when py install fails, got: {}",
        result.output
    );
    assert!(result.output.contains("[FAIL] Python"));
    assert!(result.output.contains("Some adapter(s) failed to install"));
}

// ---------------------------------------------------------------------------
// Config show tests
// ---------------------------------------------------------------------------

#[test]
fn test_config_show_without_config_orchestrator_uses_fallback() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.config_show();
    assert!(result.success);
    assert!(result.output.contains("Active Configuration"));
    // Without orchestrator, config_show does CWD fallback search.
    // It either finds a real config or falls back to built-in defaults.
    assert!(
        result.output.contains("Source:") || result.output.contains("built-in defaults"),
        "expected source or built-in defaults in output: {}",
        result.output
    );
}

// ---------------------------------------------------------------------------
// Doctor with real MaintenanceCommandsAggregate wiring
// ---------------------------------------------------------------------------

struct MockMaintenanceAggregate;

#[async_trait::async_trait]
impl shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate
    for MockMaintenanceAggregate
{
    async fn stats(
        &self,
        _: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::project_setup::taxonomy_stats_vo::MaintenanceStatsVO {
        unimplemented!()
    }
    async fn clean(&self) {}
    async fn update(&self) {}
    async fn doctor(&self) -> shared::project_setup::taxonomy_doctor_vo::DoctorResultVO {
        unimplemented!()
    }
    async fn cancel(&self, _: shared::mcp_server::taxonomy_action_vo::JobId) {}
    async fn diagnose_toolchain(
        &self,
    ) -> shared::project_setup::taxonomy_doctor_vo::ToolchainDiagnostics {
        // Deterministic fake — no process spawning
        shared::project_setup::taxonomy_doctor_vo::ToolchainDiagnostics {
            rust_tools: vec![shared::project_setup::taxonomy_doctor_vo::ToolStatus {
                name: "cargo".to_string(),
                status: "OK".to_string(),
                version: "cargo 1.80.0".to_string(),
            }],
            python_tools: vec![shared::project_setup::taxonomy_doctor_vo::ToolStatus {
                name: "python3".to_string(),
                status: "OK".to_string(),
                version: "Python 3.11.0".to_string(),
            }],
            js_tools: vec![shared::project_setup::taxonomy_doctor_vo::ToolStatus {
                name: "node".to_string(),
                status: "OK".to_string(),
                version: "v20.0.0".to_string(),
            }],
            vcs_tools: vec![shared::project_setup::taxonomy_doctor_vo::ToolStatus {
                name: "git".to_string(),
                status: "OK".to_string(),
                version: "git 2.40.0".to_string(),
            }],
            binary_path: "/usr/bin/lint-arwaky".to_string(),
        }
    }
    async fn run_security_scan(
        &self,
        _: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::project_setup::taxonomy_doctor_vo::SecurityScanReport {
        unimplemented!()
    }
    async fn run_dependency_report(
        &self,
        _: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Result<shared::project_setup::taxonomy_doctor_vo::DependencyReport, String> {
        unimplemented!()
    }
}

#[test]
fn test_doctor_with_maintenance_returns_real_diagnostics() {
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis::empty()))
        .with_maintenance(Arc::new(MockMaintenanceAggregate));
    let result = executor.doctor();
    assert!(result.success, "doctor() failed: {}", result.output);
    assert!(result.output.contains("Environment Diagnostics"));
    // Should contain mock tool versions, NOT the CLI stub
    assert!(
        result.output.contains("cargo 1.80.0"),
        "Missing cargo version: {}",
        result.output
    );
    assert!(result.output.contains("Python 3.11.0"));
    assert!(result.output.contains("All required tools OK"));
    assert!(
        !result.output.contains("lint-arwaky-cli maintenance doctor"),
        "Should NOT show CLI stub when maintenance is wired"
    );
}
// ---------------------------------------------------------------------------
// Git hook tests
// ---------------------------------------------------------------------------

struct MockHookPort {
    install_ok: bool,
    uninstall_ok: bool,
}

impl MockHookPort {
    fn success() -> Self {
        Self {
            install_ok: true,
            uninstall_ok: true,
        }
    }
    fn install_error() -> Self {
        Self {
            install_ok: false,
            uninstall_ok: true,
        }
    }
    fn uninstall_error() -> Self {
        Self {
            install_ok: true,
            uninstall_ok: false,
        }
    }
}

impl shared::git_hooks::contract_manager_port::IHookManagerPort for MockHookPort {
    fn install_pre_commit(
        &self,
        _executable_path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Result<
        shared::mcp_server::taxonomy_job_vo::SuccessStatus,
        shared::git_hooks::taxonomy_hook_error::GitHookError,
    > {
        if self.install_ok {
            Ok(shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(
                true,
            ))
        } else {
            Err(shared::git_hooks::taxonomy_hook_error::GitHookError::new(
                shared::common::taxonomy_message_vo::LintMessage::new(
                    "hook installation failed".to_string(),
                ),
            ))
        }
    }
    fn uninstall_pre_commit(
        &self,
    ) -> Result<
        shared::mcp_server::taxonomy_job_vo::SuccessStatus,
        shared::git_hooks::taxonomy_hook_error::GitHookError,
    > {
        if self.uninstall_ok {
            Ok(shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(
                true,
            ))
        } else {
            Err(shared::git_hooks::taxonomy_hook_error::GitHookError::new(
                shared::common::taxonomy_message_vo::LintMessage::new(
                    "hook removal failed".to_string(),
                ),
            ))
        }
    }
}

#[test]
fn test_install_hook_with_port_success() {
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis::empty()))
        .with_hook_port(Arc::new(MockHookPort::success()));
    let result = executor.install_hook();
    assert!(result.success);
    assert!(result.output.contains("installed successfully"));
}

#[test]
fn test_install_hook_with_port_error() {
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis::empty()))
        .with_hook_port(Arc::new(MockHookPort::install_error()));
    let result = executor.install_hook();
    assert!(!result.success);
    assert!(result
        .output
        .contains("Git pre-commit hook installation failed"));
    assert!(result.output.contains("hook installation failed"));
}

#[test]
fn test_install_hook_without_port_falls_back_to_stub() {
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis::empty()));
    let result = executor.install_hook();
    assert!(result.success);
    assert!(result.output.contains("lint-arwaky-cli install-hook"));
}

#[test]
fn test_uninstall_hook_with_port_success() {
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis::empty()))
        .with_hook_port(Arc::new(MockHookPort::success()));
    let result = executor.uninstall_hook();
    assert!(result.success);
    assert!(result.output.contains("removed successfully"));
}

#[test]
fn test_uninstall_hook_with_port_error() {
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis::empty()))
        .with_hook_port(Arc::new(MockHookPort::uninstall_error()));
    let result = executor.uninstall_hook();
    assert!(!result.success);
    assert!(result.output.contains("Git pre-commit hook removal failed"));
    assert!(result.output.contains("hook removal failed"));
}

#[test]
fn test_uninstall_hook_without_port_falls_back_to_stub() {
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis::empty()));
    let result = executor.uninstall_hook();
    assert!(result.success);
    assert!(result.output.contains("lint-arwaky-cli uninstall-hook"));
}

// --- Mocks for orphan detection ---

struct MockOrphanAggregate {
    orphan_count: usize,
}
impl MockOrphanAggregate {
    fn with_orphans(count: usize) -> Self {
        Self {
            orphan_count: count,
        }
    }
}
impl IOrphanAggregate for MockOrphanAggregate {
    fn build_orphan_graph_context(
        &self,
        _files: &[String],
        _root_dir: &str,
    ) -> GraphAnalysisContext {
        GraphAnalysisContext::new(
            ImportGraph::new(std::collections::HashMap::new()),
            InboundLinkMap::new(std::collections::HashMap::new()),
            InheritanceMap::new(std::collections::HashMap::new()),
            FileDefinitionMap::new(std::collections::HashMap::new()),
        )
    }
    fn identify_orphan_entry_points(&self, _files: &[String]) -> std::collections::HashSet<String> {
        std::collections::HashSet::new()
    }
    fn check_orphans(
        &self,
        _layer_detector: &dyn ILayerDetectionAggregate,
        _files: &[String],
        _root_dir: &str,
    ) -> Vec<LintResult> {
        (0..self.orphan_count)
            .map(|i| LintResult {
                file: FilePath::new(format!("orphan_{}.rs", i)).unwrap_or_default(),
                line: LineNumber::new(1),
                column: Default::default(),
                code: ErrorCode::raw("ORPHAN001"),
                message: LintMessage::new(format!("orphan file {}", i)),
                source: Some(shared::common::taxonomy_adapter_name_vo::AdapterName::raw(
                    "orphan_detector",
                )),
                severity: Severity::MEDIUM,
                enclosing_scope: None,
                related_locations: Default::default(),
            })
            .collect()
    }
}

struct MockLayerDetector;
impl ILayerDetectionAggregate for MockLayerDetector {
    fn detect_layer(&self, _file_path: &str, _root_dir: &str) -> Option<String> {
        Some("taxonomy".to_string())
    }
    fn get_layer_def(
        &self,
        _layer: &str,
    ) -> Option<shared::common::taxonomy_definition_vo::LayerDefinition> {
        None
    }
    fn get_orphan_entry_points(&self) -> Vec<String> {
        vec![]
    }
}

struct MockScannerProvider;
impl IScannerProviderPort for MockScannerProvider {
    fn scan_directory(
        &self,
        _path: &DirectoryPath,
    ) -> Result<FilePathList, shared::common::taxonomy_filesystem_error::FileSystemError> {
        Ok(FilePathList::new(vec![
            FilePath::new("src/main.rs".to_string()).unwrap_or_default(),
            FilePath::new("src/lib.rs".to_string()).unwrap_or_default(),
        ]))
    }
    fn get_ignored_files(&self) -> FilePathList {
        FilePathList::new(vec![])
    }
}

fn make_executor_with_orphan(
    mock: MockCodeAnalysis,
    orphan_agg: MockOrphanAggregate,
) -> LintExecutor {
    LintExecutor::new(Arc::new(mock)).with_orphan(
        Arc::new(orphan_agg),
        Arc::new(MockLayerDetector),
        Arc::new(MockScannerProvider),
    )
}

#[test]
fn test_orphan_with_real_detection() {
    let executor = make_executor_with_orphan(
        MockCodeAnalysis::empty(),
        MockOrphanAggregate::with_orphans(3),
    );
    let result = executor.orphan("/some/path");
    assert!(result.success);
    assert!(result.output.contains("Orphan detection"));
    assert!(result.output.contains("Scanned 2 files"));
    assert!(result.output.contains("Found 3 orphan(s)"));
    assert!(result.output.contains("orphan file 0"));
    assert!(result.output.contains("ORPHAN001"));
}

#[test]
fn test_orphan_with_real_detection_no_orphans() {
    let executor = make_executor_with_orphan(
        MockCodeAnalysis::empty(),
        MockOrphanAggregate::with_orphans(0),
    );
    let result = executor.orphan("/some/path");
    assert!(result.success);
    assert!(result.output.contains("No orphan files detected."));
}

#[test]
fn test_orphan_real_vs_stub_distinction() {
    // With orphan aggregate wired in -> real detection
    let real = make_executor_with_orphan(
        MockCodeAnalysis::empty(),
        MockOrphanAggregate::with_orphans(1),
    );
    let result_real = real.orphan("/path");
    assert!(result_real.output.contains("Scanned"));

    // Without orphan aggregate -> stub
    let stub = make_executor(MockCodeAnalysis::empty());
    let result_stub = stub.orphan("/path");
    assert!(result_stub.output.contains("lint-arwaky-cli orphan"));
}

struct MockMultiProjectOrchestrator {
    workspaces: Vec<shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo>,
}

#[async_trait::async_trait]
impl shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate
    for MockMultiProjectOrchestrator
{
    async fn discover_workspaces(
        &self,
        _root: &FilePath,
    ) -> Vec<shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo> {
        self.workspaces.clone()
    }
}

#[test]
fn test_scan_with_multi_project_discovery() {
    let mock_config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    let ws = shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo::new(
        FilePath::new("/root/member1".to_string()).unwrap_or_default(),
        "rust".to_string(),
        mock_config,
    );
    let orch = MockMultiProjectOrchestrator {
        workspaces: vec![ws],
    };
    let executor =
        make_executor(MockCodeAnalysis::empty()).with_multi_project_orchestrator(Arc::new(orch));

    let result = executor.scan("/root");
    assert!(result.success);
}
