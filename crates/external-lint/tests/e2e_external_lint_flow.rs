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

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::IExternalLintExecutorProtocol;
use shared::external_lint::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
    ExternalLintDeps, ExternalLintOrchestrator,
};
use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;

// ─── Mocks ────────────────────────────────────────────────

struct MockLintExecutor;
impl IExternalLintExecutorProtocol for MockLintExecutor {
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
    fn js_apply_fix(
        &self,
        _: &FilePath,
        _: &str,
        _: &str,
    ) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

// ─── E2E: Rust-only project ───────────────────────────────

#[test]
fn e2e_rust_only_project_selects_clippy_rustfmt_audit() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, false, false);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["clippy", "rustfmt", "cargo-audit"]);
}

// ─── E2E: Python-only project ─────────────────────────────

#[test]
fn e2e_python_only_project_selects_ruff_mypy_bandit() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, true, false);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["ruff", "mypy", "bandit"]);
}

// ─── E2E: JS-only project ─────────────────────────────────

#[test]
fn e2e_js_only_project_selects_eslint_prettier_tsc() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, true);
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["eslint", "prettier", "tsc"]);
}

// ─── E2E: Mixed project ───────────────────────────────────

#[test]
fn e2e_mixed_project_selects_all_nine() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, true);
    assert_eq!(selected.len(), 9);
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
}

// ─── E2E: No languages detected ───────────────────────────

#[test]
fn e2e_no_languages_detected_selects_nothing() {
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, false);
    assert!(selected.is_empty());
}

// ─── E2E: Full pipeline — Rust+Python project ─────────────

#[test]
fn e2e_full_pipeline_rust_python() {
    // Step 1: Select adapters (simulating language detection)
    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, false);
    let selected_names: Vec<String> = selected.iter().map(|a| a.value().to_string()).collect();

    // Step 2: Build orchestrator with matching adapters
    let lint_exec: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(MockLintExecutor);
    let files = vec!["main.rs".to_string(), "app.py".to_string()];
    let fs_arc: Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate> =
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
                        fs_arc.clone(),
                    )),
                );
            }
            "mypy" => {
                adapters.insert(
                    name.clone(),
                    Arc::new(external_lint_lint_arwaky::MyPyAdapter::new(
                        lint_exec.clone(),
                        None,
                        fs_arc.clone(),
                    )),
                );
            }
            "bandit" => {
                adapters.insert(
                    name.clone(),
                    Arc::new(external_lint_lint_arwaky::BanditAdapter::new(
                        lint_exec.clone(),
                        None,
                        fs_arc.clone(),
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
