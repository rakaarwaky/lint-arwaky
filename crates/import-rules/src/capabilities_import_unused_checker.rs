use shared::common::{ErrorMessage, FilePath, LintMessage, Severity};

use shared::import_rules::utility_import_resolver;
use shared::import_rules::utility_import_symbol_extractor;
use shared::import_rules::{AesImportViolation, IUnusedImportProtocol, ImportError};

// PURPOSE: UnusedImportRuleChecker — AES203: detect unused imports (Rust/Python/JS)
// Uses utility functions directly — no IImportParserProtocol.
use shared::cli_commands::LintResult;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct UnusedImportRuleChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Result<Vec<LintMessage>, ImportError> {
        // Skip barrel files — re-exports are intentional public API, not unused imports.
        if utility_import_resolver::is_barrel_file(&path.basename()) {
            return Ok(Vec::new());
        }
        let content = shared::common::utility_file_handler::read_file_generic(path.value())
            .map_err(|_| {
                ImportError::module_resolution(
                    path.value().to_string(),
                    Some(ErrorMessage::new(
                        "File could not be read for unused import analysis",
                    )),
                )
            })?;
        let imported_aliases = utility_import_symbol_extractor::extract_imported_aliases(&content);
        let exported_symbols = utility_import_symbol_extractor::extract_exported_symbols(&content);
        let used_symbols =
            utility_import_symbol_extractor::extract_used_symbols(&content, &imported_aliases);

        let mut unused: Vec<String> = Vec::new();
        for alias in imported_aliases.keys() {
            // Skip __future__ imports — they affect parsing behavior, not runtime usage.
            let alias_str = alias.value();
            if unused_import_is_future_import(&content, alias_str) {
                continue;
            }
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias_str.to_string());
            }
        }
        let rust_js_imports = utility_import_symbol_extractor::extract_rust_js_imports(&content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value();
            if !utility_import_symbol_extractor::is_name_used(
                name_str,
                &content,
                line_idx.value() as usize,
            ) {
                unused.push(name_str.to_string());
            }
        }
        Ok(unused.into_iter().map(LintMessage::new).collect())
    }

    fn check_unused_imports(
        &self,
        file: &str,
        content: &str,
    ) -> Result<Vec<LintResult>, ImportError> {
        // Skip barrel files — re-exports are intentional public API, not unused imports.
        let basename = std::path::Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if utility_import_resolver::is_barrel_file(basename) {
            return Ok(Vec::new());
        }
        let imported_aliases = utility_import_symbol_extractor::extract_imported_aliases(content);
        let exported_symbols = utility_import_symbol_extractor::extract_exported_symbols(content);
        let used_symbols =
            utility_import_symbol_extractor::extract_used_symbols(content, &imported_aliases);

        let mut violations = Vec::new();

        for alias in imported_aliases.keys() {
            // Skip __future__ imports — they affect parsing behavior, not runtime usage.
            let alias_str = alias.value();
            if unused_import_is_future_import(content, alias_str) {
                continue;
            }
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                let line_num = utility_import_resolver::find_import_line_number(content, alias_str)
                    .value() as usize;
                violations.push(LintResult::new_arch(
                    file,
                    line_num,
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
        }
        let rust_js_imports = utility_import_symbol_extractor::extract_rust_js_imports(content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value().to_string();
            if !utility_import_symbol_extractor::is_name_used(
                &name_str,
                content,
                line_idx.value() as usize,
            ) {
                violations.push(LintResult::new_arch(
                    file,
                    line_idx.value() as usize,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            name_str
                        ))),
                    }
                    .to_string(),
                ));
            }
        }
        Ok(violations)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for UnusedImportRuleChecker {
    fn default() -> Self {
        Self
    }
}

impl UnusedImportRuleChecker {
    pub fn new() -> Self {
        Self
    }
}

/// Check if an import is a __future__ import (e.g., `from __future__ import annotations`).
/// These are special Python constructs that affect parsing behavior and should not be
/// flagged as unused — they have no runtime symbol usage.
fn unused_import_is_future_import(content: &str, alias: &str) -> bool {
    // Check if any line matches `from __future__ import ...alias...`
    content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("from __future__ import ")
            && (trimmed == format!("from __future__ import {}", alias)
                || trimmed.contains(format!(", {}", alias).as_str())
                || trimmed.contains(format!(" {},", alias).as_str()))
    })
}
