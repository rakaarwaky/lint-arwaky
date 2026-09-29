// Contract tests — verify all adapters and services implement their declared protocol traits.
//
// These tests use the `dyn Trait` pattern to statically assert that each struct
// satisfies the protocol contract. They do NOT exercise business logic — that
// belongs in unit/integration tests.

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use std::collections::HashMap;
use std::sync::Arc;

use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_protocol::IAdapterScanProtocol;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;

use mock_filesystem::MockFilesystem;

// ─── Contract: ILinterAdapterProtocol ─────────────────────

fn assert_adapter_contract(adapter: &dyn ILinterAdapterProtocol, expected_name: &str) {
    assert_eq!(adapter.name().value(), expected_name);
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let _ = adapter.scan(&path);
    let _ = adapter.fix(&path);
}

#[test]
fn ruff_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::RuffAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "ruff");
}

#[test]
fn bandit_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::BanditAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "bandit");
}

#[test]
fn mypy_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::MyPyAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "mypy");
}

#[test]
fn clippy_adapter_implements_protocol() {
    let adapter =
        external_lint_lint_arwaky::RustLinterAdapter::new(None, Arc::new(MockFilesystem::new()));
    assert_adapter_contract(&adapter, "clippy");
}

#[test]
fn rustfmt_adapter_implements_protocol() {
    let adapter =
        external_lint_lint_arwaky::RustFmtAdapter::new(None, Arc::new(MockFilesystem::new()));
    assert_adapter_contract(&adapter, "rustfmt");
}

#[test]
fn cargo_audit_adapter_implements_protocol() {
    let adapter =
        external_lint_lint_arwaky::CargoAuditAdapter::new(Arc::new(MockFilesystem::new()));
    assert_adapter_contract(&adapter, "cargo-audit");
}

#[test]
fn eslint_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::ESLintAdapter::new(
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "eslint");
}

#[test]
fn prettier_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::PrettierAdapter::new(
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "prettier");
}

#[test]
fn tsc_adapter_implements_protocol() {
    let adapter = external_lint_lint_arwaky::TSCAdapter::new(
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    assert_adapter_contract(&adapter, "tsc");
}

// ─── Contract: dyn trait object coercion ───────────────────

#[test]
fn all_adapters_coerce_to_dyn_protocol() {
    let _io = Arc::new(MockFilesystem::new());
    let tr = Arc::new(MockFilesystem::new());

    let _dyn_ruff: Box<dyn ILinterAdapterProtocol> = Box::new(
        external_lint_lint_arwaky::RuffAdapter::new(None, _io.clone(), tr.clone()),
    );
    let _dyn_bandit: Box<dyn ILinterAdapterProtocol> = Box::new(
        external_lint_lint_arwaky::BanditAdapter::new(None, _io.clone(), tr.clone()),
    );
    let _dyn_mypy: Box<dyn ILinterAdapterProtocol> = Box::new(
        external_lint_lint_arwaky::MyPyAdapter::new(None, _io.clone(), tr.clone()),
    );
    let _dyn_clippy: Box<dyn ILinterAdapterProtocol> = Box::new(
        external_lint_lint_arwaky::RustLinterAdapter::new(None, tr.clone()),
    );
    let _dyn_fmt: Box<dyn ILinterAdapterProtocol> = Box::new(
        external_lint_lint_arwaky::RustFmtAdapter::new(None, tr.clone()),
    );
    let _dyn_audit: Box<dyn ILinterAdapterProtocol> = Box::new(
        external_lint_lint_arwaky::CargoAuditAdapter::new(tr.clone()),
    );
    let _dyn_eslint: Box<dyn ILinterAdapterProtocol> = Box::new(
        external_lint_lint_arwaky::ESLintAdapter::new(_io.clone(), tr.clone()),
    );
    let _dyn_prettier: Box<dyn ILinterAdapterProtocol> = Box::new(
        external_lint_lint_arwaky::PrettierAdapter::new(_io.clone(), tr.clone()),
    );
    let _dyn_tsc: Box<dyn ILinterAdapterProtocol> = Box::new(
        external_lint_lint_arwaky::TSCAdapter::new(_io.clone(), tr.clone()),
    );
}

// ─── Contract: ExternalLintOrchestrator implements IExternalLintAggregate ──

#[test]
fn orchestrator_implements_aggregate_protocol() {
    use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
        ExternalLintDeps, ExternalLintOrchestrator,
    };

    let deps = ExternalLintDeps {
        adapters: HashMap::new(),
        filesystem: Arc::new(MockFilesystem::new()),
        filesystem_io: Arc::new(MockFilesystem::new()),
        adapter_groups: None,
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let _dyn_agg: &dyn IExternalLintAggregate = &orchestrator;

    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = _dyn_agg
        .execute(shared::external_lint::ExternalLintRequest::scan_all(&path))
        .into_violations();
    assert!(result.values.is_empty());
    let names = _dyn_agg
        .execute(shared::external_lint::ExternalLintRequest::adapter_names())
        .into_adapter_names();
    assert!(names.is_empty());
}

// ─── Contract: ExternalLintOrchestrator implements IAdapterScanProtocol (FR-001) ──

#[test]
fn orchestrator_implements_scan_protocol() {
    use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
        ExternalLintDeps, ExternalLintOrchestrator,
    };
    use shared::external_lint::taxonomy_external_lint_vo::ExternalLintContext;

    let deps = ExternalLintDeps {
        adapters: HashMap::new(),
        filesystem: Arc::new(MockFilesystem::new()),
        filesystem_io: Arc::new(MockFilesystem::new()),
        adapter_groups: None,
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let _dyn_scan: &dyn IAdapterScanProtocol = &orchestrator;
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let context = ExternalLintContext::default();
    let _ = orchestrator.scan_all_with_context(&path, &context);
}

// ─── Contract: OutputNormalizer implements INormalizeProtocol (FR-003) ──

#[test]
fn output_normalizer_implements_protocol() {
    use shared::external_lint::INormalizeProtocol;
    let normalizer = external_lint_lint_arwaky::OutputNormalizer;
    let _dyn: &dyn INormalizeProtocol = &normalizer;
    let root = FilePath::new("/tmp".to_string()).unwrap();
    let (results, warnings) = _dyn.normalize(
        &shared::common::taxonomy_tool_name_vo::ToolName::new("clippy"),
        r#"[{"file":"main.rs","line":3,"column":1,"code":"dead_code","message":"unused","severity":"style"}]"#,
        &root,
    );
    assert_eq!(results.values.len(), 1);
    assert_eq!(results.values[0].code.to_string(), "clippy::dead_code");
    assert_eq!(results.values[0].file.value(), "/tmp/main.rs");
    assert!(warnings.is_empty());

    // Malformed JSON yields no results plus a warning, never a panic.
    let (empty, warns) = _dyn.normalize(
        &shared::common::taxonomy_tool_name_vo::ToolName::new("clippy"),
        "not json",
        &root,
    );
    assert!(empty.values.is_empty());
    assert_eq!(warns.len(), 1);
}
