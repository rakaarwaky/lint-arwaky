// PURPOSE: IUnusedImportProtocol — unified port trait for AES203: detect unused imports across Rust, Python, JavaScript
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `Vec<String>` returns → `Vec<LintMessage>` (semantic messages, not raw strings)
//   * `&str file_path` params → kept as `&str` (idiomatic borrow, AES402 allows)
//   * `&mut Vec<LintResult>` → kept (`LintResult` is itself a VO)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use std::collections::{HashMap, HashSet};

pub trait IUnusedImportProtocol: Send + Sync {
    /// Find unused imports in a file by path (reads file internally).
    /// Returns a list of human-readable lint messages describing each unused
    /// import. Replaces the previous `Vec<String>` so callers can introspect,
    /// translate, or log messages without parsing free-form strings.
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage>;

    /// Check unused imports given file content directly (for inline checking).
    /// Useful when content is already available (avoids re-reading file).
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);

    // ═══════════════════════════════════════════════════════════════════════
    // Internal computation methods — needed for trait 1:1 matching
    // ═══════════════════════════════════════════════════════════════════════

    /// Check if a name is a Rust trait import (Protocol, Port, Trait, Aggregate, etc).
    fn is_rust_trait_import(&self, name: &str) -> bool;

    /// Extract imported aliases from file content.
    fn extract_imported_aliases(&self, content: &str) -> HashMap<Identity, Identity>;

    /// Extract exported symbols from file content.
    fn extract_exported_symbols(&self, content: &str) -> HashSet<Identity>;

    /// Extract used symbols from file content given imported aliases.
    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity>;

    /// Extract Rust/JS named imports.
    fn extract_rust_js_imports(&self, content: &str) -> Vec<(SymbolName, LineNumber)>;

    /// Check whether a symbol name is used in the content (excluding the import line).
    fn is_name_used(&self, name: &str, content: &str, exclude_line: usize) -> bool;
}
