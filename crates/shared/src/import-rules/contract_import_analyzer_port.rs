// PURPOSE: IImportAnalyzerPort — contract port trait for import analysis logic
//
// Infrastructure calls this via DI. This keeps computation in capabilities, I/O in infrastructure.

use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::import_rules::taxonomy_language_vo::LanguageVO;
use std::collections::{HashMap, HashSet};

pub trait IImportAnalyzerPort: Send + Sync {
    fn extract_import_modules(&self, content: &str) -> Vec<SymbolName>;

    fn get_dummy_function_ranges(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(LineNumber, LineNumber)>;

    fn get_imported_symbols(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(SymbolName, LineNumber)>;

    fn get_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)>;

    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(usize, usize)],
        dummy_impl_traits: &[String],
    ) -> bool;

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    fn extract_imported_aliases(&self, content: &str) -> HashMap<Identity, Identity>;

    fn extract_exported_symbols(&self, content: &str) -> HashSet<Identity>;

    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity>;

    fn extract_rust_js_imports(&self, content: &str) -> Vec<(SymbolName, LineNumber)>;

    fn is_name_used(&self, name: &str, content: &str, exclude_line: usize) -> bool;
}
