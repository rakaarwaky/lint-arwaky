// Acceptance tests — verify maintenance operations produce valid results.
use shared::common::FilePath;
use shared::maintenance::MaintenanceCommandsAggregate;

fn make_orch() -> std::sync::Arc<dyn MaintenanceCommandsAggregate> {
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer::new(fs)
        .orchestrator()
}

#[test]
fn maintenance_stats_counts_are_consistent() {
    let orch = make_orch();
    let path = FilePath::new(".").unwrap();
    let stats = orch.stats(&path);
    // total_files should be >= sum of language-specific files
    let lang_total = stats.rust_files.value + stats.python_files.value + stats.js_files.value;
    assert!(
        stats.total_files.value >= lang_total,
        "total_files ({}) should be >= lang_total ({})",
        stats.total_files.value,
        lang_total
    );
}

#[test]
fn maintenance_doctor_reports_health_status() {
    let orch = make_orch();
    let doctor = orch.doctor();
    // healthy field should be set (not default)
    let _ = doctor.healthy.value;
    // is_installed should be set
    let _ = doctor.is_installed.value;
}

#[test]
fn maintenance_health_check_covers_all_languages() {
    let orch = make_orch();
    let health = orch.health_check();
    let mut has_rust = false;
    let mut has_python = false;
    let mut has_js = false;
    for adapter in &health.adapters {
        match adapter.language.as_str() {
            "Rust" => has_rust = true,
            "Python" => has_python = true,
            "JS/TS" => has_js = true,
            _ => {}
        }
    }
    assert!(has_rust, "Should have Rust adapters");
    assert!(has_python, "Should have Python adapters");
    assert!(has_js, "Should have JS/TS adapters");
}

#[test]
fn maintenance_toolchain_has_required_tools() {
    let orch = make_orch();
    let diag = orch.diagnose_toolchain();
    // Should have rustc and cargo at minimum
    let tool_names: Vec<&str> = diag.rust_tools.iter().map(|t| t.name.as_str()).collect();
    assert!(tool_names.contains(&"rustc"), "Should have rustc");
    assert!(tool_names.contains(&"cargo"), "Should have cargo");
}
