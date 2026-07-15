use std::sync::Arc;

use import_rules_lint_arwaky::capabilities_dummy_import_checker::DummyImportChecker;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::import_rules::contract_dummy_import_checker_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------------------
// Mock parser (minimal implementation)
// ---------------------------------------------------------------------------
struct MockDummyParser;

impl IImportParserPort for MockDummyParser {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
        (LayerNameVO::new(scope.value()), vec![])
    }
    fn import_matches_scope(&self, _: &LineContentVO, _: &LayerNameVO, _: &[Identity]) -> bool {
        false
    }
    fn get_basename(&self, _: &FilePath) -> Identity {
        Identity::new("test.rs")
    }
    fn read_import_lines(&self, _: &FilePath) -> Vec<(LineNumber, LineContentVO)> {
        vec![]
    }
    fn parse_import_lines(&self, _: &FileContentVO) -> Vec<(LineNumber, LineContentVO)> {
        vec![]
    }
    fn extract_module_from_line(&self, _: &LineContentVO) -> Option<Identity> {
        None
    }
    fn extract_layer_from_import(&self, _: &Identity) -> Option<LayerNameVO> {
        None
    }
    fn read_file_to_message(&self, _: &FilePath) -> Result<LintMessage, std::io::Error> {
        Ok(LintMessage::new(""))
    }
    fn extract_import_modules(&self, _: &str) -> Vec<SymbolName> {
        vec![]
    }
    fn get_language_from_path(&self, _: &str) -> LanguageVO {
        LanguageVO::Rust
    }
    fn get_dummy_function_ranges(
        &self,
        _: &[&str],
        _: LanguageVO,
    ) -> Vec<(LineNumber, LineNumber)> {
        vec![]
    }
    fn get_imported_symbols(&self, _: &[&str], _: LanguageVO) -> Vec<(SymbolName, LineNumber)> {
        vec![]
    }
    fn get_dummy_impl_traits_with_lines(&self, _: &[&str]) -> Vec<(SymbolName, LineNumber)> {
        vec![]
    }
    fn is_symbol_used_real(
        &self,
        _: &[&str],
        _: &str,
        _: &[(LineNumber, LineNumber)],
        _: &[String],
    ) -> bool {
        false
    }
    fn detect_cycle_edges(&self, _: &[DependencyEdge]) -> Vec<SymbolName> {
        vec![]
    }
    fn extract_imported_aliases(&self, _: &str) -> HashMap<Identity, Identity> {
        HashMap::new()
    }
    fn extract_exported_symbols(&self, _: &str) -> HashSet<Identity> {
        HashSet::new()
    }
    fn extract_used_symbols(&self, _: &str, _: &HashMap<Identity, Identity>) -> HashSet<Identity> {
        HashSet::new()
    }
    fn find_import_line_number(&self, _: &str, _: &str) -> LineNumber {
        LineNumber::new(0)
    }
    fn extract_rust_js_imports(&self, _: &str) -> Vec<(SymbolName, LineNumber)> {
        vec![]
    }
    fn is_name_used(&self, _: &str, _: &str, _: LineNumber) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn rule_name_is_aes204() {
    let parser = MockDummyParser;
    let checker = DummyImportChecker::new(Arc::new(parser));
    assert_eq!(checker.rule_name().value(), "AES204");
}

#[test]
fn checker_constructs_with_parser() {
    let parser = MockDummyParser;
    let checker = DummyImportChecker::new(Arc::new(parser));
    assert_eq!(checker.rule_name().value(), "AES204");
}
