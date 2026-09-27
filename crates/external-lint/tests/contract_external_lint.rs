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
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_protocol::ICommandExecutorProtocol;
use shared::external_lint::contract_external_lint_protocol::IExternalLintSelectorProtocol;
use shared::external_lint::contract_external_lint_protocol::IJsToolResolutionProtocol;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;

use mock_filesystem::MockFilesystem;

/// Backs the `ICommandExecutorProtocol` seam: raw execution plus the FR-006
/// `exec_cmd_*` error-mapping wrappers.
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
}

/// Backs the FR-007 `IJsToolResolutionProtocol` seam for the JS adapters.
struct MockJsResolution;
impl IJsToolResolutionProtocol for MockJsResolution {
    fn resolve_js_cmd(
        &self,
        _: &shared::filesystem::taxonomy_filesystem_vo::ToolName,
        _: Vec<String>,
        _: &FilePath,
    ) -> Option<Vec<String>> {
        None
    }
    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
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

// ─── Contract: ILinterAdapterProtocol ─────────────────────

fn assert_adapter_contract(adapter: &dyn ILinterAdapterProtocol, expected_name: &str) {
    // name() must return the declared adapter name
    assert_eq!(adapter.name().value(), expected_name);

    // scan() and apply_fix() must be callable (no panic)
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let _ = adapter.scan(&path);
    let _ = adapter.fix(&path);
}

#[test]
fn ruff_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::RuffAdapter::new(
        Arc::new(MockCmdExecutor),
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "ruff");
}

#[test]
fn bandit_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::BanditAdapter::new(
        Arc::new(MockCmdExecutor),
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "bandit");
}

#[test]
fn mypy_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::MyPyAdapter::new(
        Arc::new(MockCmdExecutor),
        None,
        Arc::new(MockFilesystem::new()),
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
        Arc::new(MockCmdExecutor),
        Arc::new(MockJsResolution),
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "eslint");
}

#[test]
fn prettier_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::PrettierAdapter::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockJsResolution),
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "prettier");
}

#[test]
fn tsc_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::TSCAdapter::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "tsc");
}

// ─── Contract: dyn trait object coercion ───────────────────

#[test]
fn all_adapters_coerce_to_dyn_protocol() {
    let _dyn_ruff: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::RuffAdapter::new(
            Arc::new(MockCmdExecutor),
            None,
            Arc::new(MockFilesystem::new()),
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_bandit: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::BanditAdapter::new(
            Arc::new(MockCmdExecutor),
            None,
            Arc::new(MockFilesystem::new()),
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_mypy: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::MyPyAdapter::new(
            Arc::new(MockCmdExecutor),
            None,
            Arc::new(MockFilesystem::new()),
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
            Arc::new(MockCmdExecutor),
            Arc::new(MockJsResolution),
            Arc::new(MockFilesystem::new()),
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_prettier: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::PrettierAdapter::new(
            Arc::new(MockCmdExecutor),
            Arc::new(MockJsResolution),
            Arc::new(MockFilesystem::new()),
            Arc::new(MockFilesystem::new()),
        ));
    let _dyn_tsc: Box<dyn ILinterAdapterProtocol> =
        Box::new(external_lint_lint_arwaky::TSCAdapter::new(
            Arc::new(MockCmdExecutor),
            Arc::new(MockFilesystem::new()),
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

// ─── Contract: ExternalLintExecutor implements ICommandExecutorProtocol ──

#[test]
fn external_lint_executor_implements_protocol() {
    let executor = external_lint_lint_arwaky::ExternalLintExecutor::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let _dyn_exec: &dyn ICommandExecutorProtocol = &executor;
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
        filesystem_io: Arc::new(MockFilesystem::new()),
        selector: Arc::new(
            external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::with_defaults(),
        ),
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let _dyn_agg: &dyn IExternalLintAggregate = &orchestrator;

    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = _dyn_agg
        .execute(shared::external_lint::ExternalLintRequest::scan_all(&path))
        .into_violations();
    assert!(result.values.is_empty()); // no adapters registered, so no results
    let names = _dyn_agg
        .execute(shared::external_lint::ExternalLintRequest::adapter_names())
        .into_adapter_names();
    assert!(names.is_empty());
}

// ─── Contract: ExternalLintExecutor implements ICargoDirProtocol (FR-008) ──

#[test]
fn external_lint_executor_implements_cargo_dir_protocol() {
    use shared::external_lint::ICargoDirProtocol;
    let executor = external_lint_lint_arwaky::ExternalLintExecutor::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let _dyn: &dyn ICargoDirProtocol = &executor;
    let wd = _dyn.resolve_cargo_working_dir(&FilePath::new("/tmp".to_string()).unwrap());
    assert!(!wd.value().is_empty());
}

// ─── Contract: LanguageDetector implements ILanguageDetectProtocol (FR-001) ──

#[test]
fn language_detector_implements_protocol() {
    use shared::external_lint::ILanguageDetectProtocol;
    let detector =
        external_lint_lint_arwaky::LanguageDetector::new(Arc::new(MockFilesystem::new()));
    let _dyn: &dyn ILanguageDetectProtocol = &detector;
    let (has_rust, has_python, has_js) =
        _dyn.detect_languages(&FilePath::new("/tmp".to_string()).unwrap());
    // No files were registered on the mock filesystem — all booleans are false.
    assert!(!has_rust && !has_python && !has_js);
}

// ─── Contract: OutputNormalizer implements INormalizeProtocol (FR-005) ──

#[test]
fn output_normalizer_implements_protocol() {
    use shared::external_lint::INormalizeProtocol;
    let normalizer = external_lint_lint_arwaky::OutputNormalizer;
    let _dyn: &dyn INormalizeProtocol = &normalizer;
    let root = FilePath::new("/tmp".to_string()).unwrap();
    let (results, warnings) = _dyn.normalize(
        "clippy",
        r#"[{"file":"main.rs","line":3,"column":1,"code":"dead_code","message":"unused","severity":"style"}]"#,
        &root,
    );
    assert_eq!(results.values.len(), 1);
    assert_eq!(results.values[0].code.to_string(), "clippy::dead_code");
    assert_eq!(results.values[0].file.value(), "/tmp/main.rs");
    assert!(warnings.is_empty());

    // Malformed JSON yields no results plus a warning, never a panic.
    let (empty, warns) = _dyn.normalize("clippy", "not json", &root);
    assert!(empty.values.is_empty());
    assert_eq!(warns.len(), 1);
}

// ─── Contract: ExternalLintExecutor implements IJsToolResolutionProtocol (FR-007) ──

#[test]
fn external_lint_executor_implements_js_resolution_protocol() {
    use shared::external_lint::IJsToolResolutionProtocol;
    use shared::filesystem::taxonomy_filesystem_vo::ToolName;
    let executor = external_lint_lint_arwaky::ExternalLintExecutor::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let _dyn: &dyn IJsToolResolutionProtocol = &executor;
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let tool = ToolName::new("eslint").unwrap();
    // The mock filesystem holds no local `node_modules/.bin/eslint`.
    assert!(
        _dyn.resolve_js_cmd(&tool, vec!["file.js".into(), "--fix".into()], &path)
            .is_none()
    );
    assert_eq!(_dyn.resolve_js_working_dir(&path).value(), "/tmp");
}
