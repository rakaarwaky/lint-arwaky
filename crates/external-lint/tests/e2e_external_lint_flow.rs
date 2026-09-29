// E2E tests — full pipeline: detect languages → select adapters → verify adapter names.
//
// These tests simulate the complete flow that ExternalLintOrchestrator follows:
// language detection → adapter selection → scan orchestration.
// We don't actually run the linters (that would require installed tools),
// but we verify the end-to-end wiring from detection to selection.

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::MockFilesystem;

use std::collections::HashMap;
use std::sync::Arc;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared::external_lint::taxonomy_external_lint_vo::ExternalLintContext;

use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
    AdapterGroups, ExternalLintDeps, ExternalLintOrchestrator,
};

// ─── E2E: Default adapter groups ──────────────────────────

#[test]
fn e2e_rust_adapters_default_to_clippy_rustfmt_audit() {
    // Verify the default Rust group contains the expected adapters
    let groups = ExternalLintOrchestrator::new_default_groups();
    let rust_names: Vec<&str> = groups.rust.iter().map(|a| a.value()).collect();
    assert_eq!(rust_names, vec!["clippy", "rustfmt", "cargo-audit"]);
}

#[test]
fn e2e_python_adapters_default_to_ruff_mypy_bandit() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let py_names: Vec<&str> = groups.python.iter().map(|a| a.value()).collect();
    assert_eq!(py_names, vec!["ruff", "mypy", "bandit"]);
}

#[test]
fn e2e_js_adapters_default_to_eslint_prettier_tsc() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let js_names: Vec<&str> = groups.js.iter().map(|a| a.value()).collect();
    assert_eq!(js_names, vec!["eslint", "prettier", "tsc"]);
}

#[test]
fn e2e_markdown_adapters_default_to_markdownlint() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let md_names: Vec<&str> = groups.markdown.iter().map(|a| a.value()).collect();
    assert_eq!(md_names, vec!["markdownlint"]);
}

#[test]
fn e2e_mixed_project_defaults_to_ten_adapters() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let total = groups.rust.len() + groups.python.len() + groups.js.len() + groups.markdown.len();
    assert_eq!(total, 10);
}

// ─── E2E: Full pipeline — Rust+Python project ─────────────

#[test]
fn e2e_full_pipeline_rust_python() {
    let files = vec!["main.rs".to_string(), "app.py".to_string()];
    let fs_arc: Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate> =
        Arc::new(MockFilesystem::with_files(files.clone()));
    let tr_arc: Arc<dyn shared::filesystem::IToolResolutionProtocol> =
        Arc::new(MockFilesystem::with_files(files.clone()));
    let io_arc: Arc<dyn shared::filesystem::IFileSystemIOProtocol> =
        Arc::new(MockFilesystem::with_files(files.clone()));

    let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
    adapters.insert(
        "ruff".to_string(),
        Arc::new(external_lint_lint_arwaky::RuffAdapter::new(
            None,
            io_arc.clone(),
            tr_arc.clone(),
        )),
    );
    adapters.insert(
        "mypy".to_string(),
        Arc::new(external_lint_lint_arwaky::MyPyAdapter::new(
            None,
            io_arc.clone(),
            tr_arc.clone(),
        )),
    );
    adapters.insert(
        "bandit".to_string(),
        Arc::new(external_lint_lint_arwaky::BanditAdapter::new(
            None,
            io_arc.clone(),
            tr_arc.clone(),
        )),
    );

    let deps = ExternalLintDeps {
        adapters,
        filesystem: fs_arc.clone(),
        filesystem_io: io_arc.clone(),
        adapter_groups: None,
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let adapter_names = orchestrator.adapter_names();
    let registered: Vec<&str> = adapter_names.iter().map(|a| a.value()).collect();
    assert!(registered.contains(&"ruff"));
    assert!(registered.contains(&"mypy"));
    assert!(registered.contains(&"bandit"));

    let path = FilePath::new("/tmp".to_string()).unwrap();
    let results = orchestrator.scan_all(&path);
    assert!(results.values.is_empty());
}

// ─── E2E: Full pipeline — Markdown project ────────────────

#[test]
fn e2e_full_pipeline_markdown_only() {
    let files = vec!["README.md".to_string(), "CHANGELOG.md".to_string()];
    let fs_arc: Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate> =
        Arc::new(MockFilesystem::with_files(files.clone()));
    let tr_arc: Arc<dyn shared::filesystem::IToolResolutionProtocol> =
        Arc::new(MockFilesystem::with_files(files.clone()));
    let io_arc: Arc<dyn shared::filesystem::IFileSystemIOProtocol> =
        Arc::new(MockFilesystem::with_files(files.clone()));

    let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
    adapters.insert(
        "markdownlint".to_string(),
        Arc::new(external_lint_lint_arwaky::MarkdownLintAdapter::new(
            io_arc.clone(),
            tr_arc.clone(),
        )),
    );

    let deps = ExternalLintDeps {
        adapters,
        filesystem: fs_arc.clone(),
        filesystem_io: io_arc.clone(),
        adapter_groups: None,
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let adapter_names = orchestrator.adapter_names();
    let registered: Vec<&str> = adapter_names.iter().map(|a| a.value()).collect();
    assert!(registered.contains(&"markdownlint"));

    let path = FilePath::new("/tmp".to_string()).unwrap();
    let results = orchestrator.scan_all(&path);
    assert!(results.values.is_empty());
}

// ─── E2E: Context-driven scan — no languages ──────────────

#[test]
fn e2e_no_languages_selects_nothing() {
    let deps = ExternalLintDeps {
        adapters: HashMap::new(),
        filesystem: Arc::new(MockFilesystem::new()),
        filesystem_io: Arc::new(MockFilesystem::new()),
        adapter_groups: None,
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let context = ExternalLintContext {
        has_rust: false,
        has_python: false,
        has_js: false,
        has_markdown: false,
        ..Default::default()
    };
    let results = orchestrator.scan_all_with_context(&path, &context);
    assert!(results.values.is_empty());
}

// ─── E2E: Context-driven scan — Rust only ─────────────────

#[test]
fn e2e_rust_only_project_with_context() {
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
            python: vec![],
            js: vec![],
            markdown: vec![],
        }),
    };
    let orchestrator = ExternalLintOrchestrator::new(deps);
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let context = ExternalLintContext {
        has_rust: true,
        has_python: false,
        has_js: false,
        has_markdown: false,
        ..Default::default()
    };
    let results = orchestrator.scan_all_with_context(&path, &context);
    assert!(results.values.is_empty());
}
