// PURPOSE: UnusedImportRuleChecker — AES203: detect unused imports.
// V2: Cross-file trait usage analysis using implemented_traits_map.
// When a trait import appears unused, checks if any type used in the file
// implements that trait anywhere in the project (needed for method dispatch).

use crate::utility_import_resolver;
use crate::utility_import_symbol_extractor;
use shared::cli_commands::LintResult;
use shared::common::{FilePath, LintMessage, Severity};
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_import_error::ImportError;
use std::collections::HashMap;

pub struct UnusedImportRuleChecker;

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(
        &self,
        path: &FilePath,
        content: &str,
        import_entries: &[ImportEntry],
        used_identifiers: &[String],
    ) -> Result<Vec<LintMessage>, ImportError> {
        if utility_import_resolver::is_barrel_file(&path.basename()) {
            return Ok(Vec::new());
        }
        let imported_aliases =
            utility_import_symbol_extractor::extract_imported_aliases_from_entries(import_entries);
        let exported_symbols =
            utility_import_symbol_extractor::extract_exported_symbols_from_entries(import_entries);
        let used_symbols = utility_import_symbol_extractor::extract_used_symbols(
            path.value(),
            content,
            &imported_aliases,
            used_identifiers,
        );
        let mut unused: Vec<String> = Vec::new();
        for alias in imported_aliases.keys() {
            let alias_str = alias.value();
            if crate::utility_import_resolver::is_future_import(content, alias_str) {
                continue;
            }
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias_str.to_string());
            }
        }
        Ok(unused.into_iter().map(LintMessage::new).collect())
    }

    fn check_unused_imports(
        &self,
        file: &str,
        content: &str,
        import_entries: &[ImportEntry],
        used_identifiers: &[String],
        implemented_traits: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<LintResult>, ImportError> {
        let basename = std::path::Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if utility_import_resolver::is_barrel_file(basename) {
            return Ok(Vec::new());
        }
        let imported_aliases =
            utility_import_symbol_extractor::extract_imported_aliases_from_entries(import_entries);
        let exported_symbols =
            utility_import_symbol_extractor::extract_exported_symbols_from_entries(import_entries);
        let used_symbols = utility_import_symbol_extractor::extract_used_symbols(
            file,
            content,
            &imported_aliases,
            used_identifiers,
        );
        let mut violations = Vec::new();
        for alias in imported_aliases.keys() {
            let alias_str = alias.value();
            if crate::utility_import_resolver::is_future_import(content, alias_str) {
                continue;
            }
            if used_symbols.contains(alias) || exported_symbols.contains(alias) {
                continue;
            }
            // ─── Cross-file trait usage detection ───────────────
            // If the trait is implemented for any type used in this file,
            // the import is needed for method dispatch (not truly unused).
            if is_trait_used_for_method_dispatch(
                alias_str,
                implemented_traits,
                used_identifiers,
            ) {
                continue;
            }
            // ─── Fallback: well-known trait patterns ────────────
            if let Some(raw_path) = imported_aliases.get(alias) {
                let rp = raw_path.value();
                if is_known_trait_pattern(rp, alias_str) {
                    continue;
                }
            }
            let ast_line = utility_import_resolver::find_import_line_number(content, alias_str)
                .value() as usize;
            violations.push(LintResult::new_arch(
                file,
                ast_line,
                "AES203",
                Severity::MEDIUM,
                format!(
                    "AES203 UNUSED_IMPORT: Unused import '{alias_str}' detected.\n\
                        WHY? Unused imports clutter the codebase.\n\
                        FIX: Remove the unused import."
                ),
            ));
        }
        Ok(violations)
    }
}

impl Default for UnusedImportRuleChecker {
    fn default() -> Self {
        Self::new()
    }
}
impl UnusedImportRuleChecker {
    pub fn new() -> Self {
        Self
    }
}

