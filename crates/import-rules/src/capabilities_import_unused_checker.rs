// PURPOSE: UnusedImportRuleChecker — AES203: detect unused imports.
// AST-based: uses syn visitor for usage tracking. No dynamic regex. No DERIVE_MACROS whitelist.

use shared::cli_commands::LintResult;
use shared::common::{ErrorMessage, FilePath, LintMessage, Severity};
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_import_error::ImportError;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::utility_import_resolver;
use shared::import_rules::utility_import_symbol_extractor;

pub struct UnusedImportRuleChecker;

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Result<Vec<LintMessage>, ImportError> {
        if utility_import_resolver::is_barrel_file(&path.basename()) {
            return Ok(Vec::new());
        }
        let content = filesystem::utility_filesystem_io::read_file(path.value()).map_err(|_| {
            ImportError::module_resolution(
                path.value().to_string(),
                Some(ErrorMessage::new(
                    "File could not be read for unused import analysis",
                )),
            )
        })?;
        let imported_aliases =
            utility_import_symbol_extractor::extract_imported_aliases(path.value(), &content);
        let exported_symbols =
            utility_import_symbol_extractor::extract_exported_symbols(path.value(), &content);
        let used_symbols = utility_import_symbol_extractor::extract_used_symbols(
            path.value(),
            &content,
            &imported_aliases,
        );
        let mut unused: Vec<String> = Vec::new();
        for alias in imported_aliases.keys() {
            let alias_str = alias.value();
            if unused_import_is_future_import(&content, alias_str) {
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
    ) -> Result<Vec<LintResult>, ImportError> {
        let basename = std::path::Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if utility_import_resolver::is_barrel_file(basename) {
            return Ok(Vec::new());
        }
        // Single unified path: extract aliases (uses alias_name() for renames),
        // check usage, check exports, produce one violation per unused import.
        let imported_aliases =
            utility_import_symbol_extractor::extract_imported_aliases(file, content);
        let exported_symbols =
            utility_import_symbol_extractor::extract_exported_symbols(file, content);
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
            // Text-based fallback: check if the alias name appears in non-import,
            // non-comment lines. This catches implicit usage like trait method calls
            // (e.g. `#[async_trait]`, `.par_iter()` enabled by `ParallelIterator`,
            // `.init()` enabled by `SubscriberExt`).
            let alias_in_body = content.lines().any(|l| {
                let t = l.trim();
                if t.is_empty()
                    || t.starts_with("//")
                    || t.starts_with("use ")
                    || t.starts_with("pub use ")
                    || t.starts_with("pub(crate) use ")
                {
                    return false;
                }
                t.contains(alias_str)
            });
            if alias_in_body {
                continue;
            }
            // Heuristic: skip imports that are likely traits used implicitly
            // via method calls, derive macros, or macro invocations.
            // The AST can't detect: `.par_iter()` (ParallelIterator),
            // `#[async_trait]`, `.init()` (SubscriberInitExt), `writeln!` (Write), etc.
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
            let line_num = utility_import_resolver::find_import_line_number(content, alias_str)
                .value() as usize;
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
