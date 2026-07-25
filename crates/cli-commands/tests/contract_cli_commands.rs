//! Contract tests — verify trait implementations exist and are wired correctly.
//!
//! These tests compile-time assert that concrete types implement their
//! declared contract traits. If a trait impl is removed, these fail at compile.

use std::sync::Arc;

use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::code_analysis::ICodeAnalysisAggregate;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::external_lint::IExternalLintAggregate;
use shared::file_watch::IWatchAggregate;
use shared::git_hooks::GitHooksAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::maintenance::MaintenanceCommandsAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_detector::IOrphanAggregate;
use shared::project_setup::SetupManagementAggregate;
use shared::report_formatter::{
    IReportFormatterAggregate,
    IReportFormatterProtocol,
};

use shared::role_rules::IRoleRunnerAggregate;

use cli_commands_lint_arwaky::root_cli_container::CliContainer;

// ─── Root Layer Contracts ────────────────────────────────────────────────────

#[test]
#[allow(clippy::type_complexity)]
fn cli_container_exposes_fix_orchestrator_factory() {
    // Compile-time: fix_orchestrator_factory returns a closure producing Arc<dyn LintFixOrchestratorAggregate>
    fn assert_factory(
        _f: fn(
            &CliContainer,
        ) -> Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
    ) {
    }
    assert_factory(CliContainer::fix_orchestrator_factory);
}

// ─── Surface Layer Struct Existence ─────────────────────────────────────────

#[test]
#[allow(clippy::type_complexity)]
fn check_commands_surface_is_constructible() {
    use cli_commands_lint_arwaky::surface_check_command::CheckCommandsSurface;
    // Compile-time: CheckCommandsSurface::new exists with correct signature
    fn assert_new(
        _f: fn(
            Arc<dyn IReportFormatterAggregate>,
            Option<Arc<dyn IConfigOrchestratorAggregate>>,
        ) -> CheckCommandsSurface,
    ) {
    }
    assert_new(CheckCommandsSurface::new);
}

#[test]
#[allow(clippy::type_complexity)]
fn fix_commands_surface_is_constructible() {
    use cli_commands_lint_arwaky::surface_fix_action::FixCommandsSurface;
    fn assert_new(
        _f: fn(
            Arc<dyn ICodeAnalysisAggregate>,
            Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
        ) -> FixCommandsSurface,
    ) {
    }
    assert_new(FixCommandsSurface::new);
}

#[test]
fn watch_commands_surface_is_constructible() {
    use cli_commands_lint_arwaky::surface_watch_command::WatchCommandsSurface;
    let _surface = WatchCommandsSurface::new();
}

#[test]
fn watch_commands_surface_implements_default() {
    use cli_commands_lint_arwaky::surface_watch_command::WatchCommandsSurface;
    fn assert_default<T: Default>() {}
    assert_default::<WatchCommandsSurface>();
}

// ─── Report Formatter Protocol Contracts ─────────────────────────────────────

#[test]
fn report_formatter_orchestrator_implements_i_report_formatter_aggregate() {
    use report_formatter::ReportFormatterOrchestrator;
    fn assert_trait<T: IReportFormatterAggregate>() {}
    assert_trait::<ReportFormatterOrchestrator>();
}

// ─── Trait Object Safety ─────────────────────────────────────────────────────

#[test]
fn all_aggregate_traits_are_object_safe() {
    let _: Option<Arc<dyn IReportFormatterAggregate>> = None;
    let _: Option<Arc<dyn IReportFormatterProtocol>> = None;
    let _: Option<Arc<dyn ICodeAnalysisAggregate>> = None;
    let _: Option<Arc<dyn IConfigOrchestratorAggregate>> = None;
    let _: Option<Arc<dyn IExternalLintAggregate>> = None;
    let _: Option<Arc<dyn IImportRunnerAggregate>> = None;
    let _: Option<Arc<dyn INamingRunnerAggregate>> = None;
    let _: Option<Arc<dyn IRoleRunnerAggregate>> = None;
    let _: Option<Arc<dyn IOrphanAggregate>> = None;
    let _: Option<Arc<dyn GitHooksAggregate>> = None;
    let _: Option<Arc<dyn LintFixOrchestratorAggregate>> = None;
    let _: Option<Arc<dyn MaintenanceCommandsAggregate>> = None;
    let _: Option<Arc<dyn SetupManagementAggregate>> = None;
    let _: Option<Arc<dyn IWatchAggregate>> = None;
}
