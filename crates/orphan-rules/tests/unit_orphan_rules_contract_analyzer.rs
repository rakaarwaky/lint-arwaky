// Unit tests for ContractOrphanAnalyzer — orphan detection for contract-layer files.
use orphan_rules_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use shared::orphan_rules::{IContractOrphanProtocol, IOrphanParserProtocol};
use shared::quality_rules::taxonomy_analysis_vo::{InheritanceMap, ReachabilityResult};
use std::collections::HashSet;

fn empty_reachability() -> ReachabilityResult {
    ReachabilityResult::new(HashSet::new())
}

use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::*;
use std::collections::HashMap;
use std::sync::Arc;

// ── Mock Parser ─────────────────────────────────────────

struct MockParser;

impl IOrphanParserProtocol for MockParser {
    fn parse_file(&self, path: &str, content: &str) -> FileParseResultVO {
        if path.ends_with(".rs") {
            let mut traits = Vec::new();
            let mut trait_impls = Vec::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("pub trait ") || trimmed.starts_with("trait ") {
                    let raw = trimmed
                        .trim_start_matches("pub ")
                        .trim_start_matches("trait ");
                    // Strip bounds: "IFooProtocol: Send + Sync {" → "IFooProtocol"
                    let name = if let Some(pos) = raw.find(':') {
                        raw[..pos].trim().to_string()
                    } else {
                        raw.trim_end_matches('{').trim().to_string()
                    };
                    traits.push(AstTraitDefVO {
                        name,
                        is_pub: trimmed.starts_with("pub trait"),
                    });
                }
                if trimmed.starts_with("impl ") && trimmed.contains("for ") {
                    let rest = trimmed.trim_start_matches("impl ");
                    if let Some(pos) = rest.find(" for ") {
                        let trait_name = rest[..pos].trim().to_string();
                        trait_impls.push(AstTraitImplVO {
                            trait_name,
                            type_name: rest[pos + 5..].trim_end_matches('{').trim().to_string(),
                            has_generics: false,
                            line: 0,
                            is_dummy: false,
                        });
                    }
                }
            }
            FileParseResultVO::Rust(RustParseResultVO {
                traits,
                trait_impls,
                ..Default::default()
            })
        } else {
            FileParseResultVO::Unsupported
        }
    }

    fn is_supported(&self, path: &str) -> bool {
        path.ends_with(".rs")
    }
}

fn reachable_for(fp: &FilePath) -> ReachabilityResult {
    ReachabilityResult::new(HashSet::from([fp.clone()]))
}

// ── Tests ───────────────────────────────────────────────

#[test]
fn test_constructor() {
    let _parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
    let _analyzer = ContractOrphanAnalyzer::new();
}

#[test]
fn test_empty_content_is_not_orphan() {
    let _parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
    let analyzer = ContractOrphanAnalyzer::new();

    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let content_map: HashMap<String, String> = HashMap::new();

    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &InheritanceMap::new(HashMap::new()),
        &[],
        &content_map,
        &empty_reachability(),
    );
    assert!(!result.is_orphan);
}

#[test]
fn test_no_traits_is_not_orphan() {
    let _parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
    let analyzer = ContractOrphanAnalyzer::new();

    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "use something::Foo;\nfn do_thing() {}".to_string(),
    );

    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &InheritanceMap::new(HashMap::new()),
        &[],
        &content_map,
        &empty_reachability(),
    );
    assert!(!result.is_orphan);
}

#[test]
fn test_trait_not_implemented_is_orphan() {
    let _parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
    let analyzer = ContractOrphanAnalyzer::new();

    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
    );
    content_map.insert(
        "crates/shared/src/other_file.rs".to_string(),
        "fn something_else() {}".to_string(),
    );

    let all_files = vec![
        fp.value().to_string(),
        "crates/shared/src/other_file.rs".to_string(),
    ];

    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &InheritanceMap::new(HashMap::new()),
        &all_files,
        &content_map,
        &empty_reachability(),
    );
    assert!(result.is_orphan);
    assert!(result.severity == Severity::MEDIUM);
    assert!(result.reason.contains("IFooProtocol"));
}

#[test]
fn test_trait_implemented_is_not_orphan() {
    let _parser: Arc<dyn IOrphanParserProtocol> = Arc::new(MockParser);
    let analyzer = ContractOrphanAnalyzer::new();

    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
    );
    content_map.insert(
        "crates/shared/src/capabilities_foo.rs".to_string(),
        "impl IFooProtocol for Foo {\n    fn do_thing(&self) {}\n}".to_string(),
    );

    let all_files = vec![
        fp.value().to_string(),
        "crates/shared/src/capabilities_foo.rs".to_string(),
    ];

    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &InheritanceMap::new(HashMap::new()),
        &all_files,
        &content_map,
        &reachable_for(&fp),
    );
    assert!(!result.is_orphan);
}
