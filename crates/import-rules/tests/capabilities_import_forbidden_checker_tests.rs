use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use import_rules_lint_arwaky::capabilities_import_forbidden_checker::ImportForbiddenChecker;
use shared::common::taxonomy_common_vo::{LineNumber, PatternList};
use shared::common::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::{ArchitectureConfig, ArchitectureRule};
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::taxonomy_definition_vo::LayerDefinition;

// ---------------------------------------------------------------------------
// Mock parser
// ---------------------------------------------------------------------------
struct MockForbiddenParser {
    import_lines: Vec<(i64, String)>,
    basename: String,
    scope_match: bool,
}

impl MockForbiddenParser {
    fn new() -> Self {
        Self {
            import_lines: vec![],
            basename: "test.rs".to_string(),
            scope_match: false,
        }
    }
}

impl IImportParserPort for MockForbiddenParser {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
        (LayerNameVO::new(scope.value()), vec![])
    }
    fn import_matches_scope(&self, _: &LineContentVO, _: &LayerNameVO, _: &[Identity]) -> bool {
        self.scope_match
    }
    fn get_basename(&self, _: &FilePath) -> Identity {
        Identity::new(self.basename.clone())
    }
    fn read_import_lines(&self, _: &FilePath) -> Vec<(LineNumber, LineContentVO)> {
        self.import_lines
            .iter()
            .map(|(n, l)| (LineNumber::new(*n), LineContentVO::new(l.clone())))
            .collect()
    }
    fn parse_import_lines(&self, _: &FileContentVO) -> Vec<(LineNumber, LineContentVO)> {
        vec![]
    }
    fn extract_module_from_line(&self, line: &LineContentVO) -> Option<Identity> {
        let s = line.value();
        if let Some(rest) = s.strip_prefix("use ") {
            Some(Identity::new(rest.trim_end_matches(';').to_string()))
        } else {
            None
        }
    }
    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO> {
        let s = segment.value();
        // Prefix-based matching (like the real parser)
        const PREFIX_MAP: &[(&str, &str)] = &[
            ("taxonomy_", "taxonomy"),
            ("contract_", "contract"),
            ("capabilities_", "capabilities"),
            ("infrastructure_", "infrastructure"),
            ("agent_", "agent"),
            ("surface_", "surfaces"),
            ("root_", "root"),
        ];
        for &(prefix, layer) in PREFIX_MAP {
            if s.starts_with(prefix) {
                return Some(LayerNameVO::new(layer));
            }
        }
        // Direct match
        match s {
            "taxonomy" => Some(LayerNameVO::new("taxonomy")),
            "contract" => Some(LayerNameVO::new("contract")),
            "capabilities" => Some(LayerNameVO::new("capabilities")),
            "infrastructure" => Some(LayerNameVO::new("infrastructure")),
            "agent" => Some(LayerNameVO::new("agent")),
            "surfaces" | "surface" => Some(LayerNameVO::new("surfaces")),
            "root" => Some(LayerNameVO::new("root")),
            _ => None,
        }
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
    fn extract_exported_symbols(&self, _: &str) -> std::collections::HashSet<Identity> {
        std::collections::HashSet::new()
    }
    fn extract_used_symbols(
        &self,
        _: &str,
        _: &HashMap<Identity, Identity>,
    ) -> std::collections::HashSet<Identity> {
        std::collections::HashSet::new()
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
    fn extract_layer_from_prefix(
        &self,
        _: &str,
    ) -> Option<shared::common::taxonomy_layer_vo::LayerNameVO> {
        None
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_def(forbidden: Vec<&str>, allowed: Vec<&str>, exceptions: Vec<&str>) -> LayerDefinition {
    LayerDefinition {
        forbidden: PatternList::new(forbidden.into_iter().map(String::from).collect::<Vec<_>>()),
        allowed: PatternList::new(allowed.into_iter().map(String::from).collect::<Vec<_>>()),
        mandatory: PatternList::default(),
        exceptions: PatternList::new(exceptions.into_iter().map(String::from).collect::<Vec<_>>()),
        ..LayerDefinition::default()
    }
}

fn fp(s: &str) -> FilePath {
    FilePath::new(s.to_string()).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn forbidden_exception_file_skips_check() {
    let mut parser = MockForbiddenParser::new();
    parser.basename = "skip.rs".to_string();
    let checker = ImportForbiddenChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let mut processed = HashSet::new();
    let def = make_def(vec!["capabilities"], vec![], vec!["skip.rs"]);
    checker.check_forbidden_imports_layer(
        &fp("src/skip.rs"),
        "surfaces",
        &def,
        &[],
        &mut violations,
        &mut processed,
    );
    assert!(violations.is_empty(), "exception file should be skipped");
}

#[test]
fn forbidden_no_forbidden_list_and_not_surfaces_skips() {
    let parser = MockForbiddenParser::new();
    let checker = ImportForbiddenChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let mut processed = HashSet::new();
    let def = make_def(vec![], vec![], vec![]);
    checker.check_forbidden_imports_layer(
        &fp("src/taxonomy_config.rs"),
        "taxonomy",
        &def,
        &[],
        &mut violations,
        &mut processed,
    );
    assert!(
        violations.is_empty(),
        "non-surfaces with empty forbidden should skip"
    );
}

#[test]
fn forbidden_surfaces_default_forbidden_list() {
    let mut parser = MockForbiddenParser::new();
    parser.import_lines = vec![(10, "use agent_orchestrator;".to_string())];
    let checker = ImportForbiddenChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let mut processed = HashSet::new();
    let def = make_def(vec![], vec!["shared"], vec![]);
    let default_forbidden = vec![
        "agent".to_string(),
        "infrastructure".to_string(),
        "capabilities".to_string(),
    ];
    checker.check_forbidden_imports_layer(
        &fp("src/surface_command.rs"),
        "surfaces",
        &def,
        &default_forbidden,
        &mut violations,
        &mut processed,
    );
    assert_eq!(
        violations.len(),
        1,
        "surfaces defaults should forbid agent imports"
    );
    assert!(violations[0].code.to_string().contains("AES201"));
}

#[test]
fn forbidden_detects_forbidden_import() {
    let mut parser = MockForbiddenParser::new();
    parser.import_lines = vec![(5, "use infrastructure::Scanner;".to_string())];
    let checker = ImportForbiddenChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let mut processed = HashSet::new();
    let def = make_def(vec!["infrastructure"], vec!["shared"], vec![]);
    checker.check_forbidden_imports_layer(
        &fp("src/surface_command.rs"),
        "surfaces",
        &def,
        &[],
        &mut violations,
        &mut processed,
    );
    assert_eq!(violations.len(), 1);
    assert!(violations[0].message.value().contains("infrastructure"));
}

#[test]
fn forbidden_allows_non_forbidden_import() {
    let mut parser = MockForbiddenParser::new();
    parser.import_lines = vec![(5, "use shared::common::Path;".to_string())];
    let checker = ImportForbiddenChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let mut processed = HashSet::new();
    let def = make_def(vec!["infrastructure"], vec!["shared"], vec![]);
    checker.check_forbidden_imports_layer(
        &fp("src/surface_command.rs"),
        "surfaces",
        &def,
        &[],
        &mut violations,
        &mut processed,
    );
    assert!(violations.is_empty(), "shared imports should be allowed");
}

#[test]
fn forbidden_multiple_forbidden_imports() {
    let mut parser = MockForbiddenParser::new();
    parser.import_lines = vec![
        (3, "use agent::Orchestrator;".to_string()),
        (7, "use infrastructure::Scanner;".to_string()),
    ];
    let checker = ImportForbiddenChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let mut processed = HashSet::new();
    let def = make_def(vec!["agent", "infrastructure"], vec!["shared"], vec![]);
    checker.check_forbidden_imports_layer(
        &fp("src/surface_command.rs"),
        "surfaces",
        &def,
        &[],
        &mut violations,
        &mut processed,
    );
    assert_eq!(
        violations.len(),
        2,
        "both forbidden imports should be flagged"
    );
}

#[test]
fn scope_forbidden_skips_entry_files() {
    let parser = MockForbiddenParser::new();
    let checker = ImportForbiddenChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let mut processed = HashSet::new();
    let config = ArchitectureConfig::default();
    checker.check_scope_forbidden_imports(
        &fp("src/mod.rs"),
        &config,
        &mut violations,
        &mut processed,
    );
    assert!(violations.is_empty());
    checker.check_scope_forbidden_imports(
        &fp("src/lib.rs"),
        &config,
        &mut violations,
        &mut processed,
    );
    assert!(violations.is_empty());
    checker.check_scope_forbidden_imports(
        &fp("src/main.rs"),
        &config,
        &mut violations,
        &mut processed,
    );
    assert!(violations.is_empty());
}

#[test]
fn scope_forbidden_skip_exception() {
    let mut parser = MockForbiddenParser::new();
    parser.basename = "skip_me.rs".to_string();
    parser.import_lines = vec![(1, "use infrastructure::x;".to_string())];
    let checker = ImportForbiddenChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let mut processed = HashSet::new();
    let config = ArchitectureConfig {
        rules: vec![ArchitectureRule {
            name: "test-rule".to_string().into(),
            scope: "agent(orchestrator)".to_string().into(),
            forbidden: PatternList::new(vec!["infrastructure".to_string()]),
            allowed: PatternList::default(),
            exceptions: PatternList::new(vec!["skip_me.rs".to_string()]),
            ..ArchitectureRule::default()
        }],
        ..ArchitectureConfig::default()
    };
    checker.check_scope_forbidden_imports(
        &fp("src/agent_orchestrator.rs"),
        &config,
        &mut violations,
        &mut processed,
    );
    assert!(violations.is_empty(), "exception should skip scope check");
}

#[test]
fn rule_name_is_aes201() {
    let parser = MockForbiddenParser::new();
    let checker = ImportForbiddenChecker::new(Arc::new(parser));
    assert_eq!(checker.rule_name().value(), "AES201");
}
