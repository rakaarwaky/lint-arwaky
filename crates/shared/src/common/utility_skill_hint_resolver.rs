// PURPOSE: File-path → layer → skill hint, the resolution surface output uses.
// Utility layer: standalone functions only. The layer-prefix table is declared
// here rather than imported, because a utility file may not import another
// utility module (AES201 self-import).
use crate::taxonomy_error_vo::ErrorCode;
use crate::taxonomy_skill_hint_vo::SkillHint;
use crate::taxonomy_skill_hint_vo::resolve_skill_hint;

/// Filename stem prefix → layer name. Mirrors the AES102 prefix convention;
/// `surfaces` is the layer name the taxonomy routing table uses.
const LAYER_PREFIXES: &[(&str, &str)] = &[
    ("taxonomy_", "taxonomy"),
    ("contract_", "contract"),
    ("capabilities_", "capabilities"),
    ("utility_", "utility"),
    ("agent_", "agent"),
    ("surface_", "surfaces"),
    ("root_", "root"),
];

/// Detect the AES layer of a file path, tolerating both `/` and `\` separators.
/// Returns `None` for files outside the AES naming convention.
fn layer_from_path(file_path: &str) -> Option<&'static str> {
    let filename = file_path.rsplit(['/', '\\']).next().unwrap_or(file_path);
    let stem = filename.split('.').next().unwrap_or(filename);
    LAYER_PREFIXES
        .iter()
        .find(|(prefix, _)| stem.starts_with(prefix))
        .map(|(_, layer)| *layer)
}

/// Resolve the remediation for `code` in `file_path`.
///
/// The file's layer determines the skill — every code routes the same way.
/// Auto-fixable codes (AES203, AES304) return a fix command instead.
/// Files outside the AES naming convention fall back to `aes-lint-arwaky`.
pub fn resolve_skill_hint_for_file(code: &str, file_path: &str) -> SkillHint {
    resolve_skill_hint(code, layer_from_path(file_path))
}

/// Typed variant of `resolve_skill_hint_for_file`.
pub fn resolve_skill_hint_for_file_typed(code: &ErrorCode, file_path: &str) -> SkillHint {
    resolve_skill_hint_for_file(code.code(), file_path)
}
