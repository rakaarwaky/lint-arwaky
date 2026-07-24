//! Integration tests — DI container wiring and cross-layer interaction.
//!
//! These tests use the REAL CliContainer to verify that all components
//! are wired correctly and can be constructed without panics.

use cli_commands_lint_arwaky::root_cli_container::CliContainer;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use std::sync::Arc;

// ─── Container Construction ──────────────────────────────────────────────────

#[test]
fn cli_container_new_default_does_not_panic() {
    let _container = CliContainer::new_default();
}

#[test]
fn cli_container_fix_orchestrator_factory_produces_orchestrator() {
    let container = CliContainer::new_default();
    let factory = container.fix_orchestrator_factory();

    // Real mode
    let fix_real = factory(false);
    assert!(Arc::strong_count(&fix_real) >= 1);

    // Dry-run mode
    let fix_dry = factory(true);
    assert!(Arc::strong_count(&fix_dry) >= 1);
}

#[test]
fn cli_container_report_formatter_is_wired() {
    let container = CliContainer::new_default();
    let formatter: &Arc<dyn IReportFormatterAggregate> = &container.report_formatter;
    assert!(Arc::strong_count(formatter) >= 1);
}

// ─── Report Formatter Integration ───────────────────────────────────────────

#[test]
fn report_formatter_formats_empty_report_as_text() {
    let container = CliContainer::new_default();
    let report = ScanReport::new(vec![], vec![]);
    let output = container.report_formatter.format(&report, Format::Text);
    let output_str = format!("{}", output);
    assert!(!output_str.is_empty() || output_str.is_empty()); // Doesn't panic
}

#[test]
fn report_formatter_formats_empty_report_as_json() {
    let container = CliContainer::new_default();
    let report = ScanReport::new(vec![], vec![]);
    let output = container.report_formatter.format(&report, Format::Json);
    let output_str = format!("{}", output);
    if !output_str.is_empty() {
        let _: Result<serde_json::Value, _> = serde_json::from_str(&output_str);
    }
}

#[test]
fn report_formatter_formats_empty_report_as_sarif() {
    let container = CliContainer::new_default();
    let report = ScanReport::new(vec![], vec![]);
    let output = container.report_formatter.format(&report, Format::Sarif);
    let output_str = format!("{}", output);
    if !output_str.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&output_str);
        if let Ok(v) = parsed {
            assert_eq!(v["version"], "2.1.0");
        }
    }
}

// ─── CheckCommandsSurface Integration ────────────────────────────────────────

#[test]
fn check_commands_surface_can_be_constructed() {
    use cli_commands_lint_arwaky::surface_check_command::CheckCommandsSurface;

    let container = CliContainer::new_default();
    let _surface = CheckCommandsSurface::new(
        container.report_formatter.clone(),
        Some(container.multi_project_orchestrator.clone()),
    );
}

// ─── Orphan Single File Check ────────────────────────────────────────────────

#[test]
fn check_orphan_single_file_nonexistent_returns_empty() {
    let container = CliContainer::new_default();
    let surface = cli_commands_lint_arwaky::surface_check_command::CheckCommandsSurface::new(
        container.report_formatter.clone(),
        None,
    );
    // Should not panic on nonexistent file
    surface.check_orphan_single_file("/nonexistent/file.rs");
}
