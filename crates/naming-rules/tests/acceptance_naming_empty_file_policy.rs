// Acceptance tests — the empty-file policy of the naming audit.
//
// A file with no content still has a name, and the name is what these rules
// judge. The audit used to drop such a file before the rules ran, so a
// zero-byte file with a broken name produced a clean report — a silent pass
// on the state that most needs a finding.
use naming_rules_lint_arwaky::agent_naming_orchestrator::{
    NamingOrchestrator, NamingOrchestratorDeps,
};
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_policy_checker::SuffixPolicyChecker;
use naming_rules_lint_arwaky::capabilities_test_file_prefix_checker::TestFilePrefixChecker;
use shared_common::PatternList;
use shared_common::SuffixPolicyVO;
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_layer_vo::{LayerDefinition, LayerMapVO};
use shared_common::taxonomy_violation_item_vo::ViolationItem;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_naming_rules::INamingRunnerAggregate;
use shared_naming_rules::SUFFIX_POLICY_STRICT;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_naming_rules::taxonomy_naming_rules_response::NamingResponse;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// The layer map the audit runs against.
fn layer_map() -> LayerMapVO {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    def.naming.allowed_suffix =
        PatternList::new(vec!["checker".to_string(), "adapter".to_string()]);
    def.naming.forbidden_suffix = PatternList::new(vec!["vo".to_string()]);

    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    LayerMapVO::new(layers)
}

/// The orchestrator the audit runs through.
fn orchestrator() -> NamingOrchestrator {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(layer_map());
    NamingOrchestrator::new(NamingOrchestratorDeps {
        naming_convention: Arc::new(NamingConventionChecker::new()),
        suffix_policy: Arc::new(SuffixPolicyChecker::new()),
        test_file_prefix: Arc::new(TestFilePrefixChecker::new()),
        config,
        layer_map,
    })
}

/// One file entry with no content at all.
fn empty_entry(path: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: 0,
        content: String::new(),
        parse_ok: true,
        parse_metadata: None,
    }
}

/// Run the audit over one source set and one test set.
fn audit(source: &[FileEntry], tests: &[FileEntry]) -> Vec<ViolationItem> {
    let orchestrator = orchestrator();
    match orchestrator.execute(NamingRequest::audit_with_tests(source, tests)) {
        NamingResponse::Audit { violations, .. } => violations,
        NamingResponse::Name { .. } => panic!("expected an audit response"),
    }
}

/// How many findings carry *code*.
fn count_of(results: &[ViolationItem], code: &str) -> usize {
    results.iter().filter(|r| r.code.code() == code).count()
}

#[test]
fn a_zero_byte_file_with_a_broken_stem_still_fails_aes101() {
    let entries = vec![empty_entry("src/capabilities_BadName.rs")];
    let results = audit(&entries, &[]);
    assert_eq!(
        count_of(&results, "AES101"),
        1,
        "a file with no content still has a name, and the name must be judged; got {results:#?}"
    );
}

#[test]
fn a_zero_byte_file_with_a_forbidden_suffix_still_fails_aes102() {
    let entries = vec![empty_entry("src/capabilities_user_vo.rs")];
    let results = audit(&entries, &[]);
    assert_eq!(
        count_of(&results, "AES102"),
        1,
        "a file with no content still carries a suffix, and the suffix must be judged; got \
         {results:#?}"
    );
}

#[test]
fn a_zero_byte_test_file_without_a_legal_prefix_still_fails_aes103() {
    let tests = vec![empty_entry("crates/calc/tests/helpers.rs")];
    let results = audit(&[], &tests);
    assert_eq!(
        count_of(&results, "AES103"),
        1,
        "a test file with no content still carries a name, and the name must be judged; got \
         {results:#?}"
    );
}

#[test]
fn a_zero_byte_file_with_a_legal_name_stays_clean() {
    let source = vec![empty_entry("src/capabilities_user_checker.rs")];
    let tests = vec![empty_entry("crates/calc/tests/unit_adder.rs")];
    let results = audit(&source, &tests);
    assert!(
        results.is_empty(),
        "the empty-file policy judges names, it does not invent findings; got {results:#?}"
    );
}

#[test]
fn the_audit_counts_the_empty_files_it_inspected() {
    let source = vec![empty_entry("src/capabilities_BadName.rs")];
    let tests = vec![empty_entry("crates/calc/tests/helpers.rs")];
    let orchestrator = orchestrator();
    let response = orchestrator.execute(NamingRequest::audit_with_tests(&source, &tests));
    let NamingResponse::Audit { stats, .. } = response else {
        panic!("expected an audit response");
    };
    assert_eq!(stats.aes101_inspected, 1);
    assert_eq!(stats.aes102_inspected, 1);
    assert_eq!(stats.aes103_inspected, 1);
}
