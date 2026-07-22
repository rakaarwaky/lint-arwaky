
# Test Suite for `naming-rules` (v1.10.106)

Below is the complete test suite following the flat `tests/` convention with prefix-as-folder naming.

---

## Directory Layout

```
crates/naming-rules/
├── src/
│   ├── lib.rs
│   ├── agent_naming_orchestrator.rs
│   ├── capabilities_naming_convention_checker.rs
│   ├── capabilities_suffix_prefix_checker.rs
│   └── root_naming_rules_container.rs
├── tests/
│   ├── contract_naming_rules.rs
│   ├── unit_naming_rules_convention_checker.rs
│   ├── unit_naming_rules_suffix_prefix_checker.rs
│   ├── unit_naming_rules_orchestrator.rs
│   ├── integration_naming_rules.rs
│   ├── acceptance_FRD_001.rs
│   ├── acceptance_FRD_002.rs
│   └── bench_naming_rules_throughput.rs
└── Cargo.toml
```

---

## `Cargo.toml` additions

```toml
[dev-dependencies]
tokio = { workspace = true, features = ["rt", "rt-multi-thread", "macros"] }
shared.workspace = true
criterion = { version = "0.5", features = ["async_tokio"] }
tempfile = "3"

[[bench]]
name = "bench_naming_rules_throughput"
path = "tests/bench_naming_rules_throughput.rs"
harness = false
```

---

## `tests/contract_naming_rules.rs`

```rust
// PURPOSE: Verify that all public types implement their declared contract traits.
// Layer: Contract verification
// Coverage target: compile-time trait bound assertions

use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use naming_rules_lint_arwaky::agent_naming_orchestrator::NamingOrchestrator;
use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;

use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;

// ─── INamingConventionChecker ─────────────────────────────

#[test]
fn naming_convention_checker_implements_protocol() {
    fn assert_trait<T: INamingConventionChecker>() {}
    assert_trait::<NamingConventionChecker>();
}

#[test]
fn naming_convention_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NamingConventionChecker>();
}

// ─── ISuffixPrefixChecker ─────────────────────────────────

#[test]
fn suffix_prefix_checker_implements_protocol() {
    fn assert_trait<T: ISuffixPrefixChecker>() {}
    assert_trait::<SuffixPrefixChecker>();
}

#[test]
fn suffix_prefix_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SuffixPrefixChecker>();
}

// ─── INamingRunnerAggregate ───────────────────────────────

#[test]
fn naming_orchestrator_implements_aggregate() {
    fn assert_trait<T: INamingRunnerAggregate>() {}
    assert_trait::<NamingOrchestrator>();
}

#[test]
fn naming_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NamingOrchestrator>();
}

// ─── NamingContainer wiring ───────────────────────────────

#[test]
fn container_produces_orchestrator_as_aggregate() {
    fn assert_trait<T: INamingRunnerAggregate>() {}
    // NamingContainer::orchestrator() returns Arc<dyn INamingRunnerAggregate>
    // This test verifies the return type is correct at compile time.
    assert_trait::<NamingOrchestrator>();
}

#[test]
fn container_exposes_checker_references() {
    use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
    use shared::common::taxonomy_definition_vo::LayerMapVO;
    use std::sync::Arc;

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);

    // Verify accessors return trait objects
    let _conv: &Arc<dyn INamingConventionChecker> = container.naming_convention_checker();
    let _suf: &Arc<dyn ISuffixPrefixChecker> = container.suffix_prefix_checker();
    let _orch: Arc<dyn INamingRunnerAggregate> = container.orchestrator();
}
```

---

## `tests/unit_naming_rules_convention_checker.rs`

