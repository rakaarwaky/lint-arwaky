// E2E tests — full pipeline: temp dir → config → container → audit → violations.
use naming_rules_lint_arwaky::agent_naming_orchestrator::{
    NamingOrchestrator, NamingOrchestratorDeps,
};
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_policy_checker::SuffixPolicyChecker;
use naming_rules_lint_arwaky::capabilities_test_file_prefix_checker::TestFilePrefixChecker;
use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared_common::PatternList;
use shared_common::SuffixPolicyVO;
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_layer_vo::{LayerDefinition, LayerMapVO};
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_naming_rules::INamingRunnerAggregate;
use shared_naming_rules::SUFFIX_POLICY_STRICT;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_naming_rules::taxonomy_naming_rules_response::NamingResponse;
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::TempDir;

fn make_layer_map() -> LayerMapVO {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    def.naming.allowed_suffix =
        PatternList::new(vec!["checker".to_string(), "adapter".to_string()]);
    def.naming.forbidden_suffix = PatternList::new(vec!["vo".to_string()]);

    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    LayerMapVO::new(layers)
}

fn make_file_entries(dir: &std::path::Path, names: &[&str]) -> Vec<FileEntry> {
    names
        .iter()
        .map(|name| {
            let path = dir.join(name);
            let content = format!("fn dummy_{}() {{}}", name.replace('.', "_"));
            std::fs::write(&path, &content).unwrap();
            FileEntry {
                path,
                extension: "rs".to_string(),
                language: Language::Rust,
                size: content.len() as u64,
                content,
                parse_ok: true,
                parse_metadata: None,
            }
        })
        .collect()
}

/// Drive the aggregate and unwrap the audit payload, panicking on any other variant.
fn run_audit(
    orch: &dyn INamingRunnerAggregate,
    entries: &[FileEntry],
) -> Vec<shared_common::taxonomy_violation_item_vo::ViolationItem> {
    let request = NamingRequest::RunAuditWithEntries {
        files: entries.to_vec(),
    };
    match orch.execute(request) {
        NamingResponse::Audit { violations } => violations,
        NamingResponse::Name { .. } => panic!("expected an audit response"),
    }
}

/// Drive the aggregate over two file sets: production source and test/bench
/// files. AES101 and AES102 read the first, AES103 the second, so a violation
/// attributed to a rule proves which set reached it.
fn run_audit_with_tests(
    orch: &dyn INamingRunnerAggregate,
    source: &[FileEntry],
    tests: &[FileEntry],
) -> Vec<shared_common::taxonomy_violation_item_vo::ViolationItem> {
    match orch.execute(NamingRequest::audit_with_tests(source, tests)) {
        NamingResponse::Audit { violations } => violations,
        NamingResponse::Name { .. } => panic!("expected an audit response"),
    }
}

/// Write *relative names* under a temp root, returning the entries.
///
/// A name containing `/` creates the intermediate directories, so a fixture can
/// name a file inside `tests/` — the only place AES103 looks.
fn make_nested_entries(root: &std::path::Path, names: &[&str]) -> Vec<FileEntry> {
    names
        .iter()
        .map(|name| {
            let path = root.join(name);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            let content = format!("fn dummy_{}() {{}}", name.replace(['.', '/'], "_"));
            std::fs::write(&path, &content).unwrap();
            FileEntry {
                path,
                extension: "rs".to_string(),
                language: Language::Rust,
                size: content.len() as u64,
                content,
                parse_ok: true,
                parse_metadata: None,
            }
        })
        .collect()
}

#[test]
fn e2e_convention_violations_found() {
    let tmp = TempDir::new().unwrap();
    let entries = make_file_entries(
        tmp.path(),
        &[
            "capabilities_BadFile.rs",      // uppercase → AES101
            "capabilities_user_checker.rs", // clean → passes
        ],
    );

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let deps = NamingOrchestratorDeps {
        naming_convention: Arc::new(NamingConventionChecker::new()),
        suffix_policy: Arc::new(SuffixPolicyChecker::new()),
        test_file_prefix: Arc::new(TestFilePrefixChecker::new()),
        config: config.clone(),
        layer_map: layer_map.clone(),
    };
    let orch = NamingOrchestrator::new(deps);
    let results = run_audit(&orch, &entries);

    assert!(
        !results.is_empty(),
        "e2e should find at least one AES101 violation"
    );
    let has_aes101 = results.iter().any(|r| r.code.code().contains("AES101"));
    assert!(has_aes101, "should contain AES101 code");
}

