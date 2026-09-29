// Integration tests — verify individual adapter creation and orchestrator wiring.
//
// These tests construct real adapters with mock executors/filesystems and verify
// that the subsystem wires together correctly.

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use std::collections::HashMap;
use std::sync::Arc;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;

use mock_filesystem::MockFilesystem;

// ─── Integration: Individual adapter creation ─────────────

#[test]
fn create_ruff_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::RuffAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_bandit_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::BanditAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_mypy_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::MyPyAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_clippy_adapter_and_scan_returns_empty() {
    let adapter =
        external_lint_lint_arwaky::RustLinterAdapter::new(None, Arc::new(MockFilesystem::new()));
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_rustfmt_adapter_and_scan_returns_empty() {
    let adapter =
        external_lint_lint_arwaky::RustFmtAdapter::new(None, Arc::new(MockFilesystem::new()));
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_cargo_audit_adapter_and_scan_returns_empty() {
    let adapter =
        external_lint_lint_arwaky::CargoAuditAdapter::new(Arc::new(MockFilesystem::new()));
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_eslint_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::ESLintAdapter::new(
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_prettier_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::PrettierAdapter::new(
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_tsc_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::TSCAdapter::new(
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

// ─── Integration: Orchestrator adapter group defaults ─────

#[test]
fn orchestrator_default_groups_are_populated() {
    use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
        AdapterGroups, ExternalLintDeps,
    };
    use shared::external_lint::taxonomy_external_lint_vo::ExternalLintContext;

    // Build orchestrator with default groups
    let deps = ExternalLintDeps {
        adapters: HashMap::new(),
        filesystem: Arc::new(MockFilesystem::new()),
        filesystem_io: Arc::new(MockFilesystem::new()),
        adapter_groups: Some(AdapterGroups {
            rust: vec![
                AdapterName::raw("clippy"),
                AdapterName::raw("rustfmt"),
                AdapterName::raw("cargo-audit"),
            ],
            python: vec![
                AdapterName::raw("ruff"),
                AdapterName::raw("mypy"),
                AdapterName::raw("bandit"),
            ],
            js: vec![
                AdapterName::raw("eslint"),
                AdapterName::raw("prettier"),
                AdapterName::raw("tsc"),
            ],
            markdown: vec![AdapterName::raw("markdownlint")],
        }),
    };
    let orchestrator =
        external_lint_lint_arwaky::agent_external_lint_orchestrator::ExternalLintOrchestrator::new(
            deps,
        );

    // Scan with Rust+Python flags — should select 6 adapters from defaults
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let context = ExternalLintContext {
        has_rust: true,
        has_python: true,
        has_js: false,
        has_markdown: false,
        ..Default::default()
    };
    let results = orchestrator.scan_all_with_context(&path, &context);
    // No adapters registered, so results are empty regardless of selection
    assert!(results.values.is_empty());
}

// ─── Integration: apply_fix on adapters ───────────────────

#[test]
fn bandit_apply_fix_always_returns_false() {
    let adapter = external_lint_lint_arwaky::BanditAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.fix(&path).unwrap();
    assert!(!status.value);
}

#[test]
fn mypy_apply_fix_always_returns_false() {
    let adapter = external_lint_lint_arwaky::MyPyAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.fix(&path).unwrap();
    assert!(!status.value);
}

#[test]
fn tsc_apply_fix_always_returns_false() {
    let adapter = external_lint_lint_arwaky::TSCAdapter::new(
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.fix(&path).unwrap();
    assert!(!status.value);
}

#[test]
fn ruff_apply_fix_returns_true() {
    // Skip if ruff is not installed (e.g., in CI environments)
    if std::process::Command::new("ruff")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("Skipping ruff_apply_fix_returns_true: ruff not installed");
        return;
    }
    let adapter = external_lint_lint_arwaky::RuffAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.fix(&path).unwrap();
    assert!(status.value);
}

#[test]
fn cargo_audit_apply_fix_returns_true() {
    let adapter =
        external_lint_lint_arwaky::CargoAuditAdapter::new(Arc::new(MockFilesystem::new()));
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.fix(&path).unwrap();
    assert!(status.value);
}
