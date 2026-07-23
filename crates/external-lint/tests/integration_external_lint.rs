// PURPOSE: Integration tests — DI container wiring, adapter registration,
// and orchestrator composition using the real ExternalLintContainer.

use external_lint_lint_arwaky::*;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use std::collections::HashMap;
use std::sync::Arc;

// ─── Container Wiring ─────────────────────────────────────

#[test]
fn container_creates_aggregate() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    // Aggregate should be a valid Arc
    assert!(Arc::strong_count(&aggregate) >= 1);
}

#[test]
fn container_default_creates_aggregate() {
    let container = ExternalLintContainer::default();
    let aggregate = container.aggregate();
    assert!(Arc::strong_count(&aggregate) >= 1);
}

#[test]
fn container_new_default_creates_aggregate() {
    let container = ExternalLintContainer::new_default();
    let aggregate = container.aggregate();
    assert!(Arc::strong_count(&aggregate) >= 1);
}

// ─── Adapter Registration ─────────────────────────────────

#[test]
fn orchestrator_has_all_nine_adapters_registered() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();
    assert_eq!(names.len(), 9);
}

#[test]
fn orchestrator_contains_rust_adapters() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();
    assert!(names.contains(&AdapterName::raw("clippy")));
    assert!(names.contains(&AdapterName::raw("rustfmt")));
    assert!(names.contains(&AdapterName::raw("cargo-audit")));
}

#[test]
fn orchestrator_contains_python_adapters() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();
    assert!(names.contains(&AdapterName::raw("ruff")));
    assert!(names.contains(&AdapterName::raw("mypy")));
    assert!(names.contains(&AdapterName::raw("bandit")));
}

#[test]
fn orchestrator_contains_js_adapters() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();
    assert!(names.contains(&AdapterName::raw("eslint")));
    assert!(names.contains(&AdapterName::raw("prettier")));
    assert!(names.contains(&AdapterName::raw("tsc")));
}

// ─── Orchestrator with Custom Adapters ────────────────────

#[test]
fn orchestrator_with_empty_adapters_returns_empty_names() {
    let orchestrator = agent_external_lint_orchestrator::ExternalLintOrchestrator::new(
        agent_external_lint_orchestrator::ExternalLintDeps {
            adapters: HashMap::new(),
        },
    );
    let names = orchestrator.adapter_names();
    assert!(names.is_empty());
}

#[tokio::test]
async fn orchestrator_with_no_adapters_returns_empty_results() {
    let orchestrator = agent_external_lint_orchestrator::ExternalLintOrchestrator::new(
        agent_external_lint_orchestrator::ExternalLintDeps {
            adapters: HashMap::new(),
        },
    );
    let path = shared::common::taxonomy_path_vo::FilePath::new("/tmp".to_string()).unwrap();
    let results = orchestrator.scan_all(&path).await;
    assert!(results.is_empty());
}

// ─── Aggregate Trait Object Safety ────────────────────────

#[test]
fn aggregate_is_object_safe() {
    let container = ExternalLintContainer::new();
    let aggregate: Arc<dyn IExternalLintAggregate> = container.aggregate();
    // Verify we can call methods through the trait object
    let _ = aggregate.adapter_names();
}

// ─── Multiple Container Instances ─────────────────────────

#[test]
fn multiple_containers_are_independent() {
    let c1 = ExternalLintContainer::new();
    let c2 = ExternalLintContainer::new();
    let a1 = c1.aggregate();
    let a2 = c2.aggregate();
    // Different Arc pointers
    assert!(!Arc::ptr_eq(&a1, &a2));
}
