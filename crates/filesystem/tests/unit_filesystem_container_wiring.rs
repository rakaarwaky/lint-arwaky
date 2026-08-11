// Unit tests — utility_container_wiring (DI-aware wiring, issues #191-193).
//
// Covers the three pure, stateless functions added in this module:
//   - path_to_relative           (absolute → workspace-relative path conversion)
//   - add_impl_bridge_edges      (contract → capabilities synthetic edges)
//   - add_container_wiring_edges (container → wired-service synthetic edges)

use filesystem_lint_arwaky::utility_container_wiring::{
    add_container_wiring_edges, add_impl_bridge_edges, path_to_relative,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ─── path_to_relative ──────────────────────────────────────

#[test]
fn path_to_relative_strips_root_prefix() {
    let root = Path::new("/workspace");
    let path = Path::new("/workspace/crates/foo/src/lib.rs");
    assert_eq!(
        path_to_relative(path, root),
        "crates/foo/src/lib.rs".to_string()
    );
}

#[test]
fn path_to_relative_falls_back_to_full_path_when_not_under_root() {
    let root = Path::new("/workspace");
    let path = Path::new("/elsewhere/crates/foo/src/lib.rs");
    assert_eq!(
        path_to_relative(path, root),
        "/elsewhere/crates/foo/src/lib.rs".to_string()
    );
}

#[test]
fn path_to_relative_root_equal_to_path_returns_empty() {
    let root = Path::new("/workspace/crates/foo");
    let path = Path::new("/workspace/crates/foo");
    assert_eq!(path_to_relative(path, root), "".to_string());
}

// ─── add_impl_bridge_edges ─────────────────────────────────

#[test]
fn add_impl_bridge_edges_wires_contract_to_single_implementor() {
    let root = Path::new("/workspace");
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "IFooProtocol".to_string(),
        vec![PathBuf::from("/workspace/src/contract_foo_protocol.rs")],
    );
    let mut implementations: HashMap<String, Vec<PathBuf>> = HashMap::new();
    implementations.insert(
        "IFooProtocol".to_string(),
        vec![PathBuf::from("/workspace/src/capabilities_foo.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_impl_bridge_edges(root, &symbol_definitions, &implementations, &mut forward);

    let edges = forward
        .get("src/contract_foo_protocol.rs")
        .expect("contract file should have outgoing edges");
    assert_eq!(edges, &vec!["src/capabilities_foo.rs".to_string()]);
}

#[test]
fn add_impl_bridge_edges_wires_multiple_implementors() {
    let root = Path::new("/workspace");
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "IProto".to_string(),
        vec![PathBuf::from("/workspace/src/contract_proto.rs")],
    );
    let mut implementations: HashMap<String, Vec<PathBuf>> = HashMap::new();
    implementations.insert(
        "IProto".to_string(),
        vec![
            PathBuf::from("/workspace/src/capabilities_impl_a.rs"),
            PathBuf::from("/workspace/src/capabilities_impl_b.rs"),
        ],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_impl_bridge_edges(root, &symbol_definitions, &implementations, &mut forward);

    let edges = forward.get("src/contract_proto.rs").unwrap();
    assert!(edges.contains(&"src/capabilities_impl_a.rs".to_string()));
    assert!(edges.contains(&"src/capabilities_impl_b.rs".to_string()));
    assert_eq!(edges.len(), 2);
}

#[test]
fn add_impl_bridge_edges_is_noop_when_trait_has_no_definition() {
    // The implementations map references a trait that has no entry in
    // symbol_definitions (e.g. an external/unresolved trait) — no edge is added.
    let root = Path::new("/workspace");
    let symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut implementations: HashMap<String, Vec<PathBuf>> = HashMap::new();
    implementations.insert(
        "UnknownTrait".to_string(),
        vec![PathBuf::from("/workspace/src/capabilities_foo.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_impl_bridge_edges(root, &symbol_definitions, &implementations, &mut forward);

    assert!(
        forward.is_empty(),
        "No edges should be added when the trait is undefined: {:?}",
        forward
    );
}

#[test]
fn add_impl_bridge_edges_skips_self_implementation() {
    // A file that both defines the trait and "implements" it (e.g. a default
    // impl in the same file as the trait declaration) must not get a self-loop.
    let root = Path::new("/workspace");
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "IProto".to_string(),
        vec![PathBuf::from("/workspace/src/contract_proto.rs")],
    );
    let mut implementations: HashMap<String, Vec<PathBuf>> = HashMap::new();
    implementations.insert(
        "IProto".to_string(),
        vec![PathBuf::from("/workspace/src/contract_proto.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_impl_bridge_edges(root, &symbol_definitions, &implementations, &mut forward);

    assert!(
        forward.is_empty(),
        "Self-implementation must not create a self-loop edge: {:?}",
        forward
    );
}

#[test]
fn add_impl_bridge_edges_appends_to_existing_forward_entries() {
    // Pre-existing static-import edges for the contract file must be preserved,
    // not overwritten, when the impl-bridge edge is appended.
    let root = Path::new("/workspace");
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "IProto".to_string(),
        vec![PathBuf::from("/workspace/src/contract_proto.rs")],
    );
    let mut implementations: HashMap<String, Vec<PathBuf>> = HashMap::new();
    implementations.insert(
        "IProto".to_string(),
        vec![PathBuf::from("/workspace/src/capabilities_impl.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();
    forward.insert(
        "src/contract_proto.rs".to_string(),
        vec!["src/other_existing_edge.rs".to_string()],
    );

    add_impl_bridge_edges(root, &symbol_definitions, &implementations, &mut forward);

    let edges = forward.get("src/contract_proto.rs").unwrap();
    assert!(edges.contains(&"src/other_existing_edge.rs".to_string()));
    assert!(edges.contains(&"src/capabilities_impl.rs".to_string()));
    assert_eq!(edges.len(), 2);
}

#[test]
fn add_impl_bridge_edges_wires_across_multiple_defining_files() {
    // If a trait name is (unusually) defined in more than one file, every
    // defining file should be wired to every implementor.
    let root = Path::new("/workspace");
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "IProto".to_string(),
        vec![
            PathBuf::from("/workspace/src/contract_a.rs"),
            PathBuf::from("/workspace/src/contract_b.rs"),
        ],
    );
    let mut implementations: HashMap<String, Vec<PathBuf>> = HashMap::new();
    implementations.insert(
        "IProto".to_string(),
        vec![PathBuf::from("/workspace/src/capabilities_impl.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_impl_bridge_edges(root, &symbol_definitions, &implementations, &mut forward);

    assert_eq!(
        forward.get("src/contract_a.rs").unwrap(),
        &vec!["src/capabilities_impl.rs".to_string()]
    );
    assert_eq!(
        forward.get("src/contract_b.rs").unwrap(),
        &vec!["src/capabilities_impl.rs".to_string()]
    );
}

// ─── add_container_wiring_edges ────────────────────────────

#[test]
fn add_container_wiring_edges_wires_identifier_to_defining_file() {
    let root = Path::new("/workspace");
    let all_files = vec!["src/root_calc_container.rs".to_string()];
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "AdditionAnalyzer".to_string(),
        vec![PathBuf::from("/workspace/src/capability_calc_addition.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_container_wiring_edges(
        &all_files,
        root,
        &symbol_definitions,
        |_p: &Path| vec!["AdditionAnalyzer".to_string()],
        &mut forward,
    );

    assert_eq!(
        forward.get("src/root_calc_container.rs").unwrap(),
        &vec!["src/capability_calc_addition.rs".to_string()]
    );
}

#[test]
fn add_container_wiring_edges_skips_files_without_container_in_name() {
    let root = Path::new("/workspace");
    let all_files = vec!["src/agent_calc_orchestrator.rs".to_string()];
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "AdditionAnalyzer".to_string(),
        vec![PathBuf::from("/workspace/src/capability_calc_addition.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_container_wiring_edges(
        &all_files,
        root,
        &symbol_definitions,
        |_p: &Path| vec!["AdditionAnalyzer".to_string()],
        &mut forward,
    );

    assert!(
        forward.is_empty(),
        "Non-container files must not get synthetic wiring edges: {:?}",
        forward
    );
}

#[test]
fn add_container_wiring_edges_skips_unknown_identifiers() {
    let root = Path::new("/workspace");
    let all_files = vec!["src/root_calc_container.rs".to_string()];
    // No symbol table entry for "UnknownType".
    let symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_container_wiring_edges(
        &all_files,
        root,
        &symbol_definitions,
        |_p: &Path| vec!["UnknownType".to_string()],
        &mut forward,
    );

    assert!(forward.is_empty());
}

#[test]
fn add_container_wiring_edges_skips_self_reference() {
    // A container file that defines a symbol itself (e.g. its own struct)
    // must not produce a self-loop edge.
    let root = Path::new("/workspace");
    let all_files = vec!["src/root_calc_container.rs".to_string()];
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "CalculatorContainer".to_string(),
        vec![PathBuf::from("/workspace/src/root_calc_container.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_container_wiring_edges(
        &all_files,
        root,
        &symbol_definitions,
        |_p: &Path| vec!["CalculatorContainer".to_string()],
        &mut forward,
    );

    assert!(
        forward.is_empty(),
        "Self-defined identifiers must not create a self-loop: {:?}",
        forward
    );
}

#[test]
fn add_container_wiring_edges_dedupes_repeated_identifiers() {
    // The same identifier appearing multiple times in the container's used
    // identifiers must only produce a single edge to the defining file.
    let root = Path::new("/workspace");
    let all_files = vec!["src/root_calc_container.rs".to_string()];
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "AdditionAnalyzer".to_string(),
        vec![PathBuf::from("/workspace/src/capability_calc_addition.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_container_wiring_edges(
        &all_files,
        root,
        &symbol_definitions,
        |_p: &Path| {
            vec![
                "AdditionAnalyzer".to_string(),
                "AdditionAnalyzer".to_string(),
            ]
        },
        &mut forward,
    );

    let edges = forward.get("src/root_calc_container.rs").unwrap();
    assert_eq!(
        edges,
        &vec!["src/capability_calc_addition.rs".to_string()],
        "Repeated identifiers must dedupe to a single edge"
    );
}

#[test]
fn add_container_wiring_edges_wires_multiple_containers_independently() {
    let root = Path::new("/workspace");
    let all_files = vec![
        "src/root_a_container.rs".to_string(),
        "src/root_b_container.rs".to_string(),
    ];
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "ServiceA".to_string(),
        vec![PathBuf::from("/workspace/src/capability_a.rs")],
    );
    symbol_definitions.insert(
        "ServiceB".to_string(),
        vec![PathBuf::from("/workspace/src/capability_b.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_container_wiring_edges(
        &all_files,
        root,
        &symbol_definitions,
        |p: &Path| {
            if p.ends_with("root_a_container.rs") {
                vec!["ServiceA".to_string()]
            } else {
                vec!["ServiceB".to_string()]
            }
        },
        &mut forward,
    );

    assert_eq!(
        forward.get("src/root_a_container.rs").unwrap(),
        &vec!["src/capability_a.rs".to_string()]
    );
    assert_eq!(
        forward.get("src/root_b_container.rs").unwrap(),
        &vec!["src/capability_b.rs".to_string()]
    );
}

#[test]
fn add_container_wiring_edges_matches_substring_container_pattern() {
    // The filter is `contains("_container")`, not an exact filename check —
    // any file with "_container" anywhere in its relative path qualifies.
    let root = Path::new("/workspace");
    let all_files = vec!["src/nested/module_container_extra.rs".to_string()];
    let mut symbol_definitions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    symbol_definitions.insert(
        "WiredService".to_string(),
        vec![PathBuf::from("/workspace/src/capability_service.rs")],
    );
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();

    add_container_wiring_edges(
        &all_files,
        root,
        &symbol_definitions,
        |_p: &Path| vec!["WiredService".to_string()],
        &mut forward,
    );

    assert_eq!(
        forward.get("src/nested/module_container_extra.rs").unwrap(),
        &vec!["src/capability_service.rs".to_string()]
    );
}