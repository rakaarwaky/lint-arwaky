// Unit tests — skill hint resolution by file path (utility layer).
// The file’s layer determines the skill for every code.
use shared_cli_commands::resolve_skill_hint_for_file;

#[test]
fn file_paths_resolve_layer_correctly() {
    assert_eq!(
        resolve_skill_hint_for_file("AES101", "crates/foo/contract_scan_protocol.rs").skill,
        Some("aes-contract")
    );
    assert_eq!(
        resolve_skill_hint_for_file("AES101", "modules/taxonomy_project_setup_vo.py").skill,
        Some("aes-taxonomy")
    );
    assert_eq!(
        resolve_skill_hint_for_file("AES405", "packages/agent_scan_orchestrator.ts").skill,
        Some("aes-agent")
    );
    assert_eq!(
        resolve_skill_hint_for_file("AES101", "src/plain_file.rs").skill,
        None
    );
}

#[test]
fn aes201_follows_file_layer_not_fixed_contract() {
    assert_eq!(
        resolve_skill_hint_for_file("AES201", "crates/foo/agent_scan_orchestrator.rs").skill,
        Some("aes-agent")
    );
    assert_eq!(
        resolve_skill_hint_for_file("AES201", "crates/foo/taxonomy_project_setup_vo.rs").skill,
        Some("aes-taxonomy")
    );
}

#[test]
fn aes403_follows_file_layer_not_fixed_capabilities() {
    assert_eq!(
        resolve_skill_hint_for_file("AES403", "crates/foo/agent_scan_orchestrator.rs").skill,
        Some("aes-agent")
    );
    assert_eq!(
        resolve_skill_hint_for_file("AES403", "crates/foo/capabilities_scan_checker.rs").skill,
        Some("aes-capabilities")
    );
}

#[test]
fn aes505_orphan_in_agent_file_routes_to_agent_skill() {
    assert_eq!(
        resolve_skill_hint_for_file("AES505", "crates/foo/agent_scan_orchestrator.rs").skill,
        Some("aes-agent")
    );
}

#[test]
fn windows_style_paths_resolve_layer() {
    assert_eq!(
        resolve_skill_hint_for_file("AES406", "crates\\foo\\surface_scan_command.rs").skill,
        Some("aes-surface")
    );
}

#[test]
fn every_aes_code_resolves_to_non_empty_guidance() {
    let codes = [
        "AES101", "AES102", "AES201", "AES202", "AES203", "AES204", "AES205", "AES301", "AES302",
        "AES303", "AES304", "AES305", "AES401", "AES402", "AES403", "AES404", "AES405", "AES406",
        "AES501", "AES502", "AES503", "AES504", "AES505", "AES506", "AES601", "AES602", "AES603",
        "AES604", "AES701", "AES702", "AES703",
    ];
    for code in codes {
        let hint = resolve_skill_hint_for_file(code, "crates/foo/contract_scan_protocol.rs");
        assert!(!hint.guidance().is_empty(), "code {code} gave empty hint");
    }
}
