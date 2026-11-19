// Acceptance tests — AES504: Utility orphan detection.
use orphan_rules_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_orphan_rules::IUtilityOrphanProtocol;
use shared_quality_rules::taxonomy_quality_rules_vo::{InboundLinkMap, ReachabilityResult};
use std::collections::{HashMap, HashSet};

fn empty_reachability() -> ReachabilityResult {
    ReachabilityResult::new(HashSet::new())
}

fn reachable_for(fp: &FilePath) -> ReachabilityResult {
    ReachabilityResult::new(HashSet::from([fp.clone()]))
}

fn utility_analyzer() -> UtilityOrphanAnalyzer {
    UtilityOrphanAnalyzer::new()
}

#[test]
fn aes504_utility_not_reachable_no_consumer_is_orphan() {
    let analyzer = utility_analyzer();
    let fp = FilePath::new("crates/shared/src/util_foo_helper.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub fn helper() -> i32 { 42 }\n".to_string(),
    );
    // No consumer layer file imports util_foo_helper
    content_map.insert(
        "crates/shared/src/other.rs".to_string(),
        "fn unrelated() {}".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/shared/src/other.rs".to_string(),
    ];

    let result = analyzer.is_utility_orphan(
        &fp,
        &root,
        &all_files,
        &InboundLinkMap::new(HashMap::new()),
        &content_map,
        &empty_reachability(),
    );
    assert!(
        result.is_orphan,
        "utility with no reachability and no consumer importer should be orphan"
    );
    assert_eq!(result.severity, Severity::MEDIUM);
    assert_eq!(result.subtype, "UTILITY_ORPHAN");
    assert_eq!(
        result.message,
        "'util_foo_helper' unreachable; no consumer."
    );
    assert!(result.message.chars().count() <= 72);
    assert!(!result.message.starts_with("AES504"));
    assert!(!result.message.contains("WHY:"));
    assert!(!result.message.contains("FIX:"));
    assert!(result.message.ends_with('.'));
    assert!(!result.why.is_empty());
    assert!(!result.fix.is_empty());
}

#[test]
fn aes504_utility_reachable_with_consumer_is_not_orphan() {
    let analyzer = utility_analyzer();
    let fp = FilePath::new("crates/shared/src/util_foo_helper.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub fn helper() -> i32 { 42 }\n".to_string(),
    );
    // Consumer surface imports the utility module
    content_map.insert(
        "crates/tui/src/surface_main.rs".to_string(),
        "use util_foo_helper::helper;\nfn main() { helper(); }\n".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/tui/src/surface_main.rs".to_string(),
    ];
    let mut inbound = HashMap::new();
    inbound.insert(
        fp.value().to_string(),
        vec!["crates/tui/src/surface_main.rs".to_string()],
    );

    let result = analyzer.is_utility_orphan(
        &fp,
        &root,
        &all_files,
        &InboundLinkMap::new(inbound),
        &content_map,
        &reachable_for(&fp),
    );
    assert!(
        !result.is_orphan,
        "utility reachable with consumer importer should NOT be orphan: {}",
        result.message
    );
    assert_eq!(result.subtype, "");
    assert_eq!(result.message, "");
    assert_eq!(result.severity, Severity::LOW);
    assert!(result.message.chars().count() <= 72);
    assert!(result.fix.is_empty());
}

#[test]
fn aes504_utility_not_reachable_imported_by_utility_only_is_orphan() {
    let analyzer = utility_analyzer();
    let fp = FilePath::new("crates/shared/src/util_foo_helper.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub fn helper() -> i32 { 42 }\n".to_string(),
    );
    // Only another utility file imports it — not a consumer
    content_map.insert(
        "crates/shared/src/util_bar_helper.rs".to_string(),
        "use util_foo_helper::helper;\n".to_string(),
    );
    let all_files = vec![
        fp.value().to_string(),
        "crates/shared/src/util_bar_helper.rs".to_string(),
    ];
    let mut inbound = HashMap::new();
    inbound.insert(
        fp.value().to_string(),
        vec!["crates/shared/src/util_bar_helper.rs".to_string()],
    );

    let result = analyzer.is_utility_orphan(
        &fp,
        &root,
        &all_files,
        &InboundLinkMap::new(inbound),
        &content_map,
        &empty_reachability(),
    );
    assert!(
        result.is_orphan,
        "utility imported only by another utility (not consumer) should be orphan"
    );
    assert_eq!(result.severity, Severity::MEDIUM);
    assert_eq!(result.subtype, "UTILITY_ORPHAN");
    assert_eq!(
        result.message,
        "'util_foo_helper' unreachable; no consumer (only by util_bar_helper)."
    );
    assert!(result.message.chars().count() <= 72);
    assert!(!result.why.is_empty());
    assert!(!result.fix.is_empty());
}