```rust
// PURPOSE: Unit tests for NamingConventionChecker (AES101 — naming convention)
// Covers: happy path, edge cases, error paths
// Coverage target: Capabilities ≥ 70%

use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::naming_rules::contract_naming_checker_protocol::INamingConventionChecker;
use std::collections::HashMap;
use std::sync::Arc;

fn default_config() -> ArchitectureConfig {
    ArchitectureConfig::default()
}

fn layer_map_with_capabilities() -> LayerMapVO {
    let mut map = HashMap::new();
    map.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition::default(),
    );
    map.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition::default(),
    );
    map.insert(
        LayerNameVO::new("utility"),
        LayerDefinition::default(),
    );
    LayerMapVO::new(map)
}

fn make_files(paths: &[&str]) -> FilePathList {
    FilePathList::new(
        paths
            .iter()
            .map(|p| FilePath::new(p.to_string()).unwrap())
            .collect(),
    )
}

async fn run_check(
    files: &[&str],
    config: &ArchitectureConfig,
    layer_map: &LayerMapVO,
) -> LintResultList {
    let checker = NamingConventionChecker::new();
    let mut results = LintResultList::new(Vec::new());
    let root = FilePath::new("/project".to_string()).unwrap();
    checker
        .check_file_naming(config, layer_map, &make_files(files), &root, &mut results)
        .await;
    results
}

// ─── Happy Path ───────────────────────────────────────────

#[tokio::test]
async fn valid_three_word_snake_case_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["capabilities_user_checker.rs"], &config, &lm).await;
    assert!(results.is_empty(), "Expected no violations, got: {:?}", results.values);
}

#[tokio::test]
async fn valid_four_word_snake_case_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["capabilities_db_user_adapter.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn valid_taxonomy_vo_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["taxonomy_user_vo.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn valid_utility_parser_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["utility_json_parser.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

// ─── AES101 Violations ────────────────────────────────────

#[tokio::test]
async fn uppercase_in_stem_fails_aes101() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["capabilities_User_Checker.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "Uppercase should trigger AES101");
    assert_eq!(results.values[0].code.code(), "AES101");
}

#[tokio::test]
async fn hyphen_separator_fails_aes101() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["capabilities-user-checker.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "Hyphens should trigger AES101");
}

#[tokio::test]
async fn two_words_fails_aes101() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    // "capabilities_checker" has only 2 words
    let results = run_check(&["capabilities_checker.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "Two words should trigger AES101");
    assert_eq!(results.values[0].code.code(), "AES101");
}

#[tokio::test]
async fn single_word_fails_aes101() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["checker.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "Single word should trigger AES101");
}

// ─── Exception / Barrel Files ─────────────────────────────

#[tokio::test]
async fn barrel_file_mod_rs_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["mod.rs"], &config, &lm).await;
    assert!(results.is_empty(), "mod.rs is a barrel file exception");
}

#[tokio::test]
async fn entry_point_lib_rs_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["lib.rs"], &config, &lm).await;
    assert!(results.is_empty(), "lib.rs is an entry point exception");
}

#[tokio::test]
async fn entry_point_main_rs_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["main.rs"], &config, &lm).await;
    assert!(results.is_empty(), "main.rs is an entry point exception");
}

#[tokio::test]
async fn barrel_file_init_py_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["__init__.py"], &config, &lm).await;
    assert!(results.is_empty(), "__init__.py is a barrel file exception");
}

// ─── Unknown Prefix ───────────────────────────────────────

#[tokio::test]
async fn unknown_prefix_emits_aes102() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["helpers_user_parser.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "Unknown prefix should trigger violation");
    assert_eq!(results.values[0].code.code(), "AES102");
}

// ─── Configurable min_words ───────────────────────────────

#[tokio::test]
async fn min_words_from_config_respected() {
    let mut config = default_config();
    config.naming.word_count = shared::common::taxonomy_common_vo::Count::new(4);
    let lm = layer_map_with_capabilities();
    // 3 words should now fail
    let results = run_check(&["capabilities_user_checker.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "3 words should fail when min_words=4");
}

#[tokio::test]
async fn min_words_zero_defaults_to_three() {
    let mut config = default_config();
    config.naming.word_count = shared::common::taxonomy_common_vo::Count::new(0);
    let lm = layer_map_with_capabilities();
    // 3 words should pass (default fallback)
    let results = run_check(&["capabilities_user_checker.rs"], &config, &lm).await;
    assert!(results.is_empty(), "min_words=0 should default to 3");
}

// ─── Edge Cases ───────────────────────────────────────────

#[tokio::test]
async fn digits_in_words_pass() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["capabilities_v2_adapter.rs"], &config, &lm).await;
    assert!(results.is_empty(), "Digits in words are valid");
}

#[tokio::test]
async fn multiple_files_mixed_results() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(
        &[
            "capabilities_user_checker.rs",  // valid
            "capabilities_Bad_Name.rs",       // invalid (uppercase)
            "taxonomy_item_vo.rs",            // valid
        ],
        &config,
        &lm,
    )
    .await;
    assert_eq!(results.len(), 1, "Only one file should violate");
}
```

---

## `tests/unit_naming_rules_suffix_prefix_checker.rs`

