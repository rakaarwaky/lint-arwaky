// PURPOSE: Utility functions for scope-based file matching in import rules
// Extracted from forbidden/mandatory checkers to eliminate duplicated
// stem/suffix extraction and scope-membership logic.

use crate::common::taxonomy_layer_vo::{Identity, LayerNameVO};

/// Check if a file belongs to a given scope rule based on its filename.
///
/// Returns `Some((layer_prefix, suffixes))` when the file's stem matches
/// the scope's layer prefix and suffix constraints, or `None` otherwise.
///
/// This function handles:
/// - Extracting the stem (filename without extension)
/// - Resolving the scope to get expected layer and suffixes
/// - Checking if stem starts with `{layer}_` prefix
/// - Verifying suffix constraints if the scope has them
///
/// # Examples
/// ```rust
/// use shared_lint_arwaky::common::utility_scope_matcher::file_belongs_to_scope;
/// use shared_lint_arwaky::common::taxonomy_layer_vo::Identity;
///
/// // "surfaces_auth.rs" belongs to layer "surfaces" (no suffix constraint)
/// let result = file_belongs_to_scope("surfaces_auth.rs", &Identity::new("surfaces"));
/// assert!(result.is_some());
/// ```
fn resolve_scope(scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
    let scope_str = scope.value();
    if let Some(paren) = scope_str.find('(') {
        let layer = scope_str[..paren].trim();
        let inner = scope_str[paren + 1..].trim_end_matches(')').trim();
        let suffixes: Vec<Identity> = if inner.contains('|') {
            inner
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(Identity::new)
                .collect()
        } else {
            inner
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(Identity::new)
                .collect()
        };
        (LayerNameVO::new(layer), suffixes)
    } else {
        (LayerNameVO::new(scope_str.trim()), vec![])
    }
}

pub fn file_belongs_to_scope(
    basename: &str,
    scope_identity: &Identity,
) -> Option<(String, Vec<Identity>)> {
    let stem = extract_file_stem(basename);

    // Resolve scope to get expected layer and suffixes
    let (expected_layer, suffixes) = resolve_scope(scope_identity);
    let expected_prefix = expected_layer.value();

    // Check if stem starts with `{layer}_` prefix
    let expected_pattern = format!("{}_{}", expected_prefix, "");
    if !stem.starts_with(&expected_pattern) {
        return None;
    }

    // Check suffix constraint if any
    if !suffixes.is_empty() {
        let file_suffix = extract_suffix(stem);
        let suffix_match = suffixes.iter().any(|s| s.value() == file_suffix);
        if !suffix_match {
            return None;
        }
    }

    Some((expected_prefix.to_string(), suffixes))
}

/// Extract the file stem (without extension) from a basename.
///
/// # Examples
/// ```rust
/// use shared_lint_arwaky::common::utility_scope_matcher::extract_file_stem;
///
/// assert_eq!(extract_file_stem("surfaces_auth.rs"), "surfaces_auth");
/// assert_eq!(extract_file_stem("mod.rs"), "mod");
/// assert_eq!(extract_file_stem("lib.rs"), "lib");
/// ```
pub fn extract_file_stem(basename: &str) -> &str {
    basename.rsplit('.').next_back().map_or(basename, |s| s)
}

/// Extract the layer prefix from a file stem (first part before `_`).
///
/// # Examples
/// ```rust
/// use shared_lint_arwaky::common::utility_scope_matcher::extract_layer_prefix;
///
/// assert_eq!(extract_layer_prefix("surfaces_auth"), "surfaces");
/// assert_eq!(extract_layer_prefix("utility_parser"), "utility");
/// assert_eq!(extract_layer_prefix("unknown_file"), "unknown");
/// ```
pub fn extract_layer_prefix(stem: &str) -> &str {
    stem.split('_').next().map_or("unknown", |s| s)
}

/// Extract the suffix from a file stem (last part after `_`).
///
/// # Examples
/// ```rust
/// use shared_lint_arwaky::common::utility_scope_matcher::extract_suffix;
///
/// assert_eq!(extract_suffix("surfaces_auth"), "auth");
/// assert_eq!(extract_suffix("utility_parser"), "parser");
/// assert_eq!(extract_suffix("no_suffix"), "suffix");
/// ```
pub fn extract_suffix(stem: &str) -> &str {
    stem.rsplit('_').next().map_or("", |s| s)
}