#[test]
fn aes504_empty_content_is_not_orphan() {
    let analyzer = utility_analyzer();
    let fp = FilePath::new("crates/shared/src/util_foo_helper.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let content_map: HashMap<String, String> = HashMap::new();

    let result = analyzer.is_utility_orphan(
        &fp,
        &root,
        &[],
        &InboundLinkMap::new(HashMap::new()),
        &content_map,
        &empty_reachability(),
    );
    assert!(!result.is_orphan, "Empty content should not be flagged");
    assert_eq!(result.severity, Severity::LOW);
    assert!(result.message.chars().count() <= 72);
}

#[test]
fn aes504_utility_reachable_no_consumer_is_orphan() {
    let analyzer = utility_analyzer();
    let fp = FilePath::new("crates/shared/src/util_foo_helper.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub fn helper() -> i32 { 42 }\n".to_string(),
    );
    let all_files = vec![fp.value().to_string()];

    let result = analyzer.is_utility_orphan(
        &fp,
        &root,
        &all_files,
        &InboundLinkMap::new(HashMap::new()),
        &content_map,
        &reachable_for(&fp),
    );
    assert!(
        result.is_orphan,
        "utility reachable but with no consumer importer should be orphan"
    );
    assert_eq!(result.severity, Severity::MEDIUM);
    assert_eq!(result.subtype, "UTILITY_ORPHAN");
    assert_eq!(
        result.message,
        "'util_foo_helper' not imported by consumer layer."
    );
    assert!(result.message.chars().count() <= 72);
    assert!(!result.why.is_empty());
    assert!(!result.fix.is_empty());
}

#[test]
fn aes504_severity_is_medium() {
    let analyzer = utility_analyzer();
    let fp = FilePath::new("crates/shared/src/util_foo_helper.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub fn helper() -> i32 { 42 }\n".to_string(),
    );
    let all_files = vec![fp.value().to_string()];

    let result = analyzer.is_utility_orphan(
        &fp,
        &root,
        &all_files,
        &InboundLinkMap::new(HashMap::new()),
        &content_map,
        &empty_reachability(),
    );
    assert_eq!(result.severity, Severity::MEDIUM);
    assert_eq!(result.subtype, "UTILITY_ORPHAN");
    assert_eq!(
        result.message,
        "'util_foo_helper' unreachable; no consumer."
    );
    assert!(result.message.chars().count() <= 72);
    assert!(!result.why.is_empty());
    assert!(!result.fix.is_empty());
}

#[test]
fn aes504_message_is_one_sentence_no_prefix() {
    let analyzer = utility_analyzer();
    let fp = FilePath::new("crates/shared/src/util_foo_helper.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "pub fn helper() -> i32 { 42 }\n".to_string(),
    );
    let all_files = vec![fp.value().to_string()];

    let result = analyzer.is_utility_orphan(
        &fp,
        &root,
        &all_files,
        &InboundLinkMap::new(HashMap::new()),
        &content_map,
        &empty_reachability(),
    );
    assert!(!result.message.starts_with("AES504"));
    assert!(!result.message.contains("WHY:"));
    assert!(!result.message.contains("FIX:"));
    assert!(result.message.ends_with('.'));
    assert!(result.message.chars().count() <= 72);
}

#[test]
fn aes504_is_module_imported_typescript_relative_and_quotes() {
    let mod_name = "contract_client_aggregate";

    // Single quotes with relative paths
    let single_rel = "import { foo } from './contract_client_aggregate';";
    assert!(UtilityOrphanAnalyzer::is_module_imported(
        "consumer.ts",
        single_rel,
        mod_name
    ));

    let single_rel_parent = "import { foo } from '../client/contract_client_aggregate';";
    assert!(UtilityOrphanAnalyzer::is_module_imported(
        "consumer.ts",
        single_rel_parent,
        mod_name
    ));

    // Double quotes with relative paths
    let double_rel = "import { foo } from \"./contract_client_aggregate\";";
    assert!(UtilityOrphanAnalyzer::is_module_imported(
        "consumer.ts",
        double_rel,
        mod_name
    ));

    let double_rel_parent = "import type { Foo } from \"../../client/contract_client_aggregate\";";
    assert!(UtilityOrphanAnalyzer::is_module_imported(
        "consumer.ts",
        double_rel_parent,
        mod_name
    ));

    // Backticks with relative paths
    let backtick_rel = "import { foo } from `./contract_client_aggregate`;";
    assert!(UtilityOrphanAnalyzer::is_module_imported(
        "consumer.ts",
        backtick_rel,
        mod_name
    ));

    // Bare imports
    let bare_single = "import { foo } from 'contract_client_aggregate';";
    assert!(UtilityOrphanAnalyzer::is_module_imported(
        "consumer.ts",
        bare_single,
        mod_name
    ));

    // Rust use and Python from
    let rust_use = "use contract_client_aggregate::foo;";
    assert!(UtilityOrphanAnalyzer::is_module_imported(
        "consumer.rs",
        rust_use,
        mod_name
    ));

    let py_from = "from contract_client_aggregate import foo";
    assert!(UtilityOrphanAnalyzer::is_module_imported(
        "consumer.py",
        py_from,
        mod_name
    ));

    // Negative cases: unrelated module
    let unrelated = "import { foo } from './contract_other_aggregate';";
    assert!(!UtilityOrphanAnalyzer::is_module_imported(
        "consumer.ts",
        unrelated,
        mod_name
    ));
}

#[test]
fn aes504_utility_rescued_by_index_ts_barrel() {
    let analyzer = utility_analyzer();
    let fp =
        FilePath::new("packages/shared/src/utils/utility_string_helper.ts".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let mut content_map = HashMap::new();
    content_map.insert(
        fp.value().to_string(),
        "export function helper(): string { return 'ok'; }\n".to_string(),
    );
    let all_files = vec![fp.value().to_string()];

    // index.ts in same directory is imported by a surface file
    let mut inbound = HashMap::new();
    inbound.insert(
        "packages/shared/src/utils/index.ts".to_string(),
        vec!["packages/cli/src/surface_main.ts".to_string()],
    );

    let result = analyzer.is_utility_orphan(
        &fp,
        &root,
        &all_files,
        &InboundLinkMap::new(inbound),
        &content_map,
        &reachable_for(&fp),
    );
    assert!(
        !result.is_orphan,
        "utility reachable and re-exported by index.ts with consumer importer should NOT be orphan"
    );
}