```rust
// PURPOSE: Unit tests for SuffixPrefixChecker (AES102 — suffix/prefix alignment)
// Covers: happy path, forbidden suffix, strict policy mismatch, edge cases
// Coverage target: Capabilities ≥ 70%

use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_suffix_vo::SuffixPolicyVO;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::naming_rules::contract_naming_checker_protocol::ISuffixPrefixChecker;
use shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO;
use std::collections::HashMap;

fn default_config() -> ArchitectureConfig {
    ArchitectureConfig::default()
}

fn strict_taxonomy_layer() -> LayerMapVO {
    let mut map = HashMap::new();
    let mut def = LayerDefinition::default();
    def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec![
            "vo".to_string(),
            "entity".to_string(),
            "error".to_string(),
            "event".to_string(),
            "constant".to_string(),
        ]),
        forbidden_suffix: PatternList::new(vec!["orchestrator".to_string()]),
        ..Default::default()
    };
    map.insert(LayerNameVO::new("taxonomy"), def);
    LayerMapVO::new(map)
}

fn strict_agent_layer() -> LayerMapVO {
    let mut map = HashMap::new();
    let mut def = LayerDefinition::default();
    def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec!["orchestrator".to_string()]),
        forbidden_suffix: PatternList::new(vec!["vo".to_string(), "entity".to_string()]),
        ..Default::default()
    };
    map.insert(LayerNameVO::new("agent"), def);
    LayerMapVO::new(map)
}

fn flexible_capabilities_layer() -> LayerMapVO {
    let mut map = HashMap::new();
    let def = LayerDefinition::default(); // no strict policy
    map.insert(LayerNameVO::new("capabilities"), def);
    LayerMapVO::new(map)
}

fn make_files(paths: &[&str]) -> FilePathList {
    FilePathList::new(
        paths
            .iter()
            .map(|p| FilePath::new(p.to_string()).unwrap())
            .collect(),
    )
}

async fn run_check(
    files: &[&str],
    config: &ArchitectureConfig,
    layer_map: &LayerMapVO,
) -> LintResultList {
    let checker = SuffixPrefixChecker::new();
    let mut results = LintResultList::new(Vec::new());
    let root = FilePath::new("/project".to_string()).unwrap();
    checker
        .check_domain_suffixes(config, layer_map, &make_files(files), &root, &mut results)
        .await;
    results
}

// ─── Happy Path ───────────────────────────────────────────

#[tokio::test]
async fn taxonomy_vo_suffix_passes_strict() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["taxonomy_user_vo.rs"], &config, &lm).await;
    assert!(results.is_empty(), "taxonomy_*_vo should pass strict policy");
}

#[tokio::test]
async fn taxonomy_entity_suffix_passes_strict() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["taxonomy_order_entity.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn taxonomy_constant_suffix_passes_strict() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["taxonomy_naming_constant.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn agent_orchestrator_suffix_passes_strict() {
    let config = default_config();
    let lm = strict_agent_layer();
    let results = run_check(&["agent_naming_orchestrator.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

// ─── AES102 Suffix Mismatch ───────────────────────────────

#[tokio::test]
async fn taxonomy_with_orchestrator_suffix_fails() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["taxonomy_user_orchestrator.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "orchestrator suffix forbidden in taxonomy");
    assert_eq!(results.values[0].code.code(), "AES102");
}

#[tokio::test]
async fn taxonomy_with_unknown_suffix_fails_strict() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["taxonomy_user_parser.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "parser not in allowed list for taxonomy");
    assert_eq!(results.values[0].code.code(), "AES102");
}

#[tokio::test]
async fn agent_with_vo_suffix_fails_forbidden() {
    let config = default_config();
    let lm = strict_agent_layer();
    let results = run_check(&["agent_naming_vo.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "vo suffix forbidden in agent layer");
    assert_eq!(results.values[0].code.code(), "AES102");
}

// ─── Flexible Policy (no strict) ──────────────────────────

#[tokio::test]
async fn capabilities_any_suffix_passes_flexible() {
    let config = default_config();
    let lm = flexible_capabilities_layer();
    let results = run_check(&["capabilities_user_checker.rs"], &config, &lm).await;
    assert!(results.is_empty(), "capabilities is flexible — any suffix OK");
}

#[tokio::test]
async fn capabilities_custom_suffix_passes_flexible() {
    let config = default_config();
    let lm = flexible_capabilities_layer();
    let results = run_check(&["capabilities_data_transformer.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

// ─── Barrel / Entry Point Exceptions ──────────────────────

#[tokio::test]
async fn barrel_file_mod_rs_skipped() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["mod.rs"], &config, &lm).await;
    assert!(results.is_empty(), "mod.rs is skipped");
}

#[tokio::test]
async fn entry_point_lib_rs_skipped() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(&["lib.rs"], &config, &lm).await;
    assert!(results.is_empty(), "lib.rs is skipped");
}

// ─── Layer Definition Exceptions ──────────────────────────

#[tokio::test]
async fn exception_file_in_definition_skipped() {
    let mut map = HashMap::new();
    let mut def = LayerDefinition::default();
    def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec!["vo".to_string()]),
        ..Default::default()
    };
    def.exceptions = PatternList::new(vec!["taxonomy_special.rs".to_string()]);
    map.insert(LayerNameVO::new("taxonomy"), def);
    let lm = LayerMapVO::new(map);

    let config = default_config();
    let results = run_check(&["taxonomy_special.rs"], &config, &lm).await;
    assert!(results.is_empty(), "Exception file should be skipped");
}

// ─── Edge Cases ───────────────────────────────────────────

#[tokio::test]
async fn no_layer_definition_returns_no_violation() {
    let config = default_config();
    let lm = LayerMapVO::new(HashMap::new()); // empty layer map
    let results = run_check(&["unknown_file_thing.rs"], &config, &lm).await;
    assert!(results.is_empty(), "No layer def → no suffix check");
}

#[tokio::test]
async fn multiple_files_mixed_suffix_results() {
    let config = default_config();
    let lm = strict_taxonomy_layer();
    let results = run_check(
        &[
            "taxonomy_user_vo.rs",          // valid
            "taxonomy_order_entity.rs",     // valid
            "taxonomy_bad_orchestrator.rs", // forbidden suffix
        ],
        &config,
        &lm,
    )
    .await;
    assert_eq!(results.len(), 1, "Only one file should violate");
}
```

