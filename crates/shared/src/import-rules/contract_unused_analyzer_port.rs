// PURPOSE: IUnusedAnalyzerPort — contract trait for unused import detection
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_name_vo::SymbolName;
use std::collections::{HashMap, HashSet};

pub trait IUnusedAnalyzerPort: Send + Sync {
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
