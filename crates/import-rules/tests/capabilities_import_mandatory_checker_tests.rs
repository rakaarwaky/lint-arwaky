use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use import_rules_lint_arwaky::capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
use shared::common::taxonomy_common_vo::{LineNumber, PatternList};
use shared::common::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::{ArchitectureConfig, ArchitectureRule};
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_layer_prefix_port;
use shared::import_rules::contract_rule_protocol::IArchRuleProtocol;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::taxonomy_definition_vo::LayerDefinition;

// ---------------------------------------------------------------------------
// Mock parser
// ---------------------------------------------------------------------------
struct MockMandatoryParser {
    file_content: String,
    import_lines: Vec<(i64, String)>,
    basename: String,
    scope_match: bool,
}

impl MockMandatoryParser {
    fn new() -> Self {
        Self {
            file_content: String::new(),
            import_lines: vec![],
            basename: "test.rs".to_string(),
            scope_match: false,
        }
    }
}

impl IImportParserPort for MockMandatoryParser {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
        let s = scope.value();
        if let Some(paren) = s.find('(') {
            let layer = &s[..paren];
            let inner = s[paren + 1..].trim_end_matches(')').trim();
            let suffixes: Vec<Identity> = inner
                .split(|c: char| c == ',' || c == '|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(Identity::new)
                .collect();
            (LayerNameVO::new(layer), suffixes)
        } else {
            (LayerNameVO::new(s.trim()), vec![])
        }
    }

    fn import_matches_scope(&self, _: &LineContentVO, _: &LayerNameVO, _: &[Identity]) -> bool {
        self.scope_match
    }

    fn get_basename(&self, _: &FilePath) -> Identity {
        Identity::new(&self.basename)
    }

    fn read_import_lines(&self, _: &FilePath) -> Vec<(LineNumber, LineContentVO)> {
        self.import_lines
            .iter()
            .map(|(ln, line)| (LineNumber::new(*ln), LineContentVO::new(line.clone())))
            .collect()
    }

    fn parse_import_lines(&self, _: &FileContentVO) -> Vec<(LineNumber, LineContentVO)> {
        self.import_lines
            .iter()
            .map(|(ln, line)| (LineNumber::new(*ln), LineContentVO::new(line.clone())))
            .collect()
    }

    fn extract_module_from_line(&self, line: &LineContentVO) -> Option<Identity> {
        let l = line.value();
        if let Some(rest) = l.trim().strip_prefix("use ") {
            Some(Identity::new(rest.trim_end_matches(';').trim().to_string()))
        } else {
            None
        }
    }

    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO> {
        match segment.value() {
            "taxonomy" => Some(LayerNameVO::new("taxonomy")),
            "contract" => Some(LayerNameVO::new("contract")),
            "capabilities" => Some(LayerNameVO::new("capabilities")),
            "infrastructure" => Some(LayerNameVO::new("infrastructure")),
            "agent" => Some(LayerNameVO::new("agent")),
            "surfaces" | "surface" => Some(LayerNameVO::new("surfaces")),
            "root" => Some(LayerNameVO::new("root")),
            s => {
                if let Some(layer) = contract_layer_prefix_port::extract_layer_from_prefix(s) {
                    Some(LayerNameVO::new(layer))
                } else {
                    None
                }
            }
        }
    }

    fn read_file_to_message(&self, _: &FilePath) -> Result<LintMessage, std::io::Error> {
        Ok(LintMessage::new(self.file_content.clone()))
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

fn make_def(mandatory: Vec<&str>, exceptions: Vec<&str>) -> LayerDefinition {
    LayerDefinition {
        mandatory: PatternList::new(mandatory.to_vec()),
        exceptions: PatternList::new(exceptions.to_vec()),
        ..LayerDefinition::default()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn mandatory_empty_definition_skips() {
    let parser = MockMandatoryParser::new();
    let checker = ArchImportMandatoryChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let def = make_def(vec![], vec![]);
    checker.check_mandatory_imports("src/capabilities_processor.rs", &def, &mut violations);
    assert!(violations.is_empty(), "no mandatory list should skip");
}

#[test]
fn mandatory_init_py_skipped() {
    let mut parser = MockMandatoryParser::new();
    parser.basename = "__init__.py".to_string();
    let checker = ArchImportMandatoryChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let def = make_def(vec!["shared"], vec![]);
    checker.check_mandatory_imports("src/__init__.py", &def, &mut violations);
    assert!(violations.is_empty(), "__init__.py should be skipped");
}

#[test]
fn mandatory_exception_file_skipped() {
    let mut parser = MockMandatoryParser::new();
    parser.basename = "skip_me.rs".to_string();
    let checker = ArchImportMandatoryChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let def = make_def(vec!["shared"], vec!["skip_me.rs"]);
    checker.check_mandatory_imports("src/skip_me.rs", &def, &mut violations);
    assert!(violations.is_empty(), "exception files should be skipped");
}

#[test]
fn mandatory_missing_import_detected() {
    let mut parser = MockMandatoryParser::new();
    parser.basename = "capabilities_processor.rs".to_string();
    parser.import_lines = vec![(5, "use std::collections::HashMap;".to_string())];
    parser.file_content = "use std::collections::HashMap;".to_string();
    let checker = ArchImportMandatoryChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let def = make_def(vec!["shared"], vec![]);
    checker.check_mandatory_imports("src/capabilities_processor.rs", &def, &mut violations);
    assert_eq!(
        violations.len(),
        1,
        "missing mandatory import should be flagged"
    );
    assert!(violations[0].code.to_string().contains("AES202"));
}

#[test]
fn mandatory_import_present_no_violation() {
    let mut parser = MockMandatoryParser::new();
    parser.basename = "capabilities_processor.rs".to_string();
    parser.import_lines = vec![(5, "use shared::common::HashMap;".to_string())];
    parser.file_content = "use shared::common::HashMap;".to_string();
    let checker = ArchImportMandatoryChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let def = make_def(vec!["shared"], vec![]);
    checker.check_mandatory_imports("src/capabilities_processor.rs", &def, &mut violations);
    assert!(
        violations.is_empty(),
        "present mandatory import should not flag"
    );
}

#[test]
fn scope_mandatory_skips_entry_files() {
    let parser = MockMandatoryParser::new();
    let checker = ArchImportMandatoryChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let config = ArchitectureConfig::default();
    checker.check_scope_mandatory_imports("src/mod.rs", &config, &mut violations);
    assert!(violations.is_empty());
    checker.check_scope_mandatory_imports("src/lib.rs", &config, &mut violations);
    assert!(violations.is_empty());
    checker.check_scope_mandatory_imports("src/main.rs", &config, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn scope_mandatory_skips_rules_with_empty_mandatory() {
    let parser = MockMandatoryParser::new();
    let checker = ArchImportMandatoryChecker::new(Arc::new(parser));
    let mut violations = vec![];
    let config = ArchitectureConfig {
        rules: vec![ArchitectureRule {
            name: "no-mandatory".to_string().into(),
            scope: "capabilities(processor)".to_string().into(),
            mandatory: PatternList::default(),
            ..ArchitectureRule::default()
        }],
        ..ArchitectureConfig::default()
    };
    checker.check_scope_mandatory_imports(
        "src/capabilities_processor.rs",
        &config,
        &mut violations,
    );
    assert!(
        violations.is_empty(),
        "rule with empty mandatory should be skipped"
    );
}

#[test]
fn rule_name_is_aes202() {
    let parser = MockMandatoryParser::new();
    let checker = ArchImportMandatoryChecker::new(Arc::new(parser));
    assert_eq!(checker.rule_name().to_string(), "AES202");
}