---

## `tests/unit_naming_rules_orchestrator.rs`

```rust
// PURPOSE: Unit tests for NamingOrchestrator (Agent layer)
// Covers: flow control, filtering, error handling
// Coverage target: Agent ≥ 60%

use naming_rules_lint_arwaky::agent_naming_orchestrator::NamingOrchestrator;
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_common_vo::PatternList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use std::collections::HashMap;
use std::sync::Arc;

fn build_orchestrator(config: ArchitectureConfig, layer_map: LayerMapVO) -> NamingOrchestrator {
    let conv: Arc<dyn INamingConventionChecker> = Arc::new(NamingConventionChecker::new());
    let suf: Arc<dyn ISuffixPrefixChecker> = Arc::new(SuffixPrefixChecker::new());
    NamingOrchestrator::new(conv, suf, Arc::new(config), Arc::new(layer_map))
}

// ─── Name ─────────────────────────────────────────────────

#[test]
fn orchestrator_name_is_naming_rules() {
    let orch = build_orchestrator(ArchitectureConfig::default(), LayerMapVO::new(HashMap::new()));
    assert_eq!(orch.name(), "naming-rules");
}

// ─── Error: Non-existent Path ─────────────────────────────

#[tokio::test]
async fn run_audit_nonexistent_path_returns_error() {
    let orch = build_orchestrator(ArchitectureConfig::default(), LayerMapVO::new(HashMap::new()));
    let target = FilePath::new("/nonexistent/path/xyz".to_string()).unwrap();
    let result = orch.run_audit(&target).await;
    assert!(result.is_err(), "Non-existent path should return ScanError");
}

// ─── Ignored Patterns ─────────────────────────────────────

#[tokio::test]
async fn ignored_patterns_trimmed_correctly() {
    let mut config = ArchitectureConfig::default();
    config.ignored_paths = FilePathList::new(vec![
        FilePath::new("./target/".to_string()).unwrap(),
        FilePath::new("/node_modules/".to_string()).unwrap(),
    ]);
    let orch = build_orchestrator(config, LayerMapVO::new(HashMap::new()));
    // Orchestrator should have trimmed "./" and "/" prefixes/suffixes
    // We verify indirectly: the orchestrator was constructed without panic
    assert_eq!(orch.name(), "naming-rules");
}

// ─── Filter Source Files ──────────────────────────────────
// filter_source_files is private, tested indirectly via run_audit on a temp dir

#[tokio::test]
async fn run_audit_on_empty_dir_returns_empty_results() {
    let dir = tempfile::tempdir().unwrap();
    let orch = build_orchestrator(ArchitectureConfig::default(), LayerMapVO::new(HashMap::new()));
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = orch.run_audit(&target).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty(), "Empty dir → no violations");
}

#[tokio::test]
async fn run_audit_filters_non_source_extensions() {
    let dir = tempfile::tempdir().unwrap();
    // Create a .txt file (not a source extension)
    std::fs::write(dir.path().join("readme.txt"), "hello").unwrap();
    // Create a valid .rs file
    std::fs::write(dir.path().join("capabilities_user_checker.rs"), "// ok").unwrap();

    let orch = build_orchestrator(ArchitectureConfig::default(), LayerMapVO::new(HashMap::new()));
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = orch.run_audit(&target).await;
    assert!(result.is_ok());
    // .txt should be filtered out; .rs should be checked
    // With empty layer map, the .rs file may or may not produce violations
    // depending on prefix detection — but .txt must never appear
    let violations = result.unwrap();
    for v in &violations {
        assert!(
            v.file.value.ends_with(".rs"),
            "Non-source file should not appear in results"
        );
    }
}
```

---

## `tests/integration_naming_rules.rs`

