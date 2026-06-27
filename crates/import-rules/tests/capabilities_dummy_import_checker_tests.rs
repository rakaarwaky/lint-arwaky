use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use import_rules_lint_arwaky::capabilities_dummy_import_checker::DummyImportChecker;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::{Identity, LayerNameVO, LineContentVO};
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::{ArchitectureConfig, ConfigEnabled};
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::{IAnalyzer, IArchRuleProtocol};
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::cli_commands::taxonomy_result_vo::LintResultList;

// ---------------------------------------------------------------------------
// Mock parser
// ---------------------------------------------------------------------------
struct MockDummyParser {
    dummy_ranges: Vec<(LineNumber, LineNumber)>,
    imported_symbols: Vec<(SymbolName, LineNumber)>,
    dummy_impl_traits: Vec<(SymbolName, LineNumber)>,
    symbol_used_real: bool,
    content: String,
    lang: LanguageVO,
}

impl MockDummyParser {
    fn new() -> Self {
        Self {
            dummy_ranges: vec![],
            imported_symbols: vec![],
            dummy_impl_traits: vec![],
            symbol_used_real: false,
            content: String::new(),
            lang: LanguageVO::Rust,
        }
    }
}

impl IImportParserPort for MockDummyParser {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
        (LayerNameVO::new(scope.value()), vec![])
    }
    fn import_matches_scope(&self, _i: &LineContentVO, _l: &LayerNameVO, _s: &[Identity]) -> bool { false }
    fn get_basename(&self, _f: &FilePath) -> Identity { Identity::new("test.rs") }
    fn read_import_lines(&self, _f: &FilePath) -> Vec<(LineNumber, LineContentVO)> { vec![] }
    fn parse_import_lines(&self, _c: &FileContentVO) -> Vec<(LineNumber, LineContentVO)> { vec![] }
    fn extract_module_from_line(&self, _l: &LineContentVO) -> Option<Identity> { None }
    fn extract_layer_from_import(&self, _s: &Identity) -> Option<LayerNameVO> { None }
    fn read_file_to_message(&self, _f: &FilePath) -> Result<LintMessage, std::io::Error> {
        Ok(LintMessage::new(self.content.clone()))
    }
    fn extract_import_modules(&self, _c: &str) -> Vec<SymbolName> { vec![] }
    fn get_language_from_path(&self, _p: &str) -> LanguageVO { self.lang }
    fn get_dummy_function_ranges(&self, _l: &[&str], _lang: LanguageVO) -> Vec<(LineNumber, LineNumber)> {
        self.dummy_ranges.clone()
    }
    fn get_imported_symbols(&self, _l: &[&str], _lang: LanguageVO) -> Vec<(SymbolName, LineNumber)> {
        self.imported_symbols.clone()
    }
    fn get_dummy_impl_traits_with_lines(&self, _l: &[&str]) -> Vec<(SymbolName, LineNumber)> {
        self.dummy_impl_traits.clone()
    }
    fn is_symbol_used_real(&self, _l: &[&str], _s: &str, _d: &[(LineNumber, LineNumber)], _di: &[String]) -> bool {
        self.symbol_used_real
    }
    fn detect_cycle_edges(&self, _e: &[DependencyEdge]) -> Vec<SymbolName> { vec![] }
    fn extract_imported_aliases(&self, _c: &str) -> HashMap<Identity, Identity> { HashMap::new() }
    fn extract_exported_symbols(&self, _c: &str) -> HashSet<Identity> { HashSet::new() }
    fn extract_used_symbols(&self, _c: &str, _i: &HashMap<Identity, Identity>) -> HashSet<Identity> { HashSet::new() }
    fn find_import_line_number(&self, _c: &str, _a: &str) -> LineNumber { LineNumber::new(0) }
    fn extract_rust_js_imports(&self, _c: &str) -> Vec<(SymbolName, LineNumber)> { vec![] }
    fn is_name_used(&self, _n: &str, _c: &str, _e: LineNumber) -> bool { false }
}
use shared::common::taxonomy_layer_vo::FileContentVO;

// ---------------------------------------------------------------------------
// Mock analyzer
// ---------------------------------------------------------------------------
struct MockAnalyzer;

impl IAnalyzer for MockAnalyzer {
    fn config(&self) -> &ArchitectureConfig {
        // Use a static config since we can't create one in a const context
        static CONFIG: ArchitectureConfig = ArchitectureConfig {
            enabled: ConfigEnabled::new(true),
            layers: LayerMapVO::new(),
            rules: Vec::new(),
        };
        &CONFIG
    }
    fn detect_layer(&self, _path: &FilePath, _root: &FilePath) -> Option<LayerNameVO> {
        Some(LayerNameVO::new("capabilities"))
    }
    fn layer_map(&self) -> &LayerMapVO {
        static MAP: LayerMapVO = LayerMapVO::new();
        &MAP
    }
}

fn make_root_dir() -> FilePath {
    FilePath::new(".").unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn dummy_skips_self_check_file() {
    let parser = MockDummyParser::new();
    let checker = DummyImportChecker::new(Arc::new(parser));
    let violations = vec![];
    let files = FilePathList::new(vec![
        FilePath::new("/path/to/capabilities_dummy_import_checker.rs").unwrap_or_default()
    ]);
    let mut results = LintResultList::new(violations);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(checker.check_mandatory_imports(
        &MockAnalyzer,
        &files,
        &make_root_dir(),
        &mut results,
    ));
    // No violations since the self-check file is skipped
    assert!(results.values.is_empty());
}

#[test]
fn rule_name_is_aes204() {
    let parser = MockDummyParser::new();
    let checker = DummyImportChecker::new(Arc::new(parser));
    assert_eq!(checker.rule_name().value(), "AES204");
}

#[test]
fn checker_constructs_with_parser() {
    let parser = MockDummyParser::new();
    let checker = DummyImportChecker::new(Arc::new(parser));
    assert_eq!(checker.rule_name().value(), "AES204");
}
