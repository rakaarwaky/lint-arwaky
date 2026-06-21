// PURPOSE: IImportParserPort — contract port trait for import parsing utilities
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::import_rules::taxonomy_language_vo::LanguageVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::taxonomy_common_vo::LineNumber;
use crate::taxonomy_layer_vo::FileContentVO;
use crate::taxonomy_layer_vo::Identity;
use crate::taxonomy_layer_vo::LayerNameVO;
use crate::taxonomy_layer_vo::LineContentVO;
use crate::taxonomy_name_vo::SymbolName;

pub trait IImportParserPort: Send + Sync {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>);
    fn import_matches_scope(
        &self,
        import_line: &LineContentVO,
        layer: &LayerNameVO,
        suffixes: &[Identity],
    ) -> bool;
    fn get_basename(&self, file: &FilePath) -> Identity;
    fn read_import_lines(&self, file: &FilePath) -> Vec<(LineNumber, LineContentVO)>;
    fn parse_import_lines(&self, content: &FileContentVO) -> Vec<(LineNumber, LineContentVO)>;
    fn extract_module_from_line(&self, line: &LineContentVO) -> Option<Identity>;
    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO>;

    // New methods to extract infrastructure Concerns
    fn read_file_to_string(&self, file: &FilePath) -> Result<String, std::io::Error>;
    fn extract_import_modules(&self, content: &str) -> Vec<String>;
    fn get_language_from_path(&self, path: &str) -> LanguageVO;
    fn get_dummy_function_ranges(&self, lines: &[&str], lang: LanguageVO) -> Vec<(usize, usize)>;
    fn get_imported_symbols(&self, lines: &[&str], lang: LanguageVO) -> Vec<(String, usize)>;
    fn get_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(String, usize)>;
    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(usize, usize)],
        dummy_impl_traits: &[String],
    ) -> bool;
    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    // Fine-grained parsing utilities for unused import steps
    fn extract_imported_aliases(&self, content: &str) -> std::collections::HashMap<String, String>;
    fn extract_exported_symbols(&self, content: &str) -> std::collections::HashSet<String>;
    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &std::collections::HashMap<String, String>,
    ) -> std::collections::HashSet<String>;
    fn find_import_line_number(&self, content: &str, alias: &str) -> usize;
    fn extract_rust_js_imports(&self, content: &str) -> Vec<(String, usize)>;
    fn is_name_used(&self, name: &str, content: &str, exclude_line: usize) -> bool;
}