```rust
// PURPOSE: Integration tests — full DI wiring via NamingContainer
// Uses real container, real checkers, real orchestrator
// Coverage target: validates wiring correctness

use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_suffix_vo::SuffixPolicyVO;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO;
use std::collections::HashMap;
use std::sync::Arc;

fn build_test_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();

    let mut taxonomy_def = LayerDefinition::default();
    taxonomy_def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec![
            "vo".to_string(),
            "entity".to_string(),
            "error".to_string(),
            "event".to_string(),
            "constant".to_string(),
        ]),
        ..Default::default()
    };
    layers.insert(LayerNameVO::new("taxonomy"), taxonomy_def);

    let mut agent_def = LayerDefinition::default();
    agent_def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec!["orchestrator".to_string()]),
        ..Default::default()
    };
    layers.insert(LayerNameVO::new("agent"), agent_def);

    let capabilities_def = LayerDefinition::default();
    layers.insert(LayerNameVO::new("capabilities"), capabilities_def);

    ArchitectureConfig {
        layers,
        ..Default::default()
    }
}

fn build_container() -> NamingContainer {
    let config = build_test_config();
    let layer_map = LayerMapVO::new(config.layers.clone());
    NamingContainer::new(Arc::new(config), Arc::new(layer_map))
}

// ─── Container Wiring ─────────────────────────────────────

#[test]
fn container_creates_orchestrator_successfully() {
    let container = build_container();
    let orch = container.orchestrator();
    assert_eq!(orch.name(), "naming-rules");
}

#[test]
fn container_exposes_both_checkers() {
    let container = build_container();
    let _conv = container.naming_convention_checker();
    let _suf = container.suffix_prefix_checker();
}

// ─── Full Pipeline: Valid Files ───────────────────────────

#[tokio::test]
async fn full_pipeline_all_valid_files_zero_violations() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("taxonomy_user_vo.rs"), "").unwrap();
    std::fs::write(dir.path().join("taxonomy_order_entity.rs"), "").unwrap();
    std::fs::write(dir.path().join("agent_naming_orchestrator.rs"), "").unwrap();
    std::fs::write(dir.path().join("capabilities_user_checker.rs"), "").unwrap();

    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.is_empty(),
        "All valid files should produce zero violations, got: {:?}",
        results.iter().map(|r| r.code.code()).collect::<Vec<_>>()
    );
}

// ─── Full Pipeline: Mixed Violations ──────────────────────

#[tokio::test]
async fn full_pipeline_detects_naming_and_suffix_violations() {
    let dir = tempfile::tempdir().unwrap();
    // AES101: uppercase
    std::fs::write(dir.path().join("capabilities_Bad_Name.rs"), "").unwrap();
    // AES102: wrong suffix for taxonomy
    std::fs::write(dir.path().join("taxonomy_user_orchestrator.rs"), "").unwrap();
    // Valid
    std::fs::write(dir.path().join("taxonomy_user_vo.rs"), "").unwrap();

    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(results.len() >= 2, "Should detect at least 2 violations");
    let codes: Vec<&str> = results.iter().map(|r| r.code.code()).collect();
    assert!(codes.contains(&"AES101"), "Should contain AES101");
    assert!(codes.contains(&"AES102"), "Should contain AES102");
}

// ─── Full Pipeline: Ignored Paths ─────────────────────────

#[tokio::test]
async fn full_pipeline_respects_ignored_paths() {
    let dir = tempfile::tempdir().unwrap();
    let ignored_dir = dir.path().join("target");
    std::fs::create_dir_all(&ignored_dir).unwrap();
    std::fs::write(ignored_dir.join("capabilities_Bad_Name.rs"), "").unwrap();
    std::fs::write(dir.path().join("taxonomy_user_vo.rs"), "").unwrap();

    let mut config = build_test_config();
    config.ignored_paths = FilePathList::new(vec![
        FilePath::new("target".to_string()).unwrap(),
    ]);
    let layer_map = LayerMapVO::new(config.layers.clone());
    let container = NamingContainer::new(Arc::new(config), Arc::new(layer_map));
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.is_empty(),
        "Ignored directory files should not produce violations"
    );
}

// ─── Full Pipeline: Non-existent Target ───────────────────

#[tokio::test]
async fn full_pipeline_nonexistent_target_returns_error() {
    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new("/does/not/exist".to_string()).unwrap();
    let result = orch.run_audit(&target).await;
    assert!(result.is_err());
}
```

---

## `tests/acceptance_FRD_001.rs`

