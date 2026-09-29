// Unit tests for is_path_alive path reachability matching — helper on ContractOrphanAnalyzer
use orphan_rules_lint_arwaky::capabilities_orphan_contract_analyzer::is_path_alive;
use shared::common::taxonomy_path_vo::FilePath;
use shared::quality_rules::taxonomy_quality_rules_vo::ReachabilityResult;
use std::collections::HashSet;

fn alive(paths: &[&str]) -> ReachabilityResult {
    ReachabilityResult::new(
        paths
            .iter()
            .filter_map(|p| FilePath::new(p.to_string()).ok())
            .collect::<HashSet<_>>(),
    )
}

#[test]
fn exact_workspace_relative_match_is_alive() {
    let set = alive(&["crates/a/src/capabilities_foo.rs"]);
    assert!(is_path_alive("crates/a/src/capabilities_foo.rs", &set));
}

#[test]
fn suffix_match_at_separator_boundary_is_alive() {
    // Deeper absolute path still resolves to the same relative file.
    let set = alive(&["crates/a/src/capabilities_foo.rs"]);
    assert!(is_path_alive("a/src/capabilities_foo.rs", &set));
}

#[test]
fn partial_stem_does_not_false_match() {
    // `xcontract_foo.rs` must NOT match `contract_foo.rs`.
    let set = alive(&["crates/a/src/contract_foo.rs"]);
    assert!(!is_path_alive("crates/a/src/xcontract_foo.rs", &set));
    assert!(!is_path_alive("contract_foo.rs", &set));
}

#[test]
fn same_basename_in_other_module_does_not_false_match() {
    // A `lib.rs`/`mod.rs` in a different module must not validate an
    // unrelated file with the same basename.
    let set = alive(&["crates/a/src/lib.rs"]);
    assert!(!is_path_alive("crates/b/src/lib.rs", &set));
    assert!(!is_path_alive("lib.rs", &set));
}

#[test]
fn leading_dot_slash_and_backslashes_are_normalized() {
    let set = alive(&["crates/a/src/contract_foo.rs"]);
    assert!(is_path_alive("./crates/a/src/contract_foo.rs", &set));
    assert!(is_path_alive("crates\\a\\src\\contract_foo.rs", &set));
}
