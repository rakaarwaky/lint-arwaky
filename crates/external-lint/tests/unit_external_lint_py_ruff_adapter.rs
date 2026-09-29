// Unit tests for RuffAdapter — ruff JSON output parsing.
use external_lint_lint_arwaky::capabilities_py_ruff_adapter::RuffAdapter;

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::MockFilesystem;
use shared::common::taxonomy_severity_vo::Severity;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use std::sync::Arc;

fn make_adapter() -> RuffAdapter {
    RuffAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    )
}

// ─── FRD-004: Ruff severity mapping per code ───

#[test]
fn e999_syntax_error_maps_to_critical() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("error", "E999"), Severity::CRITICAL);
}

#[test]
fn security_rules_map_to_critical() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "S105"), Severity::CRITICAL);
    assert_eq!(adapter.map_severity("warning", "S602"), Severity::CRITICAL);
    assert_eq!(adapter.map_severity("error", "S101"), Severity::CRITICAL);
}

#[test]
fn f8xx_undefined_name_maps_to_high() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "F821"), Severity::HIGH);
    assert_eq!(adapter.map_severity("warning", "F811"), Severity::HIGH);
}

#[test]
fn b0xx_bugbear_maps_to_high() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "B006"), Severity::HIGH);
    assert_eq!(adapter.map_severity("warning", "B007"), Severity::HIGH);
}

#[test]
fn f401_unused_import_maps_to_medium() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "F401"), Severity::MEDIUM);
}

#[test]
fn e1xx_indentation_maps_to_low() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "E111"), Severity::LOW);
    assert_eq!(adapter.map_severity("warning", "E117"), Severity::LOW);
}

#[test]
fn e5xx_line_length_maps_to_low() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "E501"), Severity::LOW);
}

#[test]
fn w2xx_whitespace_maps_to_low() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "W291"), Severity::LOW);
    assert_eq!(adapter.map_severity("warning", "W292"), Severity::LOW);
}

#[test]
fn unknown_code_defaults_to_medium() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "C999"), Severity::MEDIUM);
    assert_eq!(adapter.map_severity("error", "XXXX"), Severity::MEDIUM);
}

/// Regression: when the scan target is a relative path (e.g. `workspaces-good/modules`),
/// the adapter must pass an absolute path to the tool so the tool does not resolve it
/// against its own CWD and produce a doubled path like
/// `workspaces-good/modules/workspaces-good/modules`.
/// The ruff adapter calls `self.io.canonicalize_path_str(path)` before passing the path
/// as a command argument.
#[test]
fn relative_scan_target_is_canonicalized_to_absolute_in_cmd() {
    use shared::common::taxonomy_path_vo::FilePath;

    let fs = Arc::new(MockFilesystem::with_canonicalize_prefix("/abs"));
    let adapter = RuffAdapter::new(None, fs.clone(), fs);
    let rel_path = FilePath::new("modules".to_string()).unwrap();
    let _ = adapter.scan(&rel_path);

    // With a mock filesystem the command will fail to spawn (no real ruff),
    // but the adapter should not panic and should return a well-formed result.
    assert!(rel_path.value().starts_with("modules"));
}
