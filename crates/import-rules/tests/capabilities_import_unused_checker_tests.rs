use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;

// ---------------------------------------------------------------------------
// Mock parser
// ---------------------------------------------------------------------------
struct MockUnusedParser {
    content: String,
    imported_aliases: HashMap<Identity, Identity>,
    exported_symbols: HashSet<Identity>,
    used_symbols: HashSet<Identity>,
    rust_js_imports: Vec<(SymbolName, LineNumber)>,
    name_used: bool,
    find_line: i64,
}

impl MockUnusedParser {
    fn new() -> Self {
        Self {
            content: String::new(),
            imported_aliases: HashMap::new(),
            exported_symbols: HashSet::new(),
            used_symbols: HashSet::new(),
            rust_js_imports: vec![],
            name_used: false,
            find_line: 0,
        }
    }
}

impl IImportParserPort for MockUnusedParser {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
        (LayerNameVO::new(scope.value()), vec![])
    }
    fn import_matches_scope(&self, _: &LineContentVO, _: &LayerNameVO, _: &[Identity]) -> bool { false }
    fn get_basename(&self, _: &FilePath) -> Identity { Identity::new("test.rs") }
    fn read_import_lines(&self, _: &FilePath) -> Vec<(LineNumber, LineContentVO)> { vec![] }
    fn parse_import_lines(&self, _: &FileContentVO) -> Vec<(LineNumber, LineContentVO)> { vec![] }
    fn extract_module_from_line(&self, _: &LineContentVO) -> Option<Identity> { None }
    fn extract_layer_from_import(&self, _: &Identity) -> Option<LayerNameVO> { None }
    fn read_file_to_message(&self, _: &FilePath) -> Result<LintMessage, std::io::Error> {
        Ok(LintMessage::new(self.content.clone()))
    }
    fn extract_import_modules(&self, _: &str) -> Vec<SymbolName> { vec![] }
    fn get_language_from_path(&self, _: &str) -> LanguageVO { LanguageVO::Rust }
    fn get_dummy_function_ranges(&self, _: &[&str], _: LanguageVO) -> Vec<(LineNumber, LineNumber)> { vec![] }
    fn get_imported_symbols(&self, _: &[&str], _: LanguageVO) -> Vec<(SymbolName, LineNumber)> { vec![] }
    fn get_dummy_impl_traits_with_lines(&self, _: &[&str]) -> Vec<(SymbolName, LineNumber)> { vec![] }
    fn is_symbol_used_real(&self, _: &[&str], _: &str, _: &[(LineNumber, LineNumber)], _: &[String]) -> bool { false }
    fn detect_cycle_edges(&self, _: &[DependencyEdge]) -> Vec<SymbolName> { vec![] }
    fn extract_imported_aliases(&self, _: &str) -> HashMap<Identity, Identity> {
        self.imported_aliases.clone()
    }
    fn extract_exported_symbols(&self, _: &str) -> HashSet<Identity> {
        self.exported_symbols.clone()
    }
    fn extract_used_symbols(&self, _: &str, _: &HashMap<Identity, Identity>) -> HashSet<Identity> {
        self.used_symbols.clone()
    }
    fn find_import_line_number(&self, _: &str, _: &str) -> LineNumber {
        LineNumber::new(self.find_line)
    }
    fn extract_rust_js_imports(&self, _: &str) -> Vec<(SymbolName, LineNumber)> {
        self.rust_js_imports.clone()
    }
    fn is_name_used(&self, _: &str, _: &str, _: LineNumber) -> bool {
        self.name_used
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn unused_detection_returns_empty_when_all_used() {
    let mut parser = MockUnusedParser::new();
    parser.content = "import os\nimport sys\n\nos.getcwd()\n".to_string();
    let mut aliases = HashMap::new();
    aliases.insert(Identity::new("os"), Identity::new("os"));
    parser.imported_aliases = aliases;
    let mut used = HashSet::new();
    used.insert(Identity::new("os"));
    parser.used_symbols = used;

    let checker = UnusedImportRuleChecker::new(Arc::new(parser));
    let path = FilePath::new("test.py").unwrap_or_default();
    let unused = checker.find_unused_imports(&path);
    assert!(unused.is_empty(), "all imports used");
}

#[test]
fn unused_detection_finds_unused_import() {
    let mut parser = MockUnusedParser::new();
    parser.content = "import os\nimport sys\n\nos.getcwd()\n".to_string();
    let mut aliases = HashMap::new();
    aliases.insert(Identity::new("os"), Identity::new("os"));
    aliases.insert(Identity::new("sys"), Identity::new("sys"));
    parser.imported_aliases = aliases;
    let mut used = HashSet::new();
    used.insert(Identity::new("os"));
    parser.used_symbols = used;

    let checker = UnusedImportRuleChecker::new(Arc::new(parser));
    let path = FilePath::new("test.py").unwrap_or_default();
    let unused = checker.find_unused_imports(&path);
    assert_eq!(unused.len(), 1, "sys should be flagged as unused");
    assert!(unused[0].value().contains("sys"));
}

#[test]
fn unused_exported_symbol_not_flagged() {
    let mut parser = MockUnusedParser::new();
    parser.content = "import os\nimport sys\n\n__all__ = ['os']\n".to_string();
    let mut aliases = HashMap::new();
    aliases.insert(Identity::new("os"), Identity::new("os"));
    aliases.insert(Identity::new("sys"), Identity::new("sys"));
    parser.imported_aliases = aliases;
    parser.used_symbols = HashSet::new();
    let mut exported = HashSet::new();
    exported.insert(Identity::new("os"));
    parser.exported_symbols = exported;

    let checker = UnusedImportRuleChecker::new(Arc::new(parser));
    let path = FilePath::new("test.py").unwrap_or_default();
    let unused = checker.find_unused_imports(&path);
    assert_eq!(unused.len(), 1, "sys should be flagged but os (exported) should not");
}

#[test]
fn check_unused_imports_empty_content_no_violations() {
    let parser = MockUnusedParser::new();
    let checker = UnusedImportRuleChecker::new(Arc::new(parser));
    let mut violations = vec![];
    checker.check_unused_imports("test.rs", "", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn check_unused_imports_detects_unused_rust_import() {
    let mut parser = MockUnusedParser::new();
    parser.content = "use std::collections::HashMap;\nfn main() {}".to_string();
    parser.rust_js_imports = vec![(SymbolName::new("HashMap"), LineNumber::new(1))];
    parser.name_used = false;

    let checker = UnusedImportRuleChecker::new(Arc::new(parser));
    let mut violations = vec![];
    checker.check_unused_imports("test.rs", &parser.content, &mut violations);
    assert_eq!(violations.len(), 1, "unused Rust import should be flagged");
    assert!(violations[0].code.value().contains("AES203"));
}

#[test]
fn check_unused_imports_used_rust_import_no_violation() {
    let mut parser = MockUnusedParser::new();
    parser.content = "use std::collections::HashMap;\nfn main() { let mut m = HashMap::new(); }".to_string();
    parser.rust_js_imports = vec![(SymbolName::new("HashMap"), LineNumber::new(1))];
    parser.name_used = true;

    let checker = UnusedImportRuleChecker::new(Arc::new(parser));
    let mut violations = vec![];
    checker.check_unused_imports("test.rs", &parser.content, &mut violations);
    assert!(violations.is_empty(), "used import should not flag");
}