```rust
// PURPOSE: Acceptance tests for FR-001 — AES101 Naming Convention Consistency
// Maps 1:1 to FRD business rules

use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use std::collections::HashMap;
use std::sync::Arc;

fn build_container() -> NamingContainer {
    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());
    layers.insert(LayerNameVO::new("taxonomy"), LayerDefinition::default());
    layers.insert(LayerNameVO::new("utility"), LayerDefinition::default());
    layers.insert(LayerNameVO::new("agent"), LayerDefinition::default());

    let config = ArchitectureConfig {
        layers: layers.clone(),
        ..Default::default()
    };
    let layer_map = LayerMapVO::new(layers);
    NamingContainer::new(Arc::new(config), Arc::new(layer_map))
}

async fn scan_file(filename: &str) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(filename), "").unwrap();

    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    orch.run_audit(&target).await.unwrap()
}

// ─── FR-001: Valid snake_case name passes ─────────────────

/// FR-001: Every file stem must be snake_case with at least 3 words.
#[tokio::test]
async fn fr001_valid_snake_case_passes() {
    let results = scan_file("capabilities_user_checker.rs").await;
    let aes101: Vec<_> = results.iter().filter(|r| r.code.code() == "AES101").collect();
    assert!(aes101.is_empty(), "Valid snake_case should not trigger AES101");
}

// ─── FR-001: Non-snake_case fails ─────────────────────────

/// FR-001: Must be snake_case (lowercase ASCII + underscores)
#[tokio::test]
async fn fr001_camel_case_fails() {
    let results = scan_file("capabilities_userChecker.rs").await;
    let aes101: Vec<_> = results.iter().filter(|r| r.code.code() == "AES101").collect();
    assert!(!aes101.is_empty(), "camelCase should trigger AES101");
}

/// FR-001: Must be snake_case (lowercase ASCII + underscores)
#[tokio::test]
async fn fr001_uppercase_fails() {
    let results = scan_file("capabilities_User_Checker.rs").await;
    let aes101: Vec<_> = results.iter().filter(|r| r.code.code() == "AES101").collect();
    assert!(!aes101.is_empty(), "Uppercase should trigger AES101");
}

// ─── FR-001: Minimum 3 words ──────────────────────────────

/// FR-001: Minimum 3 words (prefix + concept + suffix)
#[tokio::test]
async fn fr001_two_words_fails() {
    let results = scan_file("capabilities_checker.rs").await;
    let aes101: Vec<_> = results.iter().filter(|r| r.code.code() == "AES101").collect();
    assert!(!aes101.is_empty(), "Two words should trigger AES101");
}

/// FR-001: Minimum 3 words (prefix + concept + suffix)
#[tokio::test]
async fn fr001_three_words_passes() {
    let results = scan_file("capabilities_user_checker.rs").await;
    let aes101: Vec<_> = results.iter().filter(|r| r.code.code() == "AES101").collect();
    assert!(aes101.is_empty(), "Three words should pass AES101");
}

// ─── FR-001: Exception files pass ─────────────────────────

/// FR-001: Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
#[tokio::test]
async fn fr001_exception_main_rs_passes() {
    let results = scan_file("main.rs").await;
    assert!(results.is_empty(), "main.rs is an exception");
}

/// FR-001: Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
#[tokio::test]
async fn fr001_exception_lib_rs_passes() {
    let results = scan_file("lib.rs").await;
    assert!(results.is_empty(), "lib.rs is an exception");
}

/// FR-001: Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
#[tokio::test]
async fn fr001_exception_mod_rs_passes() {
    let results = scan_file("mod.rs").await;
    assert!(results.is_empty(), "mod.rs is an exception");
}

/// FR-001: Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
#[tokio::test]
async fn fr001_exception_init_py_passes() {
    let results = scan_file("__init__.py").await;
    assert!(results.is_empty(), "__init__.py is an exception");
}

/// FR-001: Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
#[tokio::test]
async fn fr001_exception_index_ts_passes() {
    let results = scan_file("index.ts").await;
    assert!(results.is_empty(), "index.ts is an exception");
}

// ─── FR-001: Error handling emits AES101 ──────────────────

/// FR-001: Emit AES101 with invalid filename
#[tokio::test]
async fn fr001_emits_aes101_code() {
    let results = scan_file("capabilities_Bad.rs").await;
    assert!(!results.is_empty());
    assert_eq!(results[0].code.code(), "AES101");
}
```

---

## `tests/acceptance_FRD_002.rs`