#[test]
fn e2e_suffix_violations_found() {
    let tmp = TempDir::new().unwrap();
    let entries = make_file_entries(
        tmp.path(),
        &[
            "capabilities_user_vo.rs",      // forbidden suffix 'vo' → AES102
            "capabilities_user_handler.rs", // not in allowed list → AES102
        ],
    );

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let deps = NamingOrchestratorDeps {
        naming_convention: Arc::new(NamingConventionChecker::new()),
        suffix_policy: Arc::new(SuffixPolicyChecker::new()),
        test_file_prefix: Arc::new(TestFilePrefixChecker::new()),
        config: config.clone(),
        layer_map: layer_map.clone(),
    };
    let orch = NamingOrchestrator::new(deps);
    let results = run_audit(&orch, &entries);

    assert!(
        !results.is_empty(),
        "e2e should find at least one AES102 violation"
    );
    let has_aes102 = results.iter().any(|r| r.code.code().contains("AES102"));
    assert!(has_aes102, "should contain AES102 code");
}

#[test]
fn e2e_clean_files_no_violations() {
    let tmp = TempDir::new().unwrap();
    let entries = make_file_entries(
        tmp.path(),
        &["capabilities_user_checker.rs", "capabilities_db_adapter.rs"],
    );

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let deps = NamingOrchestratorDeps {
        naming_convention: Arc::new(NamingConventionChecker::new()),
        suffix_policy: Arc::new(SuffixPolicyChecker::new()),
        test_file_prefix: Arc::new(TestFilePrefixChecker::new()),
        config: config.clone(),
        layer_map: layer_map.clone(),
    };
    let orch = NamingOrchestrator::new(deps);
    let results = run_audit(&orch, &entries);

    assert!(
        results.is_empty(),
        "e2e clean files should produce zero violations, got: {:?}",
        results
    );
}

#[test]
fn e2e_mixed_files_partial_violations() {
    let tmp = TempDir::new().unwrap();
    let entries = make_file_entries(
        tmp.path(),
        &[
            "capabilities_user_checker.rs", // clean
            "capabilities_Bad_File.rs",     // AES101
            "capabilities_user_vo.rs",      // AES102
        ],
    );

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let deps = NamingOrchestratorDeps {
        naming_convention: Arc::new(NamingConventionChecker::new()),
        suffix_policy: Arc::new(SuffixPolicyChecker::new()),
        test_file_prefix: Arc::new(TestFilePrefixChecker::new()),
        config: config.clone(),
        layer_map: layer_map.clone(),
    };
    let orch = NamingOrchestrator::new(deps);
    let results = run_audit(&orch, &entries);

    assert!(
        results.len() >= 2,
        "should find at least 2 violations from mixed files, got {}",
        results.len()
    );
}

#[test]
fn e2e_container_wiring_produces_same_results() {
    let tmp = TempDir::new().unwrap();
    let entries = make_file_entries(
        tmp.path(),
        &["capabilities_BadFile.rs", "capabilities_user_vo.rs"],
    );

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let container = NamingContainer::new(config.clone(), layer_map.clone());
    let orch = container.orchestrator();
    let results = run_audit(orch.as_ref(), &entries);

    assert!(
        !results.is_empty(),
        "container-wired orchestrator should find violations"
    );
}

// ── FR-Config: Rule disabled in config → no violations for that rule ──

fn make_config_with_disabled_aes101() -> ArchitectureConfig {
    use shared_common::taxonomy_common_vo::BooleanVO;
    use shared_common::taxonomy_error_vo::ErrorCode;
    use shared_config_system::taxonomy_config_system_vo::ArchitectureRule;

    ArchitectureConfig {
        rules: vec![ArchitectureRule {
            name: shared_common::taxonomy_message_vo::DescriptionVO::new(
                "disable AES101".to_string(),
            ),
            description: shared_common::taxonomy_message_vo::DescriptionVO::new("".to_string()),
            rule_type: ErrorCode::raw("AES101"),
            enabled: BooleanVO::new(false),
            ..Default::default()
        }],
        ..ArchitectureConfig::default()
    }
}

#[test]
fn e2e_aes101_disabled_skips_convention_check() {
    let tmp = TempDir::new().unwrap();
    let entries = make_file_entries(
        tmp.path(),
        &[
            "capabilities_BadFile.rs",      // would fail AES101 if enabled
            "capabilities_user_checker.rs", // clean
        ],
    );

    let config = Arc::new(make_config_with_disabled_aes101());
    let layer_map = Arc::new(make_layer_map());
    let deps = NamingOrchestratorDeps {
        naming_convention: Arc::new(NamingConventionChecker::new()),
        suffix_policy: Arc::new(SuffixPolicyChecker::new()),
        test_file_prefix: Arc::new(TestFilePrefixChecker::new()),
        config,
        layer_map,
    };
    let orch = NamingOrchestrator::new(deps);
    let results = run_audit(&orch, &entries);

    let aes101_count = results
        .iter()
        .filter(|r| r.code.code().contains("AES101"))
        .count();
    assert_eq!(
        aes101_count, 0,
        "AES101 disabled in config must produce zero AES101 violations, got {}",
        aes101_count
    );
}

