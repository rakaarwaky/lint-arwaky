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

// Rule codes are derived from the source of truth — the `| AESxxx |` rows of
// the rules tables in RULES_AES.md — so a rule added there is covered here
// automatically instead of silently falling behind a hard-coded array.
fn aes_codes_from_rules_doc() -> Vec<String> {
    include_str!("../../../RULES_AES.md")
        .lines()
        .filter_map(|line| {
            let rest = line.trim_start().strip_prefix("| AES")?;
            Some(format!("AES{}", rest.get(0..3)?))
        })
        .collect()
}

#[test]
fn every_aes_code_resolves_to_non_empty_guidance() {
    let codes = aes_codes_from_rules_doc();
    assert!(
        codes.len() >= 32,
        "parsed only {} rule codes from RULES_AES.md — table format changed?",
        codes.len()
    );
    for code in codes {
        let hint = resolve_skill_hint_for_file(&code, "crates/foo/contract_scan_protocol.rs");
        assert!(!hint.guidance().is_empty(), "code {code} gave empty hint");
    }
}
