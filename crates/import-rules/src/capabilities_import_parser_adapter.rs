// PURPOSE: ImportParserAdapter — thin delegation layer to shared utility functions
// This module delegates all parsing logic to shared utility functions.
// The adapter struct is kept for backward compatibility but is now a thin wrapper.

use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::import_rules::{
    taxonomy_cycle_helper, taxonomy_dummy_helper, taxonomy_parser_helper, taxonomy_unused_helper,
    utility_import_resolver,
};
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::taxonomy_name_vo::SymbolName;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;

thread_local! {
    static FILE_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub fn clear_file_cache() {
    FILE_CACHE.with(|c| c.borrow_mut().clear());
}

pub struct ImportParserAdapter;

impl ImportParserAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ImportParserAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl shared::import_rules::contract_import_parser_protocol::IImportParserProtocol for ImportParserAdapter {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
        utility_import_resolver::resolve_scope(scope)
    }

    fn import_matches_scope(
        &self,
        import_line: &LineContentVO,
        layer: &LayerNameVO,
        suffixes: &[Identity],
    ) -> bool {
        utility_import_resolver::import_matches_scope(import_line, layer, suffixes)
    }

    fn get_basename(&self, file: &FilePath) -> Identity {
        Identity::new(file.basename())
    }

    fn read_import_lines(&self, file: &FilePath) -> Vec<(LineNumber, LineContentVO)> {
        let Ok(content) = fs::read_to_string(file.value()) else {
            return vec![];
        };
        utility_import_resolver::parse_import_lines_helper(&content)
    }

    fn parse_import_lines(&self, content: &FileContentVO) -> Vec<(LineNumber, LineContentVO)> {
        utility_import_resolver::parse_import_lines_helper(content.value())
    }

    fn extract_module_from_line(&self, line: &LineContentVO) -> Option<Identity> {
        utility_import_resolver::extract_module_from_line(line)
    }

    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO> {
        utility_import_resolver::extract_layer_from_import(segment)
    }

    fn read_file_to_message(&self, file: &FilePath) -> Result<LintMessage, std::io::Error> {
        let path = file.value().to_string();
        let content = FILE_CACHE.with(|cache| -> Result<String, std::io::Error> {
            let mut cache = cache.borrow_mut();
            if let Some(cached) = cache.get(&path) {
                return Ok(cached.clone());
            }
            let raw = fs::read_to_string(&path)?;
            cache.insert(path, raw.clone());
            Ok(raw)
        })?;
        Ok(LintMessage::new(content))
    }

    fn extract_import_modules(&self, content: &str) -> Vec<SymbolName> {
        taxonomy_parser_helper::extract_import_modules(content)
    }

    fn get_language_from_path(&self, path: &str) -> LanguageVO {
        LanguageVO::from_path(path)
    }

    fn get_dummy_function_ranges(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(LineNumber, LineNumber)> {
        taxonomy_dummy_helper::dummy_function_ranges(lines, lang)
    }

    fn get_imported_symbols(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(SymbolName, LineNumber)> {
        taxonomy_dummy_helper::imported_symbols(lines, lang)
    }

    fn get_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
        taxonomy_dummy_helper::dummy_impl_traits_with_lines(lines)
    }

    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(LineNumber, LineNumber)],
        dummy_impl_traits: &[String],
    ) -> bool {
        let converted: Vec<(usize, usize)> = dummy_ranges
            .iter()
            .map(|(s, e)| (s.value() as usize, e.value() as usize))
            .collect();
        taxonomy_dummy_helper::symbol_used_real(lines, symbol, &converted, dummy_impl_traits)
    }

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        taxonomy_cycle_helper::detect_cycle_edges(edges)
    }

    fn extract_imported_aliases(&self, content: &str) -> HashMap<Identity, Identity> {
        taxonomy_unused_helper::extract_imported_aliases(content)
    }

    fn extract_exported_symbols(&self, content: &str) -> HashSet<Identity> {
        taxonomy_unused_helper::extract_exported_symbols(content)
    }

    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity> {
        taxonomy_unused_helper::extract_used_symbols(content, imported_aliases)
    }

    fn find_import_line_number(&self, content: &str, alias: &str) -> LineNumber {
        utility_import_resolver::find_import_line_number(content, alias)
    }

    fn extract_rust_js_imports(&self, content: &str) -> Vec<(SymbolName, LineNumber)> {
        taxonomy_unused_helper::extract_rust_js_imports(content)
    }

    fn is_name_used(&self, name: &str, content: &str, exclude_line: LineNumber) -> bool {
        taxonomy_unused_helper::is_name_used(name, content, exclude_line.value() as usize)
    }
}
