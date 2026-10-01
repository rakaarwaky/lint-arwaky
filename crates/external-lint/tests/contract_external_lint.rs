// Contract tests — verify all adapters and services implement their declared protocol traits.
//
// These tests use the `dyn Trait` pattern to statically assert that each struct
// satisfies the protocol contract. They do NOT exercise business logic — that
// belongs in unit/integration tests.

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use std::sync::Arc;

use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared_common::taxonomy_message_vo::ComplianceStatus;
use shared_common::taxonomy_operation_error::LinterOperationError;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_response_data_vo::ResponseData;
use shared_common::taxonomy_severity_vo::Severity;
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::IJsToolResolutionProtocol;
use shared_external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared_external_lint::contract_external_lint_protocol::IExternalLintSelectorProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use std::collections::HashMap;

use mock_filesystem::MockFilesystem;

/// Backs the `ICommandExecutorProtocol` seam: raw execution plus the FR-006
/// `exec_cmd_*` error-mapping wrappers.
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
    ) -> Result<ResponseData, shared_common::taxonomy_operation_error::LinterOperationError> {
        Ok(ResponseData::default())
    }
    fn exec_cmd_adapter(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: AdapterName,
    ) -> Result<ResponseData, shared_common::taxonomy_operation_error::LinterOperationError> {
        Ok(ResponseData::default())
    }
}

/// Backs the FR-007 `IJsToolResolutionProtocol` seam for the JS adapters.
struct MockJsResolution;
impl IJsToolResolutionProtocol for MockJsResolution {
    fn resolve_js_cmd(
        &self,
        _: &shared_common::taxonomy_tool_name_vo::ToolName,
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
        _: &shared_common::taxonomy_tool_name_vo::ToolName,
        _: &str,
    ) -> Result<
        shared_common::taxonomy_message_vo::ComplianceStatus,
        shared_common::taxonomy_operation_error::LinterOperationError,
    > {
        Ok(shared_common::taxonomy_message_vo::ComplianceStatus::new(
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

// ─── Contract: StdioClient implements ICommandExecutorProtocol (utility) ──

#[test]
fn stdio_client_implements_command_executor_protocol() {
    let client = external_lint_lint_arwaky::StdioClient::new(
        shared_external_lint::taxonomy_duration_vo::Timeout::new(5.0),
    );
    let _dyn_client: &dyn ICommandExecutorProtocol = &client;
    // health_check must not panic
    let _ = _dyn_client.health_check();
}

// ─── Contract: ExternalLintExecutor implements ICommandExecutorProtocol ──

#[test]
fn external_lint_executor_implements_protocol() {
    let executor =
        external_lint_lint_arwaky::capabilities_command_executor::ExternalLintExecutor::new(
            Arc::new(MockCmdExecutor),
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
    let result = _dyn_sel.select_adapters(true, true, true, false);
    assert_eq!(result.len(), 9); // 3 rust + 3 python + 3 js (markdown off)
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
        .execute(shared_external_lint::ExternalLintRequest::scan_all(&path))
        .into_violations();
    assert!(result.values.is_empty()); // no adapters registered, so no results
    let names = _dyn_agg
        .execute(shared_external_lint::ExternalLintRequest::adapter_names())
        .into_adapter_names();
    assert!(names.is_empty());
}

// ─── Contract: ExternalLintExecutor implements ICargoDirProtocol (utility) ──

#[test]
fn external_lint_executor_implements_cargo_dir_protocol() {
    use shared_external_lint::ICargoDirProtocol;
    let executor =
        external_lint_lint_arwaky::capabilities_cargo_dir_resolver::ExternalLintExecutor::new(
            Arc::new(MockFilesystem::new()),
        );
    let _dyn: &dyn ICargoDirProtocol = &executor;
    let wd = _dyn.resolve_cargo_working_dir(&FilePath::new("/tmp".to_string()).unwrap());
    assert!(!wd.value().is_empty());
}

// ─── FR-001: language detection lives in the filesystem aggregate ──────────
//
// external-lint no longer owns a language-detection protocol or capability.
// The orchestrator asks the filesystem aggregate via `DetectProjectLanguages`;
// see `crates/filesystem` FR-Filesystem-005 for the contract.
//
// These tests make the delegation observable: a stub adapter that always
// reports one finding, paired with a mock filesystem that reports one language.
// With the language reported, the adapter runs and its finding appears; with it
// withheld, the selector declines and nothing appears. A regression that
// reverted to `ExternalLintContext::default()` — the bug this PR fixed — would
// fail the first assertion, because the default carries no languages.

/// Adapter that reports one finding whatever it is handed, so a test can tell
/// "the adapter ran" apart from "no adapter was selected".
struct StubAdapter {
    name: &'static str,
}

impl ILinterAdapterProtocol for StubAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw(self.name.to_string())
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        Ok(LintResultList::new(vec![LintResult::new_arch(
            &path.value(),
            1,
            "stub::finding",
            Severity::MEDIUM,
            "stub finding",
        )]))
    }

    fn fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::default())
    }
}

fn orchestrator_with(
    languages: shared_filesystem::taxonomy_filesystem_vo::ProjectLanguagesVO,
) -> (
    external_lint_lint_arwaky::agent_external_lint_orchestrator::ExternalLintOrchestrator,
    Arc<StubAdapter>,
) {
    use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
        ExternalLintDeps, ExternalLintOrchestrator,
    };

