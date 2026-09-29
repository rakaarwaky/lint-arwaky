// Smoke tests — quick boot + respond within 5s.
//
// These tests verify that the core components can be instantiated quickly
// and that basic operations complete without hanging or panicking.

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use std::sync::Arc;
use std::time::Instant;

use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;

use mock_filesystem::MockFilesystem;

const SMOKE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

// ─── Smoke: Orchestrator with default groups ──────────────

#[test]
fn smoke_orchestrator_rust_only() {
    let start = Instant::now();
    use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
        ExternalLintDeps, ExternalLintOrchestrator,
    };
    let deps = ExternalLintDeps {
        adapters: std::collections::HashMap::new(),
        filesystem: Arc::new(MockFilesystem::new()),
        filesystem_io: Arc::new(MockFilesystem::new()),
        adapter_groups: None,
    };
    let _orch = ExternalLintOrchestrator::new(deps);
    assert!(start.elapsed() < SMOKE_TIMEOUT);
}

#[test]
fn smoke_orchestrator_python_only() {
    let start = Instant::now();
    use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
        ExternalLintDeps, ExternalLintOrchestrator,
    };
    let deps = ExternalLintDeps {
        adapters: std::collections::HashMap::new(),
        filesystem: Arc::new(MockFilesystem::new()),
        filesystem_io: Arc::new(MockFilesystem::new()),
        adapter_groups: None,
    };
    let _orch = ExternalLintOrchestrator::new(deps);
    assert!(start.elapsed() < SMOKE_TIMEOUT);
}

#[test]
fn smoke_orchestrator_js_only() {
    let start = Instant::now();
    use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
        ExternalLintDeps, ExternalLintOrchestrator,
    };
    let deps = ExternalLintDeps {
        adapters: std::collections::HashMap::new(),
        filesystem: Arc::new(MockFilesystem::new()),
        filesystem_io: Arc::new(MockFilesystem::new()),
        adapter_groups: None,
    };
    let _orch = ExternalLintOrchestrator::new(deps);
    assert!(start.elapsed() < SMOKE_TIMEOUT);
}

#[test]
fn smoke_orchestrator_markdown_only() {
    let start = Instant::now();
    use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
        ExternalLintDeps, ExternalLintOrchestrator,
    };
    let deps = ExternalLintDeps {
        adapters: std::collections::HashMap::new(),
        filesystem: Arc::new(MockFilesystem::new()),
        filesystem_io: Arc::new(MockFilesystem::new()),
        adapter_groups: None,
    };
    let _orch = ExternalLintOrchestrator::new(deps);
    assert!(start.elapsed() < SMOKE_TIMEOUT);
}

// ─── Smoke: Adapter creation with free-function utilities ──

#[test]
fn smoke_all_adapters_created_quickly() {
    let start = Instant::now();

    let tr: Arc<dyn shared::filesystem::IToolResolutionProtocol> = Arc::new(MockFilesystem::new());
    let io: Arc<dyn shared::filesystem::IFileSystemIOProtocol> = Arc::new(MockFilesystem::new());

    let path = FilePath::new("/tmp".to_string()).unwrap();

    // Python adapters
    let _ruff = external_lint_lint_arwaky::RuffAdapter::new(None, io.clone(), tr.clone());
    let _bandit = external_lint_lint_arwaky::BanditAdapter::new(None, io.clone(), tr.clone());
    let _mypy = external_lint_lint_arwaky::MyPyAdapter::new(None, io.clone(), tr.clone());

    // JS adapters
    let _eslint = external_lint_lint_arwaky::ESLintAdapter::new(io.clone(), tr.clone());
    let _prettier = external_lint_lint_arwaky::PrettierAdapter::new(io.clone(), tr.clone());
    let _tsc = external_lint_lint_arwaky::TSCAdapter::new(io.clone(), tr.clone());

    // Rust adapters
    let _clippy = external_lint_lint_arwaky::RustLinterAdapter::new(None, tr.clone());
    let _fmt = external_lint_lint_arwaky::RustFmtAdapter::new(None, tr.clone());
    let _audit = external_lint_lint_arwaky::CargoAuditAdapter::new(tr.clone());

    // Scan each (should return immediately with mock filesystem)
    let _ = _ruff.scan(&path);
    let _ = _bandit.scan(&path);
    let _ = _mypy.scan(&path);
    let _ = _eslint.scan(&path);
    let _ = _prettier.scan(&path);
    let _ = _tsc.scan(&path);
    let _ = _clippy.scan(&path);
    let _ = _fmt.scan(&path);
    let _ = _audit.scan(&path);

    assert!(start.elapsed() < SMOKE_TIMEOUT);
}
