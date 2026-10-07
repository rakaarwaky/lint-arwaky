// E2E tests — full pipeline: detect languages → select adapters → verify adapter names.
//
// These tests simulate the complete flow that ExternalLintOrchestrator follows:
// language detection → adapter selection → adapter name verification.
// We don't actually run the linters (that would require installed tools),
// but we verify the end-to-end wiring from detection to selection.

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::MockFilesystem;

use std::collections::HashMap;
use std::sync::Arc;

use shared_common::taxonomy_adapter_error::LinterOperationError;
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_job_vo::ResponseData;
use shared_common::taxonomy_path_vo::FilePath;
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::IJsToolResolutionProtocol;
use shared_external_lint::contract_external_lint_protocol::IExternalLintSelectorProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;

use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
    ExternalLintDeps, ExternalLintOrchestrator,
};
use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;

// ─── Mocks ────────────────────────────────────────────────

struct MockCmdExecutor;
impl ICommandExecutorProtocol for MockCmdExecutor {
    fn execute_command(
        &self,
        _: shared_common::taxonomy_common_vo::PatternList,
        _: FilePath,
        _: Option<shared_external_lint::taxonomy_duration_vo::Timeout>,
    ) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::default())
    }
    fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::default())
    }
    fn exec_cmd_scan(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: Option<AdapterName>,
        _: &FilePath,
    ) -> Result<ResponseData, LinterOperationError> {
        Ok(ResponseData::default())
    }
    fn exec_cmd_adapter(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: AdapterName,
    ) -> Result<ResponseData, LinterOperationError> {
        Ok(ResponseData::default())
    }
}

/// Minimal `IJsToolResolutionProtocol` so the markdown adapter can be wired
/// without a real filesystem.
struct MockJsResolution;
impl IJsToolResolutionProtocol for MockJsResolution {
    fn resolve_js_cmd(
        &self,
        _: &shared_common::taxonomy_adapter_name_vo::ToolName,
        _: Vec<String>,
        _: &FilePath,
    ) -> Option<Vec<String>> {
        Some(vec!["markdownlint".to_string()])
    }
    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    fn js_apply_fix(
        &self,
        _: &FilePath,
        _: &shared_common::taxonomy_adapter_name_vo::ToolName,
        _: &str,
    ) -> Result<shared_common::taxonomy_message_vo::ComplianceStatus, LinterOperationError> {
        Ok(shared_common::taxonomy_message_vo::ComplianceStatus::new(
            false,
        ))
    }
}

// ─── E2E: Rust-only project ───────────────────────────────

#[test]
fn e2e_rust_only_project_selects_clippy_rustfmt_audit() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, false, false, false);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["clippy", "rustfmt", "cargo-audit"]);
}

// ─── E2E: Python-only project ─────────────────────────────

#[test]
fn e2e_python_only_project_selects_ruff_mypy_bandit() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, true, false, false);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["ruff", "mypy", "bandit"]);
}

// ─── E2E: JS-only project ─────────────────────────────────

#[test]
fn e2e_js_only_project_selects_eslint_prettier_tsc() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, true, false);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["eslint", "prettier", "tsc"]);
}

// ─── E2E: Markdown-only project ───────────────────────────

#[test]
fn e2e_markdown_only_project_selects_markdownlint() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, false, true);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["markdownlint"]);
}

// ─── E2E: Mixed project ───────────────────────────────────

#[test]
fn e2e_mixed_project_selects_all_ten() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, true, true);
    assert_eq!(selected.len(), 10);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"rustfmt"));
    assert!(names.contains(&"cargo-audit"));
    assert!(names.contains(&"ruff"));
    assert!(names.contains(&"mypy"));
    assert!(names.contains(&"bandit"));
    assert!(names.contains(&"eslint"));
    assert!(names.contains(&"prettier"));
    assert!(names.contains(&"tsc"));
    assert!(names.contains(&"markdownlint"));
}

// ─── E2E: No languages detected ───────────────────────────

#[test]
fn e2e_no_languages_detected_selects_nothing() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, false, false);
    assert!(selected.is_empty());
}

// ─── E2E: Full pipeline — Rust+Python project ─────────────

