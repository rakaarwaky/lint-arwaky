// PURPOSE: File layer → skill routing, one rule for every violation code.
// Pure data: the layer→skill table. No file I/O, no layer detection — callers
// resolve the layer from the file path first (utility layer) and pass it here.

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
            (_, Some(command)) => format!("[run cli \"{command}\"]"),
            (Some(skill), None) => format!("[run cli \"lint-arwaky-cli skill read {skill}\"]"),
            (None, None) => "[run cli \"lint-arwaky-cli skill read aes-lint-arwaky\"]".to_string(),
        }
    }
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
            skill: layer.and_then(|l| {
                LAYER_SKILLS
                    .iter()
                    .find(|(name, _)| *name == l)
                    .map(|(_, skill)| *skill)
            }),
            fix_command: None,
        },
    }
}

/// Filename stem prefix → AES layer name. Mirrors the AES102 prefix convention;
/// `surfaces` is the layer name the taxonomy routing table uses.
///
/// Lives here, beside the routing table, so every utility resolves the layer
/// from the shared data instead of importing another utility module (AES201).
pub fn layer_from_path(file_path: &str) -> Option<&'static str> {
    const PREFIXES: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("utility_", "utility"),
        ("agent_", "agent"),
        ("surface_", "surfaces"),
        ("root_", "root"),
    ];
    let filename = file_path.rsplit(['/', '\\']).next().unwrap_or(file_path);
    let stem = filename.split('.').next().unwrap_or(filename);
    PREFIXES
        .iter()
        .find(|(prefix, _)| stem.starts_with(prefix))
        .map(|(_, layer)| *layer)
}

/// Resolve the remediation for `code` in `file_path`, detecting the layer.
///
/// The convenience form of [`resolve_skill_hint`] for callers that hold a path
/// rather than a pre-detected layer.
pub fn resolve_skill_hint_for_path(code: &str, file_path: &str) -> SkillHint {
    resolve_skill_hint(code, layer_from_path(file_path))
}
