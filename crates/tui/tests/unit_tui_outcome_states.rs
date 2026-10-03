// Unit tests — LintExecutionResult three-state outcome and TUI fallback status.
// Issue #566: capability-absent fallbacks must render "Unavailable in TUI — see CLI"
// and failure paths must render a structured message, never the bare word "Error".
use dispatcher::surface_orphan_action::OrphanFactory;
use filesystem::root_filesystem_container::FilesystemContainer;
use quality_rules::root_quality_rules_container::CodeAnalysisContainer;
use shared_tui::{LintExecutionResult, LintOutcome, outcome_label};
use std::sync::Arc;
use tui_lint_arwaky::surface_lint_action::SurfaceLintExecutor;

fn build_executor() -> Arc<SurfaceLintExecutor> {
    let code_container = CodeAnalysisContainer::new();
    let fs = FilesystemContainer::new();
    let fs_seam = Arc::new(dispatcher::surface_check_action::FilesystemSeam {
        workspace: fs.workspace(),
        parser: fs.parser(),
        aggregate: fs.orchestrator(),
    });
    let fs_factory: Arc<
        dyn Fn() -> dispatcher::surface_check_action::FilesystemSeam + Send + Sync,
    > = Arc::new(|| {
        let c = FilesystemContainer::new();
        dispatcher::surface_check_action::FilesystemSeam {
            workspace: c.workspace(),
            parser: c.parser(),
            aggregate: c.orchestrator(),
        }
    });
    // The executor under test always has orphan_aggregate unset, so the factory
    // is never invoked by the fallback paths tested here.
    let orphan_factory: Arc<OrphanFactory> = Arc::new(|config, fs, ws| {
        orphan_rules::root_orphan_detector_container::OrphanContainer::new_with_config(
            config, fs, ws,
        )
        .analyzer()
    });

    Arc::new(SurfaceLintExecutor::new(
        code_container.code_analysis_linter(),
        fs.orchestrator(),
        fs.workspace(),
        fs.tool_resolution(),
        fs_seam,
        fs_factory,
        orphan_factory,
    ))
}

// ─── LintExecutionResult constructor semantics ─────────────

#[test]
fn success_constructor_sets_outcome() {
    let r = LintExecutionResult::success("ok".to_string(), 0);
    assert_eq!(r.outcome, LintOutcome::Success);
    assert!(r.success);
}

#[test]
fn unavailable_constructor_is_distinct_from_success() {
    let r = LintExecutionResult::unavailable("use CLI".to_string());
    assert_eq!(r.outcome, LintOutcome::Unavailable);
    assert!(!r.success);
    assert_eq!(r.violation_count, 0);
}

#[test]
fn failure_constructor_sets_outcome() {
    let r = LintExecutionResult::failure("boom".to_string());
    assert_eq!(r.outcome, LintOutcome::Failure);
    assert!(!r.success);
}

// ─── outcome_label status-bar rendering ────────────────────

#[test]
fn outcome_label_success_is_done() {
    let label = outcome_label(LintOutcome::Success, "crates/foo");
    assert_eq!(label, "Done");
}

#[test]
fn outcome_label_unavailable_mentions_cli() {
    let label = outcome_label(LintOutcome::Unavailable, "crates/foo");
    assert!(label.contains("Unavailable in TUI"));
    assert!(label.contains("crates/foo"));
}

#[test]
fn outcome_label_failure_names_path_and_retry_hint() {
    let label = outcome_label(LintOutcome::Failure, "crates/foo");
    assert!(label.contains("Error on crates/foo"));
    assert!(label.contains("retry"));
    assert_ne!(label, "Error");
}

// ─── Capability-absent fallbacks return Unavailable ───────

#[test]
fn fix_without_orchestrator_is_unavailable() {
    let executor = build_executor();
    let flags = tui_lint_arwaky::ActionFlags::default();
    let result = executor.fix("./src", &flags);
    assert_eq!(result.outcome, LintOutcome::Unavailable);
    assert!(!result.success);
    assert!(result.output.contains("FixOrchestrator aggregate"));
}

#[test]
fn doctor_without_maintenance_is_unavailable() {
    let executor = build_executor();
    let result = executor.doctor();
    assert_eq!(result.outcome, LintOutcome::Unavailable);
    assert!(!result.success);
    assert!(result.output.contains("maintenance doctor"));
}

#[test]
fn init_without_setup_is_unavailable() {
    let executor = build_executor();
    let flags = tui_lint_arwaky::ActionFlags::default();
    let result = executor.init(&flags);
    assert_eq!(result.outcome, LintOutcome::Unavailable);
    assert!(!result.success);
    assert!(result.output.contains("lint-arwaky-cli init"));
}

#[test]
fn install_without_setup_is_unavailable() {
    let executor = build_executor();
    let flags = tui_lint_arwaky::ActionFlags::default();
    let result = executor.install(&flags);
    assert_eq!(result.outcome, LintOutcome::Unavailable);
    assert!(!result.success);
    assert!(result.output.contains("setup install"));
}

#[test]
fn security_without_maintenance_is_unavailable() {
    let executor = build_executor();
    let result = executor.security("./src");
    assert_eq!(result.outcome, LintOutcome::Unavailable);
    assert!(!result.success);
    assert!(result.output.contains("lint-arwaky-cli security"));
}

#[test]
fn dependencies_without_maintenance_is_unavailable() {
    let executor = build_executor();
    let result = executor.dependencies("./src");
    assert_eq!(result.outcome, LintOutcome::Unavailable);
    assert!(!result.success);
    assert!(result.output.contains("lint-arwaky-cli dependencies"));
}

#[test]
fn orphan_without_orphan_aggregate_is_unavailable() {
    let executor = build_executor();
    let result = executor.orphan("./src");
    assert_eq!(result.outcome, LintOutcome::Unavailable);
    assert!(!result.success);
    assert!(result.output.contains("lint-arwaky-cli orphan"));
}