#[test]
fn e2e_full_pipeline_rust_python() {
    // Step 1: Select adapters (simulating language detection)
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, false, false);
    let selected_names: Vec<String> = selected.iter().map(|a| a.value().to_string()).collect();

    // Step 2: Build orchestrator with matching adapters
    let lint_exec: Arc<dyn ICommandExecutorProtocol> = Arc::new(MockCmdExecutor);
    let files = vec!["main.rs".to_string(), "app.py".to_string()];
    let _fs_arc: Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate> =
        Arc::new(MockFilesystem::with_files(files.clone()));
    let tr_arc: Arc<dyn shared_filesystem::IToolResolutionProtocol> =
        Arc::new(MockFilesystem::with_files(files.clone()));
    let io_arc: Arc<dyn shared_filesystem::IFileSystemIOProtocol> =
        Arc::new(MockFilesystem::with_files(files.clone()));

    let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
    for name in &selected_names {
        match name.as_str() {
            "ruff" => {
                adapters.insert(
                    name.clone(),
                    Arc::new(external_lint_lint_arwaky::RuffAdapter::new(
                        lint_exec.clone(),
                        None,
                        io_arc.clone(),
                        tr_arc.clone(),
                    )),
                );
            }
            "mypy" => {
                adapters.insert(
                    name.clone(),
                    Arc::new(external_lint_lint_arwaky::MyPyAdapter::new(
                        lint_exec.clone(),
                        None,
                        io_arc.clone(),
                        tr_arc.clone(),
                    )),
                );
            }
            "bandit" => {
                adapters.insert(
                    name.clone(),
                    Arc::new(external_lint_lint_arwaky::BanditAdapter::new(
                        lint_exec.clone(),
                        None,
                        io_arc.clone(),
                        tr_arc.clone(),
                    )),
                );
            }
            // Skip Rust adapters (need ICommandExecutorProtocol mock)
            _ => {}
        }
    }

    // Step 3: Verify adapter_names matches
    let deps = ExternalLintDeps {
        adapters,
        filesystem: Arc::new(MockFilesystem::with_files(files)),
        filesystem_io: io_arc.clone(),
        selector: Arc::new(
            external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::with_defaults(),
        ),
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let adapter_names = orchestrator.adapter_names();

    let registered: Vec<&str> = adapter_names.iter().map(|a| a.value()).collect();
    assert!(registered.contains(&"ruff"));
    assert!(registered.contains(&"mypy"));
    assert!(registered.contains(&"bandit"));

    // Step 4: scan_all with empty adapters (no Rust ones registered)
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let results = orchestrator.scan_all(&path);
    assert!(results.values.is_empty()); // adapters scan empty dirs → no findings
}

// ─── E2E: Full pipeline — Markdown project ────────────────

#[test]
fn e2e_full_pipeline_markdown_only() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, false, true);
    let selected_names: Vec<String> = selected.iter().map(|a| a.value().to_string()).collect();

    let lint_exec: Arc<dyn ICommandExecutorProtocol> = Arc::new(MockCmdExecutor);
    let files = vec!["README.md".to_string(), "CHANGELOG.md".to_string()];
    let fs_arc: Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate> =
        Arc::new(MockFilesystem::with_files(files.clone()));
    let tr_arc: Arc<dyn shared_filesystem::IToolResolutionProtocol> =
        Arc::new(MockFilesystem::with_files(files.clone()));
    let io_arc: Arc<dyn shared_filesystem::IFileSystemIOProtocol> =
        Arc::new(MockFilesystem::with_files(files.clone()));

    let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
    for name in &selected_names {
        if name == "markdownlint" {
            adapters.insert(
                name.clone(),
                Arc::new(external_lint_lint_arwaky::MarkdownLintAdapter::new(
                    lint_exec.clone(),
                    Arc::new(MockJsResolution),
                    io_arc.clone(),
                    tr_arc.clone(),
                )),
            );
        }
    }

    let deps = ExternalLintDeps {
        adapters,
        filesystem: fs_arc.clone(),
        filesystem_io: io_arc.clone(),
        selector: Arc::new(selector),
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let adapter_names = orchestrator.adapter_names();
    let registered: Vec<&str> = adapter_names.iter().map(|a| a.value()).collect();
    assert!(registered.contains(&"markdownlint"));

    let path = FilePath::new("/tmp".to_string()).unwrap();
    let results = orchestrator.scan_all(&path);
    assert!(results.values.is_empty()); // mock executor returns no violations
}
