// PURPOSE: File layer → skill routing, one rule for every violation code.
// Pure data: the layer→skill table. No file I/O, no layer detection — callers
// resolve the layer from the file path first (utility layer) and pass it here.
use crate::common::taxonomy_error_vo::ErrorCode;

/// Layer name (as the layer detector reports it) → skill name.
const LAYER_SKILLS: &[(&str, &str)] = &[
    ("taxonomy", "aes-taxonomy"),
    ("contract", "aes-contract"),
    ("capabilities", "aes-capabilities"),
    ("utility", "aes-utility"),
    ("agent", "aes-agent"),
    ("surfaces", "aes-surface"),
    ("root", "aes-root"),
];

/// The remediation for one finding: a skill to read, or a fix command to run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillHint {
    /// Skill name, or `None` when the hint is a fix command.
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

/// Skill owning a layer, defaulting to `aes-lint-arwaky` for unknown layers.
pub fn skill_of_layer(layer: &str) -> &'static str {
    LAYER_SKILLS
        .iter()
        .find(|(name, _)| *name == layer)
        .map(|(_, skill)| *skill)
        .unwrap_or("aes-lint-arwaky")
}

/// Resolve the remediation for `code` in a file belonging to `layer`.
///
/// One rule: the file's layer owns the skill, for every code. The only codes
/// that bypass this are the two auto-fixable ones (AES203, AES304), whose fix
/// is a CLI command. Files outside the AES naming convention fall back to
/// `aes-lint-arwaky`.
pub fn resolve_skill_hint(code: &str, layer: Option<&str>) -> SkillHint {
    match code {
        "AES203" => SkillHint {
            skill: None,
            fix_command: Some("lint-arwaky-cli fix <path> --filter AES203"),
        },
        "AES304" => SkillHint {
            skill: None,
            fix_command: Some("lint-arwaky-cli fix <path> --filter AES304"),
        },
        _ => SkillHint {
            skill: layer.map(skill_of_layer),
            fix_command: None,
        },
    }
}

/// Resolve a hint from an `ErrorCode` plus layer name.
pub fn resolve_skill_hint_typed(code: &ErrorCode, layer: Option<&str>) -> SkillHint {
    resolve_skill_hint(code.code(), layer)
}
