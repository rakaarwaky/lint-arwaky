// E2E tests — full pipeline: temp dir → config → container → audit → violations.
use naming_rules_lint_arwaky::agent_naming_orchestrator::{NamingOrchestrator, NamingOrchestratorDeps};
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::PatternList;
use shared::common::SuffixPolicyVO;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::INamingRunnerAggregate;
use shared::naming_rules::SUFFIX_POLICY_STRICT;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use std::collections::HashMap;
use std::path::PathBuf;
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
            std::fs::write(&path, "fn dummy() {}").unwrap();
            FileEntry {
                path,
                extension: "rs".to_string(),
                language: Language::Rust,
                size: 13,
                content: "fn dummy() {}".to_string(),
                parse_ok: false,
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
            "capabilities_BadFile.rs",   // uppercase → AES101
            "capabilities_user_checker.rs", // clean → passes
        ],
    );

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let deps = NamingOrchestratorDeps {
        naming_convention_checker: Arc::new(NamingConventionChecker::new()),
        suffix_prefix_checker: Arc::new(SuffixPrefixChecker::new()),
        config: config.clone(),
        layer_map: layer_map.clone(),
    };
    let orch = NamingOrchestrator::new(deps);
    let results = orch.run_audit_with_entries(&entries);

    assert!(
        !results.is_empty(),
        "e2e should find at least one AES101 violation"
    );
    let has_aes101 = results.iter().any(|r| r.code.value().contains("AES101"));
    assert!(has_aes101, "should contain AES101 code");
}

#[test]
fn e2e_suffix_violations_found() {
    let tmp = TempDir::new().unwrap();
    let entries = make_file_entries(
        tmp.path(),
        &[
            "capabilities_user_vo.rs",   // forbidden suffix 'vo' → AES102
            "capabilities_user_handler.rs", // not in allowed list → AES102
        ],
    );

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let deps = NamingOrchestratorDeps {
        naming_convention_checker: Arc::new(NamingConventionChecker::new()),
        suffix_prefix_checker: Arc::new(SuffixPrefixChecker::new()),
        config: config.clone(),
        layer_map: layer_map.clone(),
    };
    let orch = NamingOrchestrator::new(deps);
    let results = orch.run_audit_with_entries(&entries);

    assert!(
        !results.is_empty(),
        "e2e should find at least one AES102 violation"
    );
    let has_aes102 = results.iter().any(|r| r.code.value().contains("AES102"));
    assert!(has_aes102, "should contain AES102 code");
}

#[test]
fn e2e_clean_files_no_violations() {
    let tmp = TempDir::new().unwrap();
    let entries = make_file_entries(
        tmp.path(),
        &[
            "capabilities_user_checker.rs",
            "capabilities_db_adapter.rs",
        ],
    );

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let deps = NamingOrchestratorDeps {
        naming_convention_checker: Arc::new(NamingConventionChecker::new()),
        suffix_prefix_checker: Arc::new(SuffixPrefixChecker::new()),
        config: config.clone(),
        layer_map: layer_map.clone(),
    };
    let orch = NamingOrchestrator::new(deps);
    let results = orch.run_audit_with_entries(&entries);

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
        naming_convention_checker: Arc::new(NamingConventionChecker::new()),
        suffix_prefix_checker: Arc::new(SuffixPrefixChecker::new()),
        config: config.clone(),
        layer_map: layer_map.clone(),
    };
    let orch = NamingOrchestrator::new(deps);
    let results = orch.run_audit_with_entries(&entries);

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
    let results = orch.run_audit_with_entries(&entries);

    assert!(
        !results.is_empty(),
        "container-wired orchestrator should find violations"
    );
}
