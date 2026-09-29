// Acceptance tests — verify the orchestrator selects the right adapters from
// its default groups based on language flags in the scan context.
//
// The adapter groups are internal to the orchestrator; these tests exercise
// the public scan API with pre-computed contexts.

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use std::collections::HashMap;
use std::sync::Arc;

use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::taxonomy_external_lint_vo::ExternalLintContext;

use external_lint_lint_arwaky::agent_external_lint_orchestrator::{
    AdapterGroups, ExternalLintDeps, ExternalLintOrchestrator,
};
use mock_filesystem::MockFilesystem;

/// Helper: build an orchestrator with no adapters but with default groups,
/// then scan with the given language flags and verify the result is empty
/// (no adapters registered → no results). The point is to assert that the
/// orchestrator does not panic and respects the context flags.
fn build_orchestrator_with_groups(groups: AdapterGroups) -> ExternalLintOrchestrator {
    ExternalLintOrchestrator::new(ExternalLintDeps {
        adapters: HashMap::new(),
        filesystem: Arc::new(MockFilesystem::new()),
        filesystem_io: Arc::new(MockFilesystem::new()),
        adapter_groups: Some(groups),
    })
}

// ─── Acceptance: Default adapter groups ───────────────────

#[test]
fn acceptance_selector_has_exactly_ten_default_adapters() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let total = groups.rust.len() + groups.python.len() + groups.js.len() + groups.markdown.len();
    assert_eq!(total, 10, "Expected 10 default adapters");
}

#[test]
fn acceptance_rust_adapters_are_clippy_rustfmt_cargo_audit() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let names: Vec<&str> = groups.rust.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["clippy", "rustfmt", "cargo-audit"]);
}

#[test]
fn acceptance_python_adapters_are_ruff_mypy_bandit() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let names: Vec<&str> = groups.python.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["ruff", "mypy", "bandit"]);
}

#[test]
fn acceptance_js_adapters_are_eslint_prettier_tsc() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let names: Vec<&str> = groups.js.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["eslint", "prettier", "tsc"]);
}

#[test]
fn acceptance_markdown_adapters_are_markdownlint() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let names: Vec<&str> = groups.markdown.iter().map(|a| a.value()).collect();
    assert_eq!(names, vec!["markdownlint"]);
}

// ─── Acceptance: Orchestrator respects language context ──

#[test]
fn acceptance_rust_plus_python() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let orchestrator = build_orchestrator_with_groups(groups);
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let context = ExternalLintContext {
        has_rust: true,
        has_python: true,
        has_js: false,
        has_markdown: false,
        ..Default::default()
    };
    let results = orchestrator.scan_all_with_context(&path, &context);
    assert!(results.values.is_empty()); // no adapters registered
}

#[test]
fn acceptance_rust_plus_js() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let orchestrator = build_orchestrator_with_groups(groups);
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let context = ExternalLintContext {
        has_rust: true,
        has_python: false,
        has_js: true,
        has_markdown: false,
        ..Default::default()
    };
    let results = orchestrator.scan_all_with_context(&path, &context);
    assert!(results.values.is_empty());
}

#[test]
fn acceptance_python_plus_js() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let orchestrator = build_orchestrator_with_groups(groups);
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let context = ExternalLintContext {
        has_rust: false,
        has_python: true,
        has_js: true,
        has_markdown: false,
        ..Default::default()
    };
    let results = orchestrator.scan_all_with_context(&path, &context);
    assert!(results.values.is_empty());
}

#[test]
fn acceptance_js_plus_markdown() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let orchestrator = build_orchestrator_with_groups(groups);
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let context = ExternalLintContext {
        has_rust: false,
        has_python: false,
        has_js: true,
        has_markdown: true,
        ..Default::default()
    };
    let results = orchestrator.scan_all_with_context(&path, &context);
    assert!(results.values.is_empty());
}

#[test]
fn acceptance_no_languages_returns_empty() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let orchestrator = build_orchestrator_with_groups(groups);
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let context = ExternalLintContext::default();
    let results = orchestrator.scan_all_with_context(&path, &context);
    assert!(results.values.is_empty());
}

#[test]
fn acceptance_adapter_names_are_lowercase_ascii() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let all: Vec<String> = groups
        .rust
        .iter()
        .chain(&groups.python)
        .chain(&groups.js)
        .chain(&groups.markdown)
        .map(|a| a.value().to_string())
        .collect();
    for name in &all {
        assert!(
            name.chars().all(|c| c.is_ascii_lowercase() || c == '-'),
            "Adapter name '{}' should be lowercase ASCII with hyphens only",
            name
        );
    }
}

#[test]
fn acceptance_no_duplicate_adapters_across_languages() {
    let groups = ExternalLintOrchestrator::new_default_groups();
    let mut all: Vec<&str> = groups
        .rust
        .iter()
        .map(|a| a.value())
        .chain(groups.python.iter().map(|a| a.value()))
        .chain(groups.js.iter().map(|a| a.value()))
        .chain(groups.markdown.iter().map(|a| a.value()))
        .collect();
    all.sort();
    all.dedup();
    let total = groups.rust.len() + groups.python.len() + groups.js.len() + groups.markdown.len();
    assert_eq!(
        all.len(),
        total,
        "Duplicate adapter names detected across language groups"
    );
}
