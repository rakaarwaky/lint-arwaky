// Smoke tests — module imports work, key types accessible within 5s.
use cli_commands::utility_output_text_formatter;

#[test]
fn smoke_utility_output_text_formatter_module_imports() {
    let start = std::time::Instant::now();
    // Verify the formatting module's public functions are accessible
    let _ = utility_output_text_formatter::status_icon as fn(bool) -> &'static str;
    let _ = utility_output_text_formatter::group_by_member;
    let _ = utility_output_text_formatter::output_violations;
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
    let _ = cli_commands::utility_output_text_formatter::group_by_member;
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
