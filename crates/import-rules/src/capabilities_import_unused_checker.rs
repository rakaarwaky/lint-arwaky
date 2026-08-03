// PURPOSE: UnusedImportRuleChecker — AES203: detect unused imports.
// AST-based: uses syn visitor for usage tracking. No dynamic regex. No DERIVE_MACROS whitelist.

use crate::utility_import_resolver;
use crate::utility_import_symbol_extractor;
use shared::cli_commands::LintResult;
use shared::common::{FilePath, LintMessage, Severity};
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_import_error::ImportError;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;

pub struct UnusedImportRuleChecker;

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(
        &self,
        path: &FilePath,
        content: &str,
        import_entries: &[ImportEntry],
    ) -> Result<Vec<LintMessage>, ImportError> {
        if utility_import_resolver::is_barrel_file(&path.basename()) {
            return Ok(Vec::new());
        }
        // Use ImportEntry from filesystem if available, fallback to line-based
        let imported_aliases = if !import_entries.is_empty() {
            utility_import_symbol_extractor::extract_imported_aliases_from_entries(import_entries)
        } else {
            utility_import_symbol_extractor::extract_imported_aliases(path.value(), content)
        };
        let exported_symbols = if !import_entries.is_empty() {
            utility_import_symbol_extractor::extract_exported_symbols_from_entries(import_entries)
        } else {
            utility_import_symbol_extractor::extract_exported_symbols(path.value(), content)
        };
        let used_symbols = utility_import_symbol_extractor::extract_used_symbols(
            path.value(),
            content,
            &imported_aliases,
        );
        let mut unused: Vec<String> = Vec::new();
        for alias in imported_aliases.keys() {
            let alias_str = alias.value();
            if unused_import_is_future_import(content, alias_str) {
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
    ) -> Result<Vec<LintResult>, ImportError> {
        let basename = std::path::Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if utility_import_resolver::is_barrel_file(basename) {
            return Ok(Vec::new());
        }
        // Use ImportEntry from filesystem if available, fallback to line-based
        let imported_aliases = if !import_entries.is_empty() {
            utility_import_symbol_extractor::extract_imported_aliases_from_entries(import_entries)
        } else {
            utility_import_symbol_extractor::extract_imported_aliases(file, content)
        };
        let exported_symbols = if !import_entries.is_empty() {
            utility_import_symbol_extractor::extract_exported_symbols_from_entries(import_entries)
        } else {
            utility_import_symbol_extractor::extract_exported_symbols(file, content)
        };
        let used_symbols =
            utility_import_symbol_extractor::extract_used_symbols(file, content, &imported_aliases);
        let mut violations = Vec::new();
        for alias in imported_aliases.keys() {
            let alias_str = alias.value();
            if unused_import_is_future_import(content, alias_str) {
                continue;
            }
            if used_symbols.contains(alias) || exported_symbols.contains(alias) {
                continue;
            }
            let alias_in_body = content.lines().any(|l| {
                let t = l.trim();
                if t.is_empty()
                    || t.starts_with("//")
                    || t.starts_with("#")
                    || t.starts_with("use ")
                    || t.starts_with("pub use ")
                    || t.starts_with("pub(crate) use ")
                    || t.starts_with("import ")
                    || t.starts_with("from ")
                    || t.starts_with("export ")
                {
                    return false;
                }
                t.contains(alias_str)
            });
            if alias_in_body {
                continue;
            }
            if let Some(raw_path) = imported_aliases.get(alias) {
                let rp = raw_path.value();
                let is_likely_trait = rp.contains("prelude")
                    || rp.contains("async_trait")
                    || rp.ends_with("::io::Write")
                    || alias_str.ends_with("Ext")
                    || alias_str.ends_with("Iterator")
                    || alias_str.ends_with("Stream")
                    || alias_str == "Write";
                if is_likely_trait {
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
                AesImportViolation::FixUnusedImport {
                    reason: Some(LintMessage::new(format!(
                        "Import '{}' is declared but never used in this file.",
                        alias_str
                    ))),
                }
                .to_string(),
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

fn unused_import_is_future_import(content: &str, alias: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("from __future__ import ")
            && (trimmed == format!("from __future__ import {}", alias)
                || trimmed.contains(format!(", {}", alias).as_str())
                || trimmed.contains(format!(" {},", alias).as_str()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_unused_rust_import() {
        let checker = UnusedImportRuleChecker::new();
        // std::collections::HashMap is imported but never used in the body
        let content = r#"use std::collections::HashMap;

fn main() {
    println!("hello");
}
"#;
        let result = checker
            .check_unused_imports("/tmp/test/src/app.rs", content, &[])
            .unwrap();
        assert!(!result.is_empty(), "Should detect unused HashMap import");
        assert_eq!(result[0].code.code(), "AES203");
        assert!(
            result[0].message.value.contains("HashMap"),
            "Violation message should mention HashMap, got: {}",
            result[0].message
        );
    }

    #[test]
    fn no_violation_when_import_is_used() {
        let checker = UnusedImportRuleChecker::new();
        let content = r#"use std::collections::HashMap;

fn main() {
    let _map = HashMap::new();
}
"#;
        let result = checker
            .check_unused_imports("/tmp/test/src/main.rs", content, &[])
            .unwrap();
        assert!(
            result.is_empty(),
            "Used import should produce no violations, got {}",
            result.len()
        );
    }

    #[test]
    fn no_violation_for_barrel_files() {
        let checker = UnusedImportRuleChecker::new();
        // lib.rs / mod.rs are barrel files and should be skipped
        let content = "use something::unused;\n";
        let result_lib = checker
            .check_unused_imports("/tmp/test/src/lib.rs", content, &[])
            .unwrap();
        let result_mod = checker
            .check_unused_imports("/tmp/test/src/mod.rs", content, &[])
            .unwrap();
        assert!(result_lib.is_empty(), "lib.rs should be skipped");
        assert!(result_mod.is_empty(), "mod.rs should be skipped");
    }

    #[test]
    fn no_violation_for_empty_content() {
        let checker = UnusedImportRuleChecker::new();
        let result = checker
            .check_unused_imports("/tmp/test/src/file.rs", "", &[])
            .unwrap();
        assert!(
            result.is_empty(),
            "Empty content should produce no violations"
        );
    }

    #[test]
    fn detects_multiple_unused_imports() {
        let checker = UnusedImportRuleChecker::new();
        let content = r#"use std::collections::HashMap;
use std::collections::BTreeMap;
use std::io::Read;

fn main() {
    println!("no imports used");
}
"#;
        let result = checker
            .check_unused_imports("/tmp/test/src/multi.rs", content, &[])
            .unwrap();
        // At least HashMap and BTreeMap should be flagged (Read is a trait — may be skipped)
        assert!(
            result.len() >= 2,
            "Should detect at least 2 unused imports, got {}",
            result.len()
        );
    }
}
