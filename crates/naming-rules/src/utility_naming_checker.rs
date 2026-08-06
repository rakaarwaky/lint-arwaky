// PURPOSE: Shared helpers for naming checkers — stem/suffix extraction, result construction.
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::ColumnNumber;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_lint_vo::LocationList;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::taxonomy_naming_constant::ADAPTER_NAME;

/// Extract the file stem using the last dot (rfind), consistent across all checkers.
///
/// For multi-dot filenames like `foo.spec.rs`, this returns `foo.spec`.
/// For single-dot files like `checker.rs`, this returns `checker`.
/// For dotfiles like `.gitignore`, the entire filename is returned.
/// If there is no dot, the entire filename is returned.
pub fn get_stem(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .file_stem()
        .and_then(|n| n.to_str())
}

/// Extract the suffix (word after the last underscore) from a stem.
pub fn get_suffix(stem: &str) -> Option<&str> {
    stem.rfind('_').map(|pos| &stem[pos + 1..])
}

/// Basename of a path — always yields at least one element.
pub fn basename_of(path: &str) -> &str {
    match path.rsplit('/').next() {
        Some(name) => name,
        None => path,
    }
}

/// Parse a FilePath; skip files that fail validation (empty path).
pub fn parse_path(filename: &str) -> Option<FilePath> {
    FilePath::new(filename.to_string()).ok()
}

/// Construct a file-level LintResult from a string filename.
pub fn string_filename_result(
    file: &str,
    code: &str,
    message: impl Into<String>,
    severity: Severity,
) -> LintResult {
    let file_path = FilePath::new(file).unwrap_or_default();
    LintResult {
        file: file_path,
        line: LineNumber::new(1),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(message),
        source: Some(AdapterName::raw(ADAPTER_NAME)),
        severity,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}

/// Collect the rule-level exceptions list for a given rule code.
///
/// FRD FR-001/FR-002: "Files in the rule's exceptions list are skipped."
/// This lookup is evaluated before layer detection so unknown-prefix files
/// can also be excepted.
pub fn rule_exception_set(
    config: &ArchitectureConfig,
    rule_code: &str,
) -> std::collections::HashSet<String> {
    config
        .rules
        .iter()
        .find(|r| r.rule_type.code() == rule_code)
        .map(|r| r.exceptions.values.iter().cloned().collect())
        .unwrap_or_default()
}

/// Detect the architectural layer from a file path using the layer prefix.
///
/// Inlines `extract_filename`, `detect_layer_from_prefix`, and
/// `resolve_specialized_layer` to avoid utility→utility imports (AES201).
pub fn detect_layer(file: &str, layer_keys: &[String]) -> Option<String> {
    // extract_filename — get last path component
    let filename = std::path::Path::new(file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // detect_layer_from_prefix — match stem against known prefixes
    let stem = std::path::Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("utility_", "utility"),
        ("agent_", "agent"),
        ("surface_", "surfaces"),
        ("root_", "root"),
    ];

    let base_layer = PREFIX_MAP
        .iter()
        .find(|(prefix, _)| stem.starts_with(*prefix))
        .map(|(_, layer)| *layer)?;

    // resolve_specialized_layer — check for sub-layer like "capabilities(command)"
    let basename = std::path::Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if let Some(underscore_pos) = basename.rfind('_') {
        let suffix = &basename[underscore_pos + 1..];
        if !suffix.is_empty() {
            let specialized = format!("{}({})", base_layer, suffix);
            if layer_keys.contains(&specialized) {
                return Some(specialized);
            }
        }
    }

    Some(base_layer.to_string())
}