// ── AES103: the test-file prefix seam, end to end ──

/// Count the findings carrying *code*, so a case asserts on which rule spoke.
fn count_of(
    results: &[shared_common::taxonomy_violation_item_vo::ViolationItem],
    code: &str,
) -> usize {
    results.iter().filter(|r| r.code.code() == code).count()
}

#[test]
fn e2e_a_test_file_with_a_legal_prefix_is_clean() {
    let tmp = TempDir::new().unwrap();
    let source = make_file_entries(tmp.path(), &["capabilities_user_checker.rs"]);
    let tests = make_nested_entries(
        tmp.path(),
        &[
            "tests/contract_user_checker.rs",
            "tests/unit_user_checker.rs",
            "tests/integration_user_checker.rs",
            "tests/dogfood_user_checker.rs",
            "tests/smoke_user_checker.rs",
            "tests/e2e_user_checker.rs",
            "tests/acceptance_user_checker.rs",
            "benches/bench_user_checker.rs",
        ],
    );

    let config = Arc::new(ArchitectureConfig::default());
    let container = NamingContainer::new(config, Arc::new(make_layer_map()));
    let results = run_audit_with_tests(container.orchestrator().as_ref(), &source, &tests);

    assert_eq!(
        count_of(&results, "AES103"),
        0,
        "a suite carrying every type under a legal prefix must be clean; got {results:#?}"
    );
}

#[test]
fn e2e_an_illegal_test_prefix_fires_only_aes103() {
    let tmp = TempDir::new().unwrap();
    let source = make_file_entries(tmp.path(), &["capabilities_user_checker.rs"]);
    let tests = make_nested_entries(
        tmp.path(),
        &[
            "tests/contract_user_checker.rs",
            "tests/helpers.rs", // no legal prefix
        ],
    );

    let config = Arc::new(ArchitectureConfig::default());
    let container = NamingContainer::new(config, Arc::new(make_layer_map()));
    let results = run_audit_with_tests(container.orchestrator().as_ref(), &source, &tests);

    assert_eq!(
        count_of(&results, "AES103"),
        1,
        "exactly one file breaks the prefix rule; got {results:#?}"
    );
    assert_eq!(
        count_of(&results, "AES101"),
        0,
        "a test file must not be judged by AES101 — that is why the rule takes its \
         own file set rather than reading the source list; got {results:#?}"
    );
}

#[test]
fn e2e_a_nested_test_file_fires_the_flatness_half_only() {
    let tmp = TempDir::new().unwrap();
    let source = make_file_entries(tmp.path(), &["capabilities_user_checker.rs"]);
    let tests = make_nested_entries(tmp.path(), &["tests/inner/unit_user_checker.rs"]);

    let config = Arc::new(ArchitectureConfig::default());
    let container = NamingContainer::new(config, Arc::new(make_layer_map()));
    let results = run_audit_with_tests(container.orchestrator().as_ref(), &source, &tests);

    assert_eq!(
        count_of(&results, "AES103"),
        1,
        "nesting is one defect reported once, not a nesting finding plus a prefix \
         finding; got {results:#?}"
    );
}

#[test]
fn e2e_aes103_disabled_in_config_skips_the_prefix_check() {
    use shared_common::taxonomy_common_vo::BooleanVO;
    use shared_common::taxonomy_error_vo::ErrorCode;
    use shared_common::taxonomy_message_vo::DescriptionVO;
    use shared_config_system::taxonomy_config_system_vo::ArchitectureRule;

    let tmp = TempDir::new().unwrap();
    let source = make_file_entries(tmp.path(), &["capabilities_user_checker.rs"]);
    let tests = make_nested_entries(tmp.path(), &["tests/helpers.rs"]);

    let config = Arc::new(ArchitectureConfig {
        rules: vec![ArchitectureRule {
            name: DescriptionVO::new("disable AES103".to_string()),
            description: DescriptionVO::new(String::new()),
            rule_type: ErrorCode::raw("AES103"),
            enabled: BooleanVO::new(false),
            ..Default::default()
        }],
        ..ArchitectureConfig::default()
    });
    let container = NamingContainer::new(config, Arc::new(make_layer_map()));
    let results = run_audit_with_tests(container.orchestrator().as_ref(), &source, &tests);

    assert_eq!(
        count_of(&results, "AES103"),
        0,
        "AES103 disabled in config must produce zero AES103 violations, got {results:#?}"
    );
}