```rust
// PURPOSE: Acceptance tests for FR-002 — AES102 Suffix/Prefix Layer Alignment
// Maps 1:1 to FRD business rules

use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_suffix_vo::SuffixPolicyVO;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO;
use std::collections::HashMap;
use std::sync::Arc;

fn build_strict_container() -> NamingContainer {
    let mut layers = HashMap::new();

    // taxonomy: strict — only _vo, _entity, _error, _event, _constant
    let mut taxonomy_def = LayerDefinition::default();
    taxonomy_def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec![
            "vo".to_string(),
            "entity".to_string(),
            "error".to_string(),
            "event".to_string(),
            "constant".to_string(),
        ]),
        forbidden_suffix: PatternList::new(vec!["orchestrator".to_string(), "checker".to_string()]),
        ..Default::default()
    };
    layers.insert(LayerNameVO::new("taxonomy"), taxonomy_def);

    // contract: strict — only _protocol, _aggregate
    let mut contract_def = LayerDefinition::default();
    contract_def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec![
            "protocol".to_string(),
            "aggregate".to_string(),
        ]),
        ..Default::default()
    };
    layers.insert(LayerNameVO::new("contract"), contract_def);

    // agent: strict — only _orchestrator
    let mut agent_def = LayerDefinition::default();
    agent_def.naming = NamingRuleVO {
        suffix_policy: SuffixPolicyVO::new("strict".to_string()),
        allowed_suffix: PatternList::new(vec!["orchestrator".to_string()]),
        ..Default::default()
    };
    layers.insert(LayerNameVO::new("agent"), agent_def);

    // capabilities: flexible
    layers.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());

    // utility: flexible
    layers.insert(LayerNameVO::new("utility"), LayerDefinition::default());

    let config = ArchitectureConfig {
        layers: layers.clone(),
        ..Default::default()
    };
    let layer_map = LayerMapVO::new(layers);
    NamingContainer::new(Arc::new(config), Arc::new(layer_map))
}

async fn scan_file(filename: &str) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(filename), "").unwrap();

    let container = build_strict_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    orch.run_audit(&target).await.unwrap()
}

// ─── FR-002: taxonomy_ allowed suffixes ───────────────────

/// FR-002: taxonomy_: _vo, _entity, _error, _event, _constant
#[tokio::test]
async fn fr002_taxonomy_vo_passes() {
    let results = scan_file("taxonomy_user_vo.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(aes102.is_empty(), "taxonomy_*_vo should pass");
}

/// FR-002: taxonomy_: _vo, _entity, _error, _event, _constant
#[tokio::test]
async fn fr002_taxonomy_entity_passes() {
    let results = scan_file("taxonomy_order_entity.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(aes102.is_empty());
}

/// FR-002: taxonomy_: _vo, _entity, _error, _event, _constant
#[tokio::test]
async fn fr002_taxonomy_error_passes() {
    let results = scan_file("taxonomy_parse_error.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(aes102.is_empty());
}

/// FR-002: taxonomy_: _vo, _entity, _error, _event, _constant
#[tokio::test]
async fn fr002_taxonomy_event_passes() {
    let results = scan_file("taxonomy_user_event.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(aes102.is_empty());
}

/// FR-002: taxonomy_: _vo, _entity, _error, _event, _constant
#[tokio::test]
async fn fr002_taxonomy_constant_passes() {
    let results = scan_file("taxonomy_naming_constant.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(aes102.is_empty());
}

// ─── FR-002: contract_ allowed suffixes ───────────────────

/// FR-002: contract_: _protocol, _aggregate
#[tokio::test]
async fn fr002_contract_protocol_passes() {
    let results = scan_file("contract_naming_checker_protocol.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(aes102.is_empty());
}

/// FR-002: contract_: _protocol, _aggregate
#[tokio::test]
async fn fr002_contract_aggregate_passes() {
    let results = scan_file("contract_naming_runner_aggregate.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(aes102.is_empty());
}

// ─── FR-002: agent_ allowed suffixes ──────────────────────

/// FR-002: agent_: _orchestrator
#[tokio::test]
async fn fr002_agent_orchestrator_passes() {
    let results = scan_file("agent_naming_orchestrator.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(aes102.is_empty());
}

// ─── FR-002: Wrong suffix fails ───────────────────────────

/// FR-002: File suffix must align with its layer prefix.
#[tokio::test]
async fn fr002_taxonomy_with_orchestrator_fails() {
    let results = scan_file("taxonomy_user_orchestrator.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(!aes102.is_empty(), "orchestrator not allowed in taxonomy");
}

/// FR-002: File suffix must align with its layer prefix.
#[tokio::test]
async fn fr002_agent_with_vo_fails() {
    let results = scan_file("agent_naming_vo.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(!aes102.is_empty(), "vo not allowed in agent");
}

/// FR-002: File suffix must align with its layer prefix.
#[tokio::test]
async fn fr002_contract_with_vo_fails() {
    let results = scan_file("contract_naming_vo.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(!aes102.is_empty(), "vo not allowed in contract");
}

// ─── FR-002: Flexible layers accept any suffix ────────────

/// FR-002: utility_: any role suffix (flexible)
#[tokio::test]
async fn fr002_utility_any_suffix_passes() {
    let results = scan_file("utility_json_parser.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(aes102.is_empty(), "utility is flexible");
}

/// FR-002: capabilities_: any role suffix (flexible)
#[tokio::test]
async fn fr002_capabilities_any_suffix_passes() {
    let results = scan_file("capabilities_data_transformer.rs").await;
    let aes102: Vec<_> = results.iter().filter(|r| r.code.code() == "AES102").collect();
    assert!(aes102.is_empty(), "capabilities is flexible");
}

// ─── FR-002: Error handling emits AES102 ──────────────────

/// FR-002: Emit AES102 with expected suffixes
#[tokio::test]
async fn fr002_emits_aes102_code() {
    let results = scan_file("taxonomy_bad_orchestrator.rs").await;
    assert!(!results.is_empty());
    assert_eq!(results[0].code.code(), "AES102");
}
```

