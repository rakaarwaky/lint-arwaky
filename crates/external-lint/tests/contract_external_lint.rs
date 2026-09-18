// Contract tests — verify all adapters and services implement their declared protocol traits.
//
// These tests use the `dyn Trait` pattern to statically assert that each struct
// satisfies the protocol contract. They do NOT exercise business logic — that
// belongs in unit/integration tests.

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use std::sync::Arc;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::external_lint::contract_executor_protocol::ICommandExecutorProtocol;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

use mock_filesystem::MockFilesystem;

struct MockLintExecutor;
impl IExternalLintExecutorProtocol for MockLintExecutor {
    fn exec_cmd_scan(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: Option<AdapterName>,
        _: &FilePath,
    ) -> Result<ResponseData, shared::common::taxonomy_operation_error::LinterOperationError> {
        Ok(ResponseData::default())
    }
    fn exec_cmd_adapter(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: AdapterName,
    ) -> Result<ResponseData, shared::common::taxonomy_operation_error::LinterOperationError> {
        Ok(ResponseData::default())
    }
    fn js_apply_fix(
        &self,
        _: &FilePath,
        _: &str,
        _: &str,
    ) -> Result<
        shared::common::taxonomy_message_vo::ComplianceStatus,
        shared::common::taxonomy_operation_error::LinterOperationError,
    > {
        Ok(shared::common::taxonomy_message_vo::ComplianceStatus::new(
            false,
        ))
    }
}

struct MockCmdExecutor;
impl ICommandExecutorProtocol for MockCmdExecutor {
    fn execute_command(
        &self,
        _: shared::common::taxonomy_common_vo::PatternList,
        _: FilePath,
        _: Option<shared::common::taxonomy_duration_vo::Timeout>,
    ) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::default())
    }
    fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::default())
    }
}

// ─── Contract: ILinterAdapterProtocol ─────────────────────

fn assert_adapter_contract(adapter: &dyn ILinterAdapterProtocol, expected_name: &str) {
    // name() must return the declared adapter name
    assert_eq!(adapter.name().value(), expected_name);

    // scan() and apply_fix() must be callable (no panic)
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let _ = adapter.scan(&path);
    let _ = adapter.apply_fix(&path);
}

#[test]
fn ruff_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::RuffAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "ruff");
}

#[test]
fn bandit_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::BanditAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "bandit");
}

#[test]
fn mypy_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::MyPyAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "mypy");
}

#[test]
fn clippy_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::RustLinterAdapter::new(
        Arc::new(MockCmdExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "clippy");
}

#[test]
fn rustfmt_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::RustFmtAdapter::new(
        Arc::new(MockCmdExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "rustfmt");
}

#[test]
fn cargo_audit_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::CargoAuditAdapter::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "cargo-audit");
}

#[test]
fn eslint_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::ESLintAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "eslint");
}

#[test]
fn prettier_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::PrettierAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "prettier");
}

#[test]
fn tsc_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::TSCAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "tsc");
}

// ─── Contract: dyn trait object coercion ───────────────────

#[test]
fn all_adapters_coerce_to_dyn_protocol() {
    let _dyn_ruff: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::RuffAdapter::new(
            Arc::new(MockLintExecutor),
            None,
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_bandit: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::BanditAdapter::new(
            Arc::new(MockLintExecutor),
            None,
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_mypy: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::MyPyAdapter::new(
            Arc::new(MockLintExecutor),
            None,
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_clippy: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::RustLinterAdapter::new(
            Arc::new(MockCmdExecutor),
            None,
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_fmt: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::RustFmtAdapter::new(
            Arc::new(MockCmdExecutor),
            None,
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_audit: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::CargoAuditAdapter::new(
            Arc::new(MockCmdExecutor),
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_eslint: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::ESLintAdapter::new(
            Arc::new(MockLintExecutor),
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_prettier: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::PrettierAdapter::new(
            Arc::new(MockLintExecutor),
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_tsc: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::TSCAdapter::new(
            Arc::new(MockLintExecutor),
            Arc::new(MockFilesystem::new()),
        ));
}

// ─── Contract: StdioClient implements ICommandExecutorProtocol ──

#[test]
fn stdio_client_implements_command_executor_protocol() {
    let client = external_lint_lint_arwaky::StdioClient::new(
        shared::common::taxonomy_duration_vo::Timeout::new(5.0),
    );
    let _dyn_client: &dyn ICommandExecutorProtocol = &client;
    // health_check must not panic
    let _ = _dyn_client.health_check();
}

// ─── Contract: ExternalLintExecutor implements IExternalLintExecutorProtocol ──

#[test]
fn external_lint_executor_implements_protocol() {
    let executor = external_lint_lint_arwaky::ExternalLintExecutor::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem::new()),
    );
    let _dyn_exec: &dyn IExternalLintExecutorProtocol = &executor;
    // verify callable methods
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let _ = _dyn_exec.exec_cmd_adapter(
        vec!["echo".into()],
        path.clone(),
        1.0,
        AdapterName::raw("test"),
    );
    let _ = _dyn_exec.exec_cmd_scan(vec!["echo".into()], path.clone(), 1.0, None, &path);
}

// ─── Contract: CapabilitiesExternalLintSelector implements IExternalLintSelectorProtocol ──

#[test]
fn selector_implements_protocol() {
    let selector = external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::with_defaults();
    let _dyn_sel: &dyn IExternalLintSelectorProtocol = &selector;
    let result = _dyn_sel.select_adapters(true, true, true);
    assert_eq!(result.len(), 9); // 3 rust + 3 python + 3 js
}

// ─── Contract: ExternalLintOrchestrator implements IExternalLintAggregate ──

#[test]
fn orchestrator_implements_aggregate_protocol() {
    use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
        ExternalLintDeps, ExternalLintOrchestrator,
    };
    use std::collections::HashMap;

    let deps = ExternalLintDeps {
        adapters: HashMap::new(),
        filesystem: Arc::new(MockFilesystem::new()),
        selector: Arc::new(
            external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::with_defaults(),
        ),
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let _dyn_agg: &dyn IExternalLintAggregate = &orchestrator;

    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = _dyn_agg.scan_all(&path);
    assert!(result.values.is_empty()); // no adapters registered, so no results
    let names = _dyn_agg.adapter_names();
    assert!(names.is_empty());
}
