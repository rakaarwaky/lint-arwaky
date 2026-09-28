// PURPOSE: AES-code → skill routing table, mirroring HOW-TO-USE-LINT-ROUTING.md.
// Pure data plus resolution over (code, layer) — no file I/O, no layer detection.
// Callers that have a file path resolve the layer first via the utility layer
// (`resolve_skill_hint_for_file`) and pass the layer name here.
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::common::taxonomy_error_vo::ErrorCode;

/// Layer name (as the layer detector reports it) → skill name.
/// The detector returns `surfaces` for surface files, matching the AES spec.
const LAYER_SKILLS: &[(&str, &str)] = &[
    ("taxonomy", "aes-taxonomy"),
    ("contract", "aes-contract"),
    ("capabilities", "aes-capabilities"),
    ("utility", "aes-utility"),
    ("agent", "aes-agent"),
    ("surfaces", "aes-surface"),
    ("root", "aes-root"),
];

/// Codes whose fix always belongs to the same skill.
const FIXED_SKILLS: &[(&str, &str)] = &[
    ("AES201", "aes-contract"),
    ("AES205", "aes-contract"),
    ("AES305", "aes-utility"),
    ("AES401", "aes-taxonomy"),
    ("AES402", "aes-contract"),
    ("AES403", "aes-capabilities"),
    ("AES404", "aes-utility"),
    ("AES405", "aes-agent"),
    ("AES406", "aes-surface"),
    ("AES601", "aes-docs"),
    ("AES602", "aes-docs"),
    ("AES603", "aes-docs"),
    ("AES604", "aes-docs"),
    ("AES605", "aes-docs"),
];

/// Codes whose fix is a single `fix` invocation rather than a skill read.
const FIX_COMMANDS: &[(&str, &str)] = &[
    ("AES203", "lint-arwaky-cli fix <path> --filter AES203"),
    ("AES304", "lint-arwaky-cli fix <path> --filter AES304"),
];

/// Orphan codes route to the skill owning the orphaned file's own layer.
const ORPHAN_CODES: &[&str] = &["AES501", "AES502", "AES503", "AES504", "AES505", "AES506"];

/// Codes routed to the skill owning the file's own layer.
const LAYER_SCOPED_CODES: &[&str] = &[
    "AES101", "AES102", "AES202", "AES204", "AES301", "AES302", "AES303",
];

fn fixed_map() -> &'static HashMap<&'static str, &'static str> {
    static MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| FIXED_SKILLS.iter().copied().collect())
}

fn fix_command_map() -> &'static HashMap<&'static str, &'static str> {
    static MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| FIX_COMMANDS.iter().copied().collect())
}

/// Skill owning a layer, defaulting to `aes-lint-arwaky` for unknown layers.
pub fn skill_of_layer(layer: &str) -> &'static str {
    LAYER_SKILLS
        .iter()
        .find(|(name, _)| *name == layer)
        .map(|(_, skill)| *skill)
        .unwrap_or("aes-lint-arwaky")
}

/// The remediation for one finding: a skill to read, or a fix command to run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillHint {
    /// Skill name (`aes-contract`), or `None` when the hint is a fix command or
    /// the file sits outside the AES naming convention.
    pub skill: Option<&'static str>,
    /// Exact command to run, when the fix is automated.
    pub fix_command: Option<&'static str>,
}

impl SkillHint {
    /// One-line guidance for text output. Never empty.
    pub fn guidance(&self) -> String {
        match (self.skill, self.fix_command) {
            (_, Some(command)) => (*command).to_string(),
            (Some(skill), None) => format!("lint-arwaky-cli skill read {skill}"),
            (None, None) => "lint-arwaky-cli skill read aes-lint-arwaky".to_string(),
        }
    }
}

/// Resolve the remediation for `code` in a file belonging to `layer`.
///
/// `layer` is the name reported by the layer detector (`surfaces` for surface
/// files) or `None` when the file is outside the AES naming convention.
/// Codes with no mapping (external tool codes) fall back to `aes-lint-arwaky`.
pub fn resolve_skill_hint(code: &str, layer: Option<&str>) -> SkillHint {
    if let Some(command) = fix_command_map().get(code) {
        return SkillHint {
            skill: None,
            fix_command: Some(command),
        };
    }
    if let Some(skill) = fixed_map().get(code) {
        return SkillHint {
            skill: Some(skill),
            fix_command: None,
        };
    }
    if ORPHAN_CODES.contains(&code) || LAYER_SCOPED_CODES.contains(&code) {
        return SkillHint {
            skill: layer.map(skill_of_layer),
            fix_command: None,
        };
    }
    SkillHint {
        skill: Some("aes-lint-arwaky"),
        fix_command: None,
    }
}

/// Resolve a hint from an `ErrorCode` plus layer name.
pub fn resolve_skill_hint_typed(code: &ErrorCode, layer: Option<&str>) -> SkillHint {
    resolve_skill_hint(code.code(), layer)
}
