// Unit tests — skill hint resolution: code + layer → routing.
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

#[test]
fn aes101_routes_to_layer_skill() {
    let hint = resolve_skill_hint("AES101", Some("contract"));
    assert_eq!(hint.skill, Some("aes-contract"));
}

#[test]
fn aes102_routes_to_layer_skill() {
    let hint = resolve_skill_hint("AES102", Some("taxonomy"));
    assert_eq!(hint.skill, Some("aes-taxonomy"));
}

#[test]
fn aes101_falls_back_when_layer_none() {
    let hint = resolve_skill_hint("AES101", None);
    assert_eq!(hint.skill, None);
    assert_eq!(
        hint.guidance(),
        "lint-arwaky-cli skill read aes-lint-arwaky"
    );
}

#[test]
fn aes201_and_aes205_always_route_to_contract() {
    for code in ["AES201", "AES205"] {
        let hint = resolve_skill_hint(code, Some("utility"));
        assert_eq!(hint.skill, Some("aes-contract"), "code {code}");
    }
}

#[test]
fn aes203_and_aes304_route_to_fix_command() {
    for code in ["AES203", "AES304"] {
        let hint = resolve_skill_hint(code, Some("agent"));
        assert_eq!(hint.skill, None, "code {code}");
        assert!(
            hint.fix_command
                .unwrap()
                .contains(&format!("--filter {code}")),
            "code {code} filter mismatch"
        );
    }
}

#[test]
fn aes305_routes_to_utility_regardless_of_layer() {
    let hint = resolve_skill_hint("AES305", Some("agent"));
    assert_eq!(hint.skill, Some("aes-utility"));
}

#[test]
fn aes40x_each_maps_to_owning_skill() {
    let cases = [
        ("AES401", "aes-taxonomy"),
        ("AES402", "aes-contract"),
        ("AES403", "aes-capabilities"),
        ("AES404", "aes-utility"),
        ("AES405", "aes-agent"),
        ("AES406", "aes-surface"),
    ];
    for (code, skill) in cases {
        let hint = resolve_skill_hint(code, Some("agent"));
        assert_eq!(hint.skill, Some(skill), "code {code}");
    }
}

#[test]
fn aes50x_orphans_route_to_the_files_own_layer_skill() {
    let cases = [
        ("AES501", "taxonomy", "aes-taxonomy"),
        ("AES502", "contract", "aes-contract"),
        ("AES503", "capabilities", "aes-capabilities"),
        ("AES504", "utility", "aes-utility"),
        ("AES505", "agent", "aes-agent"),
        ("AES506", "surfaces", "aes-surface"),
    ];
    for (code, layer, skill) in cases {
        let hint = resolve_skill_hint(code, Some(layer));
        assert_eq!(hint.skill, Some(skill), "code {code} layer {layer}");
    }
}

#[test]
fn aes60x_docs_route_to_docs_skill() {
    for code in ["AES601", "AES602", "AES603", "AES604", "AES605"] {
        let hint = resolve_skill_hint(code, Some("root"));
        assert_eq!(hint.skill, Some("aes-docs"), "code {code}");
    }
}

#[test]
fn unknown_code_falls_back_to_lint_skill() {
    let hint = resolve_skill_hint("AES999", Some("agent"));
    assert_eq!(hint.skill, Some("aes-lint-arwaky"));
}

#[test]
fn external_tool_codes_do_not_panic() {
    for code in [
        "clippy::needless_return",
        "ruff.F401",
        "eslint.no-unused-vars",
    ] {
        let hint = resolve_skill_hint(code, Some("agent"));
        assert!(!hint.guidance().is_empty(), "code {code}");
    }
}

#[test]
fn guidance_string_for_skill() {
    let hint: SkillHint = resolve_skill_hint("AES403", Some("capabilities"));
    assert_eq!(
        hint.guidance(),
        "lint-arwaky-cli skill read aes-capabilities"
    );
}

#[test]
fn guidance_string_for_fix_command() {
    let hint: SkillHint = resolve_skill_hint("AES203", Some("agent"));
    assert_eq!(
        hint.guidance(),
        "lint-arwaky-cli fix <path> --filter AES203"
    );
}

// ─── resolve_skill_hint_for_file (utility layer) ─────────────────────────────

#[test]
fn file_paths_resolve_layer_correctly() {
    assert_eq!(
        resolve_skill_hint_for_file("AES101", "crates/foo/contract_scan_protocol.rs").skill,
        Some("aes-contract")
    );
    assert_eq!(
        resolve_skill_hint_for_file("AES102", "modules/taxonomy_setup_vo.py").skill,
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
        "AES604", "AES605",
    ];
    for code in codes {
        let hint = resolve_skill_hint_for_file(code, "crates/foo/contract_scan_protocol.rs");
        assert!(!hint.guidance().is_empty(), "code {code} gave empty hint");
    }
}
