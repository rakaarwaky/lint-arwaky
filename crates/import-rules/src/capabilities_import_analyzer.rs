// PURPOSE: ImportAnalyzer — capabilities layer that wraps all import analysis logic
//
// Infrastructure calls this via DI through IImportAnalyzerPort.
// Receives all dependencies via DI, does NOT import other capabilities.
// Uses pub(crate) helper functions from capabilities_dummy_import_checker for dummy analysis.

use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_import_analyzer_port::IImportAnalyzerPort;
use shared::import_rules::contract_parser_processor_port::IParserProcessorPort;
use shared::import_rules::contract_unused_analyzer_port::IUnusedAnalyzerPort;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct ImportAnalyzer {
    cycle: Arc<dyn ICycleImportProtocol>,
    parser: Arc<dyn IParserProcessorPort>,
    unused: Arc<dyn IUnusedAnalyzerPort>,
}

impl ImportAnalyzer {
    pub fn new(
        cycle: Arc<dyn ICycleImportProtocol>,
        parser: Arc<dyn IParserProcessorPort>,
        unused: Arc<dyn IUnusedAnalyzerPort>,
    ) -> Self {
        Self {
            cycle,
            parser,
            unused,
        }
    }
}

impl IImportAnalyzerPort for ImportAnalyzer {
    fn extract_import_modules(&self, content: &str) -> Vec<SymbolName> {
        self.parser.extract_import_modules(content)
    }

    fn get_dummy_function_ranges(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(LineNumber, LineNumber)> {
        crate::capabilities_dummy_import_checker::dummy_function_ranges(lines, lang)
    }

    fn get_imported_symbols(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(SymbolName, LineNumber)> {
        crate::capabilities_dummy_import_checker::imported_symbols(lines, lang)
    }

    fn get_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
        crate::capabilities_dummy_import_checker::dummy_impl_traits_with_lines(lines)
    }

    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(usize, usize)],
        dummy_impl_traits: &[String],
    ) -> bool {
        crate::capabilities_dummy_import_checker::symbol_used_real(
            lines,
            symbol,
            dummy_ranges,
            dummy_impl_traits,
        )
    }

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        self.cycle.pure_detect_cycle_edges(edges)
    }

    fn extract_imported_aliases(&self, content: &str) -> HashMap<Identity, Identity> {
        self.unused.extract_imported_aliases(content)
    }

    fn extract_exported_symbols(&self, content: &str) -> HashSet<Identity> {
        self.unused.extract_exported_symbols(content)
    }

    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity> {
        self.unused.extract_used_symbols(content, imported_aliases)
    }

    fn extract_rust_js_imports(&self, content: &str) -> Vec<(SymbolName, LineNumber)> {
        self.unused.extract_rust_js_imports(content)
    }

    fn is_name_used(&self, name: &str, content: &str, exclude_line: usize) -> bool {
        self.unused.is_name_used(name, content, exclude_line)
    }
}