    let adapter = Arc::new(StubAdapter { name: "stub" });
    let mut adapters = HashMap::new();
    adapters.insert(
        "stub".to_string(),
        adapter.clone() as Arc<dyn ILinterAdapterProtocol>,
    );
    // The selector decides *which* adapters to run from the language flags, so
    // the stub has to be the name it selects for a Rust project — otherwise the
    // test would pass or fail on the selector's defaults rather than on the
    // delegation under test.
    let selector = external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::new(
        vec![AdapterName::raw("stub".to_string())],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let deps = ExternalLintDeps {
        adapters,
        filesystem: Arc::new(MockFilesystem::with_languages(languages)),
        filesystem_io: Arc::new(MockFilesystem::new()),
        selector: Arc::new(selector),
    };
    (ExternalLintOrchestrator::new(deps), adapter)
}

#[test]
fn orchestrator_runs_adapter_when_filesystem_reports_a_language() {
    use shared_filesystem::taxonomy_filesystem_vo::ProjectLanguagesVO;

    let mut languages = ProjectLanguagesVO::default();
    languages.has_rust = true;
    let (orchestrator, _adapter) = orchestrator_with(languages);

    let results = orchestrator.scan_all(&FilePath::new("/tmp".to_string()).unwrap());
    assert!(
        results
            .values
            .iter()
            .any(|r| r.code.to_string() == "stub::finding"),
        "the language reported by the filesystem seam must reach the selector and run \
         the adapter; got {:?}",
        results.values
    );
}

#[test]
fn orchestrator_skips_adapter_when_filesystem_reports_no_language() {
    // The same stub adapter, but the filesystem seam reports no language — the
    // control for the test above. Together the pair shows the result depends on
    // what the aggregate answered, not on the adapter map.
    let (orchestrator, _adapter) = orchestrator_with(Default::default());
    let results = orchestrator.scan_all(&FilePath::new("/tmp").unwrap());
    assert!(
        !results
            .values
            .iter()
            .any(|r| r.code.to_string() == "stub::finding"),
        "with no language reported no adapter should be selected; got {:?}",
        results.values
    );
}

// ─── Contract: OutputNormalizer implements INormalizeProtocol (FR-005) ──

#[test]
fn output_normalizer_implements_protocol() {
    use shared_external_lint::INormalizeProtocol;
    let normalizer = external_lint_lint_arwaky::OutputNormalizer;
    let _dyn: &dyn INormalizeProtocol = &normalizer;
    let root = FilePath::new("/tmp".to_string()).unwrap();
    let (results, warnings) = _dyn.normalize(
        &shared_common::taxonomy_tool_name_vo::ToolName::new("clippy"),
        r#"[{"file":"main.rs","line":3,"column":1,"code":"dead_code","message":"unused","severity":"style"}]"#,
        &root,
    );
    assert_eq!(results.values.len(), 1);
    assert_eq!(results.values[0].code.to_string(), "clippy::dead_code");
    assert_eq!(results.values[0].file.value(), "/tmp/main.rs");
    assert!(warnings.is_empty());

    // Malformed JSON yields no results plus a warning, never a panic.
    let (empty, warns) = _dyn.normalize(
        &shared_common::taxonomy_tool_name_vo::ToolName::new("clippy"),
        "not json",
        &root,
    );
    assert!(empty.values.is_empty());
    assert_eq!(warns.len(), 1);
}

// ─── Contract: ExternalLintExecutor implements IJsToolResolutionProtocol (utility) ──

#[test]
fn external_lint_executor_implements_js_resolution_protocol() {
    use shared_external_lint::IJsToolResolutionProtocol;
    let executor =
        external_lint_lint_arwaky::capabilities_js_tool_resolver::ExternalLintExecutor::new(
            Arc::new(MockCmdExecutor),
            Arc::new(MockFilesystem::new()),
            Arc::new(MockFilesystem::new()),
        );
    let _dyn: &dyn IJsToolResolutionProtocol = &executor;
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let tool = shared_common::taxonomy_tool_name_vo::ToolName::new("eslint");
    // The mock filesystem holds no local `node_modules/.bin/eslint`.
    assert!(
        _dyn.resolve_js_cmd(&tool, vec!["file.js".into(), "--fix".into()], &path)
            .is_none()
    );
    assert_eq!(_dyn.resolve_js_working_dir(&path).value(), "/tmp");
}
