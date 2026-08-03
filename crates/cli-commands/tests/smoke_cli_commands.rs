// Smoke tests — module imports work, key types accessible within 5s.
use cli_commands::surface_formatting;

#[test]
fn smoke_surface_formatting_module_imports() {
    let start = std::time::Instant::now();
    // Verify the formatting module's public functions are accessible
    let _ = surface_formatting::status_icon as fn(bool) -> &'static str;
    let _ = surface_formatting::group_by_member;
    let _ = surface_formatting::output_violations;
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn smoke_all_surface_modules_importable() {
    let start = std::time::Instant::now();
    let _ = cli_commands::surface_scan_command::handle_scan;
    let _ = cli_commands::surface_ci_command::handle_ci;
    let _ = cli_commands::surface_fix_command::handle_fix;
    let _ = cli_commands::surface_formatting::group_by_member;
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
