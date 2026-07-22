// PURPOSE: Utility functions for scope-based file matching in import rules
// Extracted from forbidden/mandatory checkers to eliminate duplicated
// stem/suffix extraction and scope-membership logic.

use super::taxonomy_layer_vo::Identity;
use crate::import_rules::utility_import_resolver;

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
/// use crate::common::utility_scope_matcher::file_belongs_to_scope;
/// use crate::taxonomy_layer_vo::Identity;
///
/// // "surfaces_auth.rs" belongs to layer "surfaces" (no suffix constraint)
/// let result = file_belongs_to_scope("surfaces_auth.rs", &Identity::new("surfaces"));
/// assert!(result.is_some());
/// ```
pub fn file_belongs_to_scope(
    basename: &str,
    scope_identity: &Identity,
) -> Option<(String, Vec<Identity>)> {
    let stem = extract_file_stem(basename);

    // Resolve scope to get expected layer and suffixes
    let (expected_layer, suffixes) = utility_import_resolver::resolve_scope(scope_identity);
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
/// use crate::common::utility_scope_matcher::extract_file_stem;
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
/// use crate::common::utility_scope_matcher::extract_layer_prefix;
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
/// use crate::common::utility_scope_matcher::extract_suffix;
///
/// assert_eq!(extract_suffix("surfaces_auth"), "auth");
/// assert_eq!(extract_suffix("utility_parser"), "parser");
/// assert_eq!(extract_suffix("no_suffix"), "");
/// ```
pub fn extract_suffix(stem: &str) -> &str {
    stem.rsplit('_').next().map_or("", |s| s)
}

#[cfg(test)]
mod tests {
    use super::{
        extract_file_stem, extract_layer_prefix, extract_suffix, file_belongs_to_scope, Identity,
    };

    #[test]
    fn test_file_belongs_to_scope_matches() {
        let scope = Identity::new("surfaces");
        let result = file_belongs_to_scope("surfaces_auth.rs", &scope);
        assert!(result.is_some());
        let (layer, suffixes) = result.unwrap();
        assert_eq!(layer, "surfaces");
        assert!(suffixes.is_empty());
    }

    #[test]
    fn test_file_belongs_to_scope_no_match() {
        let scope = Identity::new("surfaces");
        let result = file_belongs_to_scope("utility_auth.rs", &scope);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_file_stem() {
        assert_eq!(extract_file_stem("surfaces_auth.rs"), "surfaces_auth");
        assert_eq!(extract_file_stem("mod.rs"), "mod");
        assert_eq!(extract_file_stem("lib.rs"), "lib");
        assert_eq!(extract_file_stem("no_extension"), "no_extension");
    }

    #[test]
    fn test_extract_layer_prefix() {
        assert_eq!(extract_layer_prefix("surfaces_auth"), "surfaces");
        assert_eq!(extract_layer_prefix("utility_parser"), "utility");
        assert_eq!(extract_layer_prefix("unknown_file"), "unknown");
        assert_eq!(extract_layer_prefix("single"), "single");
    }

    #[test]
    fn test_extract_suffix() {
        assert_eq!(extract_suffix("surfaces_auth"), "auth");
        assert_eq!(extract_suffix("utility_parser"), "parser");
        assert_eq!(extract_suffix("no_suffix"), "suffix");
        assert_eq!(extract_suffix("single"), "single");
    }
}
