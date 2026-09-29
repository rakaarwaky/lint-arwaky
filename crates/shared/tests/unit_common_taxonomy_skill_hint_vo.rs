// Unit tests — skill hint resolution: file layer → routing.
// One rule: the file's layer determines the skill for every code.
use shared_lint_arwaky::common::resolve_skill_hint_for_file;
use shared_lint_arwaky::common::taxonomy_skill_hint_vo::{
    SkillHint, resolve_skill_hint, skill_of_layer,
};

#[test]
fn skill_of_layer_maps_each_layer() {
    assert_eq!(skill_of_layer("taxonomy"), "aes-taxonomy");
    assert_eq!(skill_of_layer("contract"), "aes-contract");
    assert_eq!(skill_of_layer("capabilities"), "aes-capabilities");
    assert_eq!(skill_of_layer("utility"), "aes-utility");
    assert_eq!(skill_of_layer("agent"), "aes-agent");
    assert_eq!(skill_of_layer("surfaces"), "aes-surface");
    assert_eq!(skill_of_layer("root"), "aes-root");
}

#[test]
fn skill_of_layer_falls_back_for_unknown() {
    assert_eq!(skill_of_layer("nonexistent"), "aes-lint-arwaky");
}

// ─── resolve_skill_hint (code + layer) ───────────────────────────────────────

#[test]
fn aes101_routes_to_layer_skill() {
    let hint = resolve_skill_hint("AES101", Some("contract"));
    assert_eq!(hint.skill, Some("aes-contract"));
}

#[test]
fn aes201_in_agent_layer_routes_to_agent_skill() {
    // AES201 is a layer-scoped code: the agent file's own skill owns the fix.
    let hint = resolve_skill_hint("AES201", Some("agent"));
    assert_eq!(hint.skill, Some("aes-agent"));
}

#[test]
fn aes201_in_tax_layer_routes_to_tax_skill() {
    let hint = resolve_skill_hint("AES201", Some("taxonomy"));
    assert_eq!(hint.skill, Some("aes-taxonomy"));
}

#[test]
fn aes203_and_aes304_always_return_fix_command() {
    for (code, layer) in [
        ("AES203", Some("agent")),
        ("AES304", Some("contract")),
        ("AES203", None),
    ] {
        let hint = resolve_skill_hint(code, layer);
        assert_eq!(hint.skill, None, "code {code}");
        assert!(
            hint.fix_command
                .unwrap()
                .contains(&format!("--filter {code}")),
            "code {code}"
        );
    }
}

#[test]
fn aes403_in_capabilities_layer_routes_to_capabilities_skill() {
    let hint = resolve_skill_hint("AES403", Some("capabilities"));
    assert_eq!(hint.skill, Some("aes-capabilities"));
}

#[test]
fn aes403_in_agent_layer_routes_to_agent_skill() {
    // Even AES403 follows the file's layer, not a fixed mapping.
    let hint = resolve_skill_hint("AES403", Some("agent"));
    assert_eq!(hint.skill, Some("aes-agent"));
}

#[test]
fn unknown_layer_falls_back_to_lint_skill() {
    let hint = resolve_skill_hint("AES101", None);
    assert_eq!(hint.skill, None);
    assert_eq!(
        hint.guidance(),
        "[run cli \"lint-arwaky-cli skill read aes-lint-arwaky\"]"
    );
}

#[test]
fn external_tool_codes_use_layer_skill_when_layer_known() {
    let hint = resolve_skill_hint("clippy::needless_return", Some("contract"));
    assert_eq!(hint.skill, Some("aes-contract"));
}

#[test]
fn external_tool_codes_fall_back_when_layer_unknown() {
    let hint = resolve_skill_hint("clippy::needless_return", None);
    assert_eq!(hint.skill, None);
}

// ─── resolve_skill_hint_for_file (utility layer) ─────────────────────────────

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
fn guidance_string_for_skill() {
    let hint: SkillHint = resolve_skill_hint("AES403", Some("capabilities"));
    assert_eq!(
        hint.guidance(),
        "[run cli \"lint-arwaky-cli skill read aes-capabilities\"]"
    );
}

#[test]
fn guidance_string_for_fix_command() {
    let hint: SkillHint = resolve_skill_hint("AES203", Some("agent"));
    assert_eq!(
        hint.guidance(),
        "[run cli \"lint-arwaky-cli fix <path> --filter AES203\"]"
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
