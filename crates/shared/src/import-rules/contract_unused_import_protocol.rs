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
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage>;
    fn check_unused_imports(
        &self,
        file: &FilePath,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
    fn is_rust_trait_import(&self, name: &SymbolName) -> bool;
    fn extract_imported_aliases(&self, content: &str) -> HashMap<Identity, Identity>;
    fn extract_exported_symbols(&self, content: &str) -> HashSet<Identity>;
    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity>;
    fn extract_rust_js_imports(&self, content: &str) -> Vec<(SymbolName, LineNumber)>;
    fn is_name_used(&self, name: &SymbolName, content: &str, exclude_line: LineNumber) -> bool;
}