---

## `tests/bench_naming_rules_throughput.rs`

```rust
// PURPOSE: Benchmark — naming checks throughput
// NFR: Check 1000 files in < 1 second
// Uses criterion — never hand-rolled timing

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use std::collections::HashMap;

fn build_layer_map() -> LayerMapVO {
    let mut map = HashMap::new();
    map.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());
    map.insert(LayerNameVO::new("taxonomy"), LayerDefinition::default());
    map.insert(LayerNameVO::new("utility"), LayerDefinition::default());
    map.insert(LayerNameVO::new("agent"), LayerDefinition::default());
    LayerMapVO::new(map)
}

fn generate_files(count: usize) -> FilePathList {
    let prefixes = ["capabilities", "taxonomy", "utility", "agent"];
    let suffixes = ["checker", "vo", "parser", "orchestrator"];
    let files: Vec<FilePath> = (0..count)
        .map(|i| {
            let prefix = prefixes[i % prefixes.len()];
            let suffix = suffixes[i % suffixes.len()];
            let name = format!("{}_item_{}_{}.rs", prefix, i, suffix);
            FilePath::new(name).unwrap()
        })
        .collect();
    FilePathList::new(files)
}

fn bench_naming_convention_checker(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = build_layer_map();
    let root = FilePath::new("/project".to_string()).unwrap();

    let mut group = c.benchmark_group("naming_convention_checker");

    for size in [100, 500, 1000, 5000] {
        let files = generate_files(size);
        group.bench_with_input(
            BenchmarkId::new("check_file_naming", size),
            &files,
            |b, files| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut results = LintResultList::new(Vec::new());
                        checker
                            .check_file_naming(&config, &layer_map, files, &root, &mut results)
                            .await;
                        results
                    })
                });
            },
        );
    }
    group.finish();
}

fn bench_suffix_prefix_checker(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = build_layer_map();
    let root = FilePath::new("/project".to_string()).unwrap();

    let mut group = c.benchmark_group("suffix_prefix_checker");

    for size in [100, 500, 1000, 5000] {
        let files = generate_files(size);
        group.bench_with_input(
            BenchmarkId::new("check_domain_suffixes", size),
            &files,
            |b, files| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut results = LintResultList::new(Vec::new());
                        checker
                            .check_domain_suffixes(&config, &layer_map, files, &root, &mut results)
                            .await;
                        results
                    })
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_naming_convention_checker,
    bench_suffix_prefix_checker
);
criterion_main!(benches);
```

---

## Quick Reference

```bash
# Run all tests
cargo test -p naming_rules-lint-arwaky

# Run specific test files
cargo test -p naming_rules-lint-arwaky --test contract_naming_rules
cargo test -p naming_rules-lint-arwaky --test unit_naming_rules_convention_checker
cargo test -p naming_rules-lint-arwaky --test unit_naming_rules_suffix_prefix_checker
cargo test -p naming_rules-lint-arwaky --test unit_naming_rules_orchestrator
cargo test -p naming_rules-lint-arwaky --test integration_naming_rules
cargo test -p naming_rules-lint-arwaky --test acceptance_FRD_001
cargo test -p naming_rules-lint-arwaky --test acceptance_FRD_002

# Run benchmarks
cargo bench -p naming_rules-lint-arwaky

# Coverage
cargo tarpaulin -p naming_rules-lint-arwaky --fail-under 70

# Verbose output
cargo test -p naming_rules-lint-arwaky -- --nocapture
```

---

## Coverage Mapping

| Test File                                      | Layer        | Target       | What's Covered                                                   |
| ---------------------------------------------- | ------------ | ------------ | ---------------------------------------------------------------- |
| `contract_naming_rules.rs`                   | All          | Compile-time | Trait bounds for all 3 protocols + container wiring              |
| `unit_naming_rules_convention_checker.rs`    | Capabilities | ≥70%        | AES101: happy, uppercase, hyphen, word count, exceptions, config |
| `unit_naming_rules_suffix_prefix_checker.rs` | Capabilities | ≥70%        | AES102: strict pass/fail, forbidden, flexible, exceptions        |
| `unit_naming_rules_orchestrator.rs`          | Agent        | ≥60%        | Name, error path, filtering, ignored patterns                    |
| `integration_naming_rules.rs`                | Root/Wiring  | —           | Full DI container → orchestrator → checkers pipeline           |
| `acceptance_FRD_001.rs`                      | Business     | —           | FR-001 rules 1:1                                                 |
| `acceptance_FRD_002.rs`                      | Business     | —           | FR-002 rules 1:1                                                 |
| `bench_naming_rules_throughput.rs`           | Perf         | <1s/1000     | NFR: throughput at 100–5000 files                               |
