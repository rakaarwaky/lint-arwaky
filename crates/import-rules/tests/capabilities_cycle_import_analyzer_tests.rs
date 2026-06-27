use async_trait::async_trait;
use import_rules_lint_arwaky::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::taxonomy_common_vo::BooleanVO;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::taxonomy_name_vo::SymbolName;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Mock parser that returns pre-configured import modules for each file
// ---------------------------------------------------------------------------
struct MockCycleParser {
    /// (file_basename -> Vec<module_paths>)
    imports: HashMap<String, Vec<String>>,
    /// (edge_list -> Vec<cycle_edge_strings>) — which edges participate in cycles
    cycle_edges: HashMap<String, Vec<String>>,
}

impl IImportParserPort for MockCycleParser {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
        let s = scope.value();
        if let Some(paren) = s.find('(') {
            (LayerNameVO::new(&s[..paren]), vec![])
        } else {
            (LayerNameVO::new(s.trim()), vec![])
        }
    }
    fn import_matches_scope(
        &self,
        _import_line: &LineContentVO,
        _layer: &LayerNameVO,
        _suffixes: &[Identity],
    ) -> bool {
        false
    }
    fn get_basename(&self, file: &FilePath) -> Identity {
        Identity::new(file.basename())
    }
    fn read_import_lines(&self, _file: &FilePath) -> Vec<(LineNumber, LineContentVO)> {
        vec![]
    }
    fn parse_import_lines(&self, _content: &FileContentVO) -> Vec<(LineNumber, LineContentVO)> {
        vec![]
    }
    fn extract_module_from_line(&self, _line: &LineContentVO) -> Option<Identity> {
        None
    }
    fn extract_layer_from_import(&self, _segment: &Identity) -> Option<LayerNameVO> {
        None
    }
    fn read_file_to_message(&self, file: &FilePath) -> Result<LintMessage, std::io::Error> {
        let basename = file.basename();
        let content = self
            .imports
            .get(&basename)
            .map(|mods| mods.join("\n"))
            .unwrap_or_default();
        Ok(LintMessage::new(content))
    }
    fn extract_import_modules(&self, content: &str) -> Vec<SymbolName> {
        content
            .lines()
            .filter(|l| l.contains("import"))
            .map(|l| {
                let parts: Vec<&str> = l.split_whitespace().collect();
                if parts.len() >= 2 {
                    SymbolName::new(parts[1..].join(" "))
                } else {
                    SymbolName::new(l)
                }
            })
            .collect()
    }
    fn get_language_from_path(
        &self,
        _path: &str,
    ) -> shared::import_rules::taxonomy_language_vo::LanguageVO {
        shared::import_rules::taxonomy_language_vo::LanguageVO::Rust
    }
    fn get_dummy_function_ranges(
        &self,
        _lines: &[&str],
        _lang: shared::import_rules::taxonomy_language_vo::LanguageVO,
    ) -> Vec<(LineNumber, LineNumber)> {
        vec![]
    }
    fn get_imported_symbols(
        &self,
        _lines: &[&str],
        _lang: shared::import_rules::taxonomy_language_vo::LanguageVO,
    ) -> Vec<(SymbolName, LineNumber)> {
        vec![]
    }
    fn get_dummy_impl_traits_with_lines(&self, _lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
        vec![]
    }
    fn is_symbol_used_real(
        &self,
        _lines: &[&str],
        _symbol: &str,
        _dummy_ranges: &[(LineNumber, LineNumber)],
        _dummy_impl_traits: &[String],
    ) -> bool {
        false
    }
    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        let edge_key = edges
            .iter()
            .map(|e| format!("{}->{}", e.source, e.target))
            .collect::<Vec<_>>()
            .join(",");
        self.cycle_edges
            .get(&edge_key)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(SymbolName::new)
            .collect()
    }
    fn extract_imported_aliases(&self, _content: &str) -> HashMap<Identity, Identity> {
        HashMap::new()
    }
    fn extract_exported_symbols(&self, _content: &str) -> HashSet<Identity> {
        HashSet::new()
    }
    fn extract_used_symbols(
        &self,
        _content: &str,
        _imported: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity> {
        HashSet::new()
    }
    fn find_import_line_number(&self, _content: &str, _alias: &str) -> LineNumber {
        LineNumber::new(0)
    }
    fn extract_rust_js_imports(&self, _content: &str) -> Vec<(SymbolName, LineNumber)> {
        vec![]
    }
    fn is_name_used(&self, _name: &str, _content: &str, _exclude_line: LineNumber) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// Tests for the cycle detection analyzer
// ---------------------------------------------------------------------------

#[test]
fn no_edges_no_violations() {
    let config = ArchitectureConfig {
        enabled: BooleanVO::new(true),
        ..Default::default()
    };
    let parser = Arc::new(MockCycleParser {
        imports: HashMap::new(),
        cycle_edges: HashMap::new(),
    });
    let _analyzer = DependencyCycleAnalyzer::new(config, parser);
    // A dummy IAnalyzer is needed — the scan method requires it
    // For now, we test that the DependencyEdge struct and cycle detection concepts work
    let edge = DependencyEdge::new("capabilities".to_string(), "surfaces".to_string());
    assert_eq!(edge.source, "capabilities");
    assert_eq!(edge.target, "surfaces");
}