/// Check if a trait import is used for method dispatch.
///
/// In Rust, importing a trait is required for calling its methods via
/// method dispatch (even without explicitly naming the trait). Tree-sitter
/// AST doesn't see the implicit trait usage at call sites, so these
/// imports appear "unused" but are semantically necessary.
///
/// This function uses the cross-file `implemented_traits` map to check:
/// 1. Is the trait name found in the project's impl blocks?
/// 2. Does any type that implements this trait appear in the file's used identifiers?
///
/// If both conditions are true, the import is needed for method dispatch.
fn is_trait_used_for_method_dispatch(
    trait_alias: &str,
    implemented_traits: &HashMap<String, Vec<String>>,
    used_identifiers: &[String],
) -> bool {
    // Find trait implementations — check both short name and full paths
    let implementing_types: Option<&Vec<String>> = implemented_traits.get(trait_alias).or_else(|| {
        // Try matching the last segment of the alias
        let last_segment = trait_alias.rsplit("::").next()?;
        implemented_traits.get(last_segment)
    });
    let types = match implementing_types {
        Some(t) => t,
        None => return false,
    };
    // Check if any type that implements this trait is used in the file
    let id_set: std::collections::HashSet<&str> =
        used_identifiers.iter().map(|s| s.as_str()).collect();
    types.iter().any(|t| id_set.contains(t.as_str()))
}

/// Fallback: check well-known trait patterns that tree-sitter can't resolve.
/// Covers std library traits, async_trait, and common naming patterns.
fn is_known_trait_pattern(raw_path: &str, alias_str: &str) -> bool {
    // ─── Well-known Rust trait paths ───
    if raw_path.contains("prelude")
        || raw_path.contains("async_trait")
        || raw_path.ends_with("::io::Write")
        || raw_path.ends_with("::fmt::Display")
        || raw_path.ends_with("::fmt::Debug")
        || raw_path.ends_with("::fmt::From")
        || raw_path.ends_with("::fmt::Into")
        || raw_path.ends_with("::clone::Clone")
        || raw_path.ends_with("::cmp::PartialEq")
        || raw_path.ends_with("::cmp::PartialOrd")
        || raw_path.ends_with("::ops::Add")
        || raw_path.ends_with("::ops::Deref")
    {
        return true;
    }
    // ─── Common trait suffix patterns ───
    if alias_str.ends_with("Ext")
        || alias_str.ends_with("Iterator")
        || alias_str.ends_with("Stream")
        || alias_str == "Write"
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_used_for_method_dispatch_detected() {
        let mut traits = HashMap::new();
        traits.insert(
            "CalculatorProtocol".to_string(),
            vec!["Calculator".to_string()],
        );
        let used_ids = vec!["Calculator".to_string(), "main".to_string()];
        assert!(is_trait_used_for_method_dispatch(
            "CalculatorProtocol",
            &traits,
            &used_ids,
        ));
    }

    #[test]
    fn trait_not_used_for_method_dispatch() {
        let mut traits = HashMap::new();
        traits.insert(
            "CalculatorProtocol".to_string(),
            vec!["Calculator".to_string()],
        );
        let used_ids = vec!["SomeOtherType".to_string()];
        assert!(!is_trait_used_for_method_dispatch(
            "CalculatorProtocol",
            &traits,
            &used_ids,
        ));
    }

    #[test]
    fn trait_not_in_project_not_dispatch() {
        let traits: HashMap<String, Vec<String>> = HashMap::new();
        let used_ids = vec!["Foo".to_string()];
        assert!(!is_trait_used_for_method_dispatch("Foo", &traits, &used_ids));
    }

    #[test]
    fn known_trait_pattern_std_prelude() {
        assert!(is_known_trait_pattern("std::prelude::v1::*", "*"));
        assert!(is_known_trait_pattern(
            r#"async_trait::async_trait"#,
            "async_trait"
        ));
        assert!(is_known_trait_pattern(
            r#"std::fmt::Display"#,
            "Display"
        ));
        assert!(is_known_trait_pattern(r#"std::fmt::Debug"#, "Debug"));
        assert!(is_known_trait_pattern(r#"std::clone::Clone"#, "Clone"));
        assert!(is_known_trait_pattern(r#"std::cmp::PartialEq"#, "PartialEq"));
        assert!(is_known_trait_pattern(
            r#"std::io::Write"#,
            "Write"
        ));
    }

    #[test]
    fn known_trait_pattern_suffix() {
        assert!(is_known_trait_pattern(r#"foo::BarExt"#, "BarExt"));
        assert!(is_known_trait_pattern(
            r#"foo::Stream"#,
            "Stream"
        ));
        assert!(!is_known_trait_pattern(r#"foo::MyTrait"#, "MyTrait"));
    }
}
