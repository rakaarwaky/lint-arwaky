// Acceptance tests — AES502: Contract orphan detection.
use orphan_rules_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::IContractOrphanProtocol;
use shared::quality_rules::taxonomy_analysis_vo::{InheritanceMap, ReachabilityResult};
use std::collections::{HashMap, HashSet};

fn empty_reachability() -> ReachabilityResult {
    ReachabilityResult::new(HashSet::new())
}

fn reachable_for(fp: &FilePath) -> ReachabilityResult {
    ReachabilityResult::new(HashSet::from([fp.clone()]))
}

fn contract_analyzer() -> ContractOrphanAnalyzer {
    ContractOrphanAnalyzer::new()
}

#[test]
fn aes502_protocol_trait_not_implemented_is_orphan() {
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
    );
    // No file implements IFooProtocol
    content_map.insert(
        "crates/shared/src/other_file.rs".to_string(),
        "fn unrelated() {}".to_string(),
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
    assert_eq!(result.severity, Severity::MEDIUM);
    assert!(result.reason.contains("IFooProtocol"));
}

#[test]
fn aes502_protocol_trait_implemented_is_not_orphan() {
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
    );
    // Capabilities file implements the trait
    content_map.insert(
        "crates/orphan-rules/src/capabilities_foo.rs".to_string(),
        "impl IFooProtocol for Foo {\n    fn do_thing(&self) {}\n}".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/orphan-rules/src/capabilities_foo.rs".to_string(),
    ];

    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &InheritanceMap::new(HashMap::new()),
        &all_files,
        &content_map,
        &reachable_for(&fp),
    );
    assert!(
        !result.is_orphan,
        "Protocol implemented by capabilities should NOT be orphan"
    );
}

#[test]
fn aes502_aggregate_not_called_by_surface_is_orphan() {
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_foo_aggregate.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooAggregate: Send + Sync {\n    fn run(&self);\n}".to_string(),
    );
    // No surface file references IFooAggregate
    content_map.insert(
        "crates/shared/src/other.rs".to_string(),
        "fn something() {}".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/shared/src/other.rs".to_string(),
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
    assert!(result.reason.contains("IFooAggregate"));
}

#[test]
fn aes502_empty_content_is_not_orphan() {
    let analyzer = contract_analyzer();
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
fn aes502_no_traits_in_content_is_not_orphan() {
    let analyzer = contract_analyzer();
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
fn aes502_trait_reexported_in_barrel_is_not_orphan() {
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
    );
    // mod.rs re-exports the trait name
    content_map.insert(
        "crates/shared/src/mod.rs".to_string(),
        "pub use contract_foo_protocol::IFooProtocol;\n".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/shared/src/mod.rs".to_string(),
    ];

    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &InheritanceMap::new(HashMap::new()),
        &all_files,
        &content_map,
        &reachable_for(&fp),
    );
    assert!(
        !result.is_orphan,
        "Trait re-exported in barrel should NOT be orphan"
    );
}

#[test]
fn aes502_multiple_traits_one_not_implemented() {
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}\npub trait IBarProtocol: Send + Sync {\n    fn other(&self);\n}".to_string(),
    );
    // Only IFooProtocol is implemented
    content_map.insert(
        "crates/orphan-rules/src/capabilities_foo.rs".to_string(),
        "impl IFooProtocol for Foo {\n    fn do_thing(&self) {}\n}".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/orphan-rules/src/capabilities_foo.rs".to_string(),
    ];

    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &InheritanceMap::new(HashMap::new()),
        &all_files,
        &content_map,
        &empty_reachability(),
    );
    assert!(
        result.is_orphan,
        "File with one unimplemented trait should be flagged"
    );
    assert!(
        result.reason.contains("IBarProtocol"),
        "Reason should mention the unimplemented trait"
    );
}

#[test]
fn aes502_non_protocol_suffix_not_checked_for_orchestration() {
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_foo_entity.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooEntity: Send + Sync {\n    fn id(&self);\n}".to_string(),
    );
    // Even with no implementation, non-protocol/non-aggregate suffixes may be treated differently
    content_map.insert(
        "crates/shared/src/other.rs".to_string(),
        "fn noop() {}".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/shared/src/other.rs".to_string(),
    ];

    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &InheritanceMap::new(HashMap::new()),
        &all_files,
        &content_map,
        &empty_reachability(),
    );
    // "entity" suffix: the analyzer checks trait implementation first
    assert!(
        result.is_orphan,
        "Unimplemented trait in entity file should be orphan"
    );
}
