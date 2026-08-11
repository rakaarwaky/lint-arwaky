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

    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &InheritanceMap::new(HashMap::new()),
        &[],
        &content_map,
        &empty_reachability(),
    );
    // "entity" suffix: the analyzer checks trait implementation first
    assert!(
        result.is_orphan,
        "Unimplemented trait in entity file should be orphan"
    );
}

#[test]
fn aes502_contract_with_alive_implementor_is_not_orphan() {
    // P3 (issues #191/192): symmetric contract wiring — a contract consumed only
    // via DI (its implementor is reachable, the contract itself is never statically
    // imported by the entry chain) must not be flagged AES502.
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
    );
    let capability_fp =
        FilePath::new("crates/orphan-rules/src/capabilities_foo.rs".to_string()).unwrap();
    // The capability implementing the contract IS in the alive set, but the
    // contract file itself is not — pure DI consumption.
    let alive = reachable_for(&capability_fp);
    let inheritance = InheritanceMap::new(HashMap::from([(
        "IFooProtocol".to_string(),
        vec![capability_fp.value().to_string()],
    )]));
    // The capability file implements the protocol (needed by condition 2).
    content_map.insert(
        capability_fp.value().to_string(),
        "struct Foo;\nimpl IFooProtocol for Foo {\n    fn do_thing(&self) {}\n}\n".to_string(),
    );
    let all_files = vec![fp.value().to_string(), capability_fp.value().to_string()];

    let result =
        analyzer.is_contract_orphan(&fp, &root, &inheritance, &all_files, &content_map, &alive);
    assert!(
        !result.is_orphan,
        "Contract with an alive implementor should NOT be orphan: {}",
        result.reason
    );
}

#[test]
fn aes502_contract_implementor_not_alive_is_still_orphan() {
    // P3 negative case: an implementor is registered in the inheritance map,
    // but it is NOT in the alive set — the contract must still be flagged.
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
    );
    let capability_fp =
        FilePath::new("crates/orphan-rules/src/capabilities_foo.rs".to_string()).unwrap();
    let inheritance = InheritanceMap::new(HashMap::from([(
        "IFooProtocol".to_string(),
        vec![capability_fp.value().to_string()],
    )]));
    let all_files = vec![fp.value().to_string(), capability_fp.value().to_string()];

    // Neither the contract nor the implementor is reachable.
    let result = analyzer.is_contract_orphan(
        &fp,
        &root,
        &inheritance,
        &all_files,
        &content_map,
        &empty_reachability(),
    );
    assert!(
        result.is_orphan,
        "Contract whose implementor is also unreachable must remain orphan"
    );
}

#[test]
fn aes502_contract_alive_implementor_matched_by_absolute_path_suffix() {
    // P3: the alive set may store absolute paths while the inheritance map
    // stores workspace-relative paths — `is_path_alive` matches on suffix.
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
    );
    let impl_rel = "crates/orphan-rules/src/capabilities_foo.rs".to_string();
    content_map.insert(
        impl_rel.clone(),
        "struct Foo;\nimpl IFooProtocol for Foo {\n    fn do_thing(&self) {}\n}\n".to_string(),
    );
    let inheritance =
        InheritanceMap::new(HashMap::from([("IFooProtocol".to_string(), vec![impl_rel.clone()])]));
    let all_files = vec![fp.value().to_string(), impl_rel.clone()];

    // Alive set stores an absolute path ending with the relative impl path.
    let absolute_alive = FilePath::new(format!("/abs/workspace/{}", impl_rel)).unwrap();
    let alive = reachable_for(&absolute_alive);

    let result =
        analyzer.is_contract_orphan(&fp, &root, &inheritance, &all_files, &content_map, &alive);
    assert!(
        !result.is_orphan,
        "Absolute alive path ending with the relative impl path should match: {}",
        result.reason
    );
}

#[test]
fn aes502_contract_alive_implementor_matched_by_basename_only() {
    // P3: as a last resort, `is_path_alive` matches purely on file basename
    // when neither path is a suffix of the other (e.g. differing directory
    // layouts between the alive set and the inheritance map).
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_foo_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}".to_string(),
    );
    let impl_rel = "totally/different/path/capabilities_foo.rs".to_string();
    content_map.insert(
        impl_rel.clone(),
        "struct Foo;\nimpl IFooProtocol for Foo {\n    fn do_thing(&self) {}\n}\n".to_string(),
    );
    let inheritance =
        InheritanceMap::new(HashMap::from([("IFooProtocol".to_string(), vec![impl_rel.clone()])]));
    let all_files = vec![fp.value().to_string(), impl_rel.clone()];

    // Alive path shares only the basename, not any directory prefix/suffix.
    let mismatched_alive =
        FilePath::new("/some/other/prefix/capabilities_foo.rs".to_string()).unwrap();
    let alive = reachable_for(&mismatched_alive);

    let result =
        analyzer.is_contract_orphan(&fp, &root, &inheritance, &all_files, &content_map, &alive);
    assert!(
        !result.is_orphan,
        "Basename-only match should still count as alive: {}",
        result.reason
    );
}

#[test]
fn aes502_contract_multiple_traits_only_one_has_alive_implementor() {
    // P3: `has_alive_implementor` uses `.any()` across all trait names in the
    // file — a single alive implementor for ANY trait is enough for condition 1.
    let analyzer = contract_analyzer();
    let fp = FilePath::new("crates/shared/src/contract_multi_protocol.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub trait IFooProtocol: Send + Sync {\n    fn do_thing(&self);\n}\npub trait IBarProtocol: Send + Sync {\n    fn other(&self);\n}".to_string(),
    );
    let foo_impl = "crates/orphan-rules/src/capabilities_foo.rs".to_string();
    let bar_impl = "crates/orphan-rules/src/capabilities_bar.rs".to_string();
    content_map.insert(
        foo_impl.clone(),
        "impl IFooProtocol for Foo {\n    fn do_thing(&self) {}\n}".to_string(),
    );
    content_map.insert(
        bar_impl.clone(),
        "impl IBarProtocol for Bar {\n    fn other(&self) {}\n}".to_string(),
    );
    let inheritance = InheritanceMap::new(HashMap::from([
        ("IFooProtocol".to_string(), vec![foo_impl.clone()]),
        ("IBarProtocol".to_string(), vec![bar_impl.clone()]),
    ]));
    let all_files = vec![fp.value().to_string(), foo_impl.clone(), bar_impl.clone()];

    // Only IBarProtocol's implementor is alive; IFooProtocol's is not.
    let alive = reachable_for(&FilePath::new(bar_impl).unwrap());

    let result =
        analyzer.is_contract_orphan(&fp, &root, &inheritance, &all_files, &content_map, &alive);
    assert!(
        !result.is_orphan,
        "One alive implementor among several traits is enough to satisfy reachability: {}",
        result.reason
    );
}
