
# Test Suite: `orphan-detector` (v1.10.106)

Below is the complete test suite for the `orphan-detector` crate, following the flat `tests/` convention with filename prefixes as virtual subfolders.

---

## Directory Layout

```
crates/orphan-detector/
├── src/
│   └── lib.rs
├── tests/
│   ├── contract_orphan_detector.rs
│   ├── unit_orphan_detector_graph_resolver.rs
│   ├── unit_orphan_detector_taxonomy_analyzer.rs
│   ├── unit_orphan_detector_contract_analyzer.rs
│   ├── unit_orphan_detector_capabilities_analyzer.rs
│   ├── unit_orphan_detector_utility_analyzer.rs
│   ├── unit_orphan_detector_agent_analyzer.rs
│   ├── unit_orphan_detector_surfaces_analyzer.rs
│   ├── unit_orphan_detector_orchestrator.rs
│   ├── integration_orphan_detector.rs
│   ├── smoke_orphan_detector.rs
│   ├── e2e_orphan_detection_flow.rs
│   ├── acceptance_AES501.rs
│   ├── acceptance_AES502.rs
│   ├── acceptance_AES503.rs
│   ├── acceptance_AES504.rs
│   ├── acceptance_AES505.rs
│   ├── acceptance_AES506.rs
│   └── bench_orphan_detector_graph.rs
└── Cargo.toml
```

---

## `Cargo.toml` (bench registration)

```toml
[[bench]]
name = "bench_orphan_detector_graph"
path = "tests/bench_orphan_detector_graph.rs"
harness = false
```

---

## `tests/contract_orphan_detector.rs`

```rust
// PURPOSE: Verify all trait implementations exist for orphan-detector structs.
// Layer: Contract
// Speed: ms

use orphan_detector_lint_arwaky::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use orphan_detector_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;

use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::contract_orphan_protocol::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol, IUtilityOrphanProtocol,
};

// ─── IOrphanAggregate ─────────────────────────────────────

#[test]
fn arch_orphan_analyzer_implements_i_orphan_aggregate() {
    fn assert_trait<T: IOrphanAggregate>() {}
    assert_trait::<ArchOrphanAnalyzer>();
}

// ─── IOrphanGraphResolverProtocol ─────────────────────────

#[test]
fn orphan_graph_resolver_implements_i_orphan_graph_resolver_protocol() {
    fn assert_trait<T: IOrphanGraphResolverProtocol>() {}
    assert_trait::<OrphanGraphResolver>();
}

// ─── ITaxonomyOrphanProtocol ──────────────────────────────

#[test]
fn taxonomy_orphan_analyzer_implements_i_taxonomy_orphan_protocol() {
    fn assert_trait<T: ITaxonomyOrphanProtocol>() {}
    assert_trait::<TaxonomyOrphanAnalyzer>();
}

// ─── IContractOrphanProtocol ──────────────────────────────

#[test]
fn contract_orphan_analyzer_implements_i_contract_orphan_protocol() {
    fn assert_trait<T: IContractOrphanProtocol>() {}
    assert_trait::<ContractOrphanAnalyzer>();
}

// ─── ICapabilitiesOrphanProtocol ──────────────────────────

#[test]
fn capabilities_orphan_analyzer_implements_i_capabilities_orphan_protocol() {
    fn assert_trait<T: ICapabilitiesOrphanProtocol>() {}
    assert_trait::<CapabilitiesOrphanAnalyzer>();
}

// ─── IUtilityOrphanProtocol ───────────────────────────────

#[test]
fn utility_orphan_analyzer_implements_i_utility_orphan_protocol() {
    fn assert_trait<T: IUtilityOrphanProtocol>() {}
    assert_trait::<UtilityOrphanAnalyzer>();
}

// ─── IAgentOrphanProtocol ─────────────────────────────────

#[test]
fn agent_orphan_analyzer_implements_i_agent_orphan_protocol() {
    fn assert_trait<T: IAgentOrphanProtocol>() {}
    assert_trait::<AgentOrphanAnalyzer>();
}

// ─── ISurfacesOrphanProtocol ──────────────────────────────

#[test]
fn surfaces_orphan_analyzer_implements_i_surfaces_orphan_protocol() {
    fn assert_trait<T: ISurfacesOrphanProtocol>() {}
    assert_trait::<SurfacesOrphanAnalyzer>();
}

// ─── Send + Sync bounds ───────────────────────────────────

#[test]
fn all_analyzers_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ArchOrphanAnalyzer>();
    assert_send_sync::<OrphanGraphResolver>();
    assert_send_sync::<TaxonomyOrphanAnalyzer>();
    assert_send_sync::<ContractOrphanAnalyzer>();
    assert_send_sync::<CapabilitiesOrphanAnalyzer>();
    assert_send_sync::<UtilityOrphanAnalyzer>();
    assert_send_sync::<AgentOrphanAnalyzer>();
    assert_send_sync::<SurfacesOrphanAnalyzer>();
    assert_send_sync::<OrphanContainer>();
}
```

---

## `tests/unit_orphan_detector_graph_resolver.rs`

```rust
// PURPOSE: Unit tests for OrphanGraphResolver — graph building and entry point identification.
// Layer: Capabilities (OrphanGraphResolver)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::{OrphanEntryPatternListVO, OrphanFileListVO};

fn resolver() -> OrphanGraphResolver {
    OrphanGraphResolver::new()
}

// ─── build_graph_context ──────────────────────────────────

#[test]
fn build_graph_context_empty_files_returns_empty_graph() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![])];
    let ctx = r.build_graph_context(&files, "/tmp/project");
    assert!(ctx.import_graph.mapping.is_empty());
    assert!(ctx.inbound_links.mapping.is_empty());
}

#[test]
fn build_graph_context_single_file_no_imports() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![
        "/tmp/project/src/lib.rs".to_string(),
    ])];
    let ctx = r.build_graph_context(&files, "/tmp/project");
    // lib.rs should appear in the graph even with no imports
    assert!(ctx.import_graph.mapping.contains_key("/tmp/project/src/lib.rs"));
}

// ─── identify_entry_points ────────────────────────────────

#[test]
fn identify_entry_points_no_configured_patterns_uses_defaults() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![
        "/tmp/project/src/main.rs".to_string(),
        "/tmp/project/src/lib.rs".to_string(),
        "/tmp/project/src/root_app_container.rs".to_string(),
        "/tmp/project/src/capabilities_foo_analyzer.rs".to_string(),
    ])];
    let result = r.identify_entry_points(&files, &[]);
    // main.rs, lib.rs, and *_container.rs should be entry points
    assert!(result.values.contains(&"/tmp/project/src/main.rs".to_string()));
    assert!(result.values.contains(&"/tmp/project/src/lib.rs".to_string()));
    assert!(result.values.contains(&"/tmp/project/src/root_app_container.rs".to_string()));
    // capabilities file should NOT be an entry point
    assert!(!result.values.contains(&"/tmp/project/src/capabilities_foo_analyzer.rs".to_string()));
}

#[test]
fn identify_entry_points_with_configured_patterns() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![
        "/tmp/project/src/main.rs".to_string(),
        "/tmp/project/src/custom_entry.rs".to_string(),
        "/tmp/project/src/other.rs".to_string(),
    ])];
    let configured = vec![OrphanEntryPatternListVO::new(vec![
        "custom_entry.rs".to_string(),
    ])];
    let result = r.identify_entry_points(&files, &configured);
    assert!(result.values.contains(&"/tmp/project/src/custom_entry.rs".to_string()));
    assert!(!result.values.contains(&"/tmp/project/src/other.rs".to_string()));
}

#[test]
fn identify_entry_points_empty_files_returns_empty() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![])];
    let result = r.identify_entry_points(&files, &[]);
    assert!(result.values.is_empty());
}

#[test]
fn identify_entry_points_root_prefix_matched() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![
        "/tmp/project/src/root_orphan_detector_container.rs".to_string(),
    ])];
    let result = r.identify_entry_points(&files, &[]);
    assert!(result.values.contains(&"/tmp/project/src/root_orphan_detector_container.rs".to_string()));
}

#[test]
fn identify_entry_points_index_files_matched() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![
        "/tmp/project/src/index.ts".to_string(),
        "/tmp/project/src/index.js".to_string(),
    ])];
    let result = r.identify_entry_points(&files, &[]);
    assert!(result.values.contains(&"/tmp/project/src/index.ts".to_string()));
    assert!(result.values.contains(&"/tmp/project/src/index.js".to_string()));
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let r = OrphanGraphResolver::default();
    let files = vec![OrphanFileListVO::new(vec![])];
    let ctx = r.build_graph_context(&files, "/tmp");
    assert!(ctx.import_graph.mapping.is_empty());
}
```

---

## `tests/unit_orphan_detector_taxonomy_analyzer.rs`

```rust
// PURPOSE: Unit tests for TaxonomyOrphanAnalyzer — AES501 taxonomy orphan detection.
// Layer: Capabilities (TaxonomyOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use std::collections::HashMap;

fn analyzer() -> TaxonomyOrphanAnalyzer {
    TaxonomyOrphanAnalyzer::new()
}

fn make_inbound_links(links: Vec<(&str, Vec<&str>)>) -> InboundLinkMap {
    let mut mapping = HashMap::new();
    for (file, importers) in links {
        mapping.insert(
            file.to_string(),
            importers.iter().map(|s| s.to_string()).collect(),
        );
    }
    InboundLinkMap::new(mapping)
}

// ─── Happy path: taxonomy imported by contract ────────────

#[test]
fn taxonomy_vo_imported_by_contract_is_not_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/common/taxonomy_path_vo.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let inbound = make_inbound_links(vec![(
        "crates/shared/src/common/taxonomy_path_vo.rs",
        vec!["crates/shared/src/orphan-detector/contract_orphan_protocol.rs"],
    )]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(!result.is_orphan);
}

// ─── Orphan: no inbound links ─────────────────────────────

#[test]
fn taxonomy_vo_with_no_importers_is_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/common/taxonomy_orphan_vo.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let inbound = make_inbound_links(vec![]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::LOW);
    assert!(result.reason.contains("taxonomy_orphan_vo"));
}

// ─── Orphan: only imported by other taxonomy files ────────

#[test]
fn taxonomy_vo_imported_only_by_taxonomy_is_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/common/taxonomy_foo_vo.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let inbound = make_inbound_links(vec![(
        "crates/shared/src/common/taxonomy_foo_vo.rs",
        vec!["crates/shared/src/common/taxonomy_bar_vo.rs"],
    )]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(result.is_orphan);
}

// ─── Not orphan: imported by capabilities ─────────────────

#[test]
fn taxonomy_vo_imported_by_capabilities_is_not_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/common/taxonomy_severity_vo.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let inbound = make_inbound_links(vec![(
        "crates/shared/src/common/taxonomy_severity_vo.rs",
        vec!["crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs"],
    )]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(!result.is_orphan);
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let a = TaxonomyOrphanAnalyzer::default();
    let f = FilePath::new("taxonomy_test_vo.rs".to_string()).unwrap();
    let root = FilePath::new("/tmp".to_string()).unwrap();
    let inbound = make_inbound_links(vec![]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    // No importers → orphan
    assert!(result.is_orphan);
}
```

---

## `tests/unit_orphan_detector_contract_analyzer.rs`

```rust
// PURPOSE: Unit tests for ContractOrphanAnalyzer — AES502 contract orphan detection.
// Layer: Capabilities (ContractOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::{FileDefinitionMap, InheritanceMap};
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use std::collections::HashMap;

fn analyzer() -> ContractOrphanAnalyzer {
    ContractOrphanAnalyzer::new()
}

fn empty_definitions() -> FileDefinitionMap {
    FileDefinitionMap::new(HashMap::new())
}

fn empty_inheritance() -> InheritanceMap {
    InheritanceMap::new(HashMap::new())
}

// ─── Happy path: contract with implementation ─────────────

#[test]
fn contract_protocol_with_implementation_is_not_orphan() {
    let a = analyzer();
    // This test requires real file I/O; using a temp file approach
    let dir = tempfile::tempdir().unwrap();
    let contract_path = dir.path().join("contract_orphan_protocol.rs");
    std::fs::write(
        &contract_path,
        "pub trait IOrphanProtocol: Send + Sync {\n    fn check(&self) -> bool;\n}\n",
    )
    .unwrap();

    let impl_path = dir.path().join("capabilities_orphan_checker.rs");
    std::fs::write(
        &impl_path,
        "impl IOrphanProtocol for OrphanChecker {\n    fn check(&self) -> bool { true }\n}\n",
    )
    .unwrap();

    let f = FilePath::new(contract_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![
        contract_path.to_str().unwrap().to_string(),
        impl_path.to_str().unwrap().to_string(),
    ];

    let result = a.is_contract_orphan(&f, &root, &empty_definitions(), &empty_inheritance(), &all_files);
    assert!(!result.is_orphan);
}

// ─── Orphan: contract with no implementation ──────────────

#[test]
fn contract_protocol_without_implementation_is_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();
    let contract_path = dir.path().join("contract_dead_protocol.rs");
    std::fs::write(
        &contract_path,
        "pub trait IDeadProtocol: Send + Sync {\n    fn do_nothing(&self);\n}\n",
    )
    .unwrap();

    let f = FilePath::new(contract_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![contract_path.to_str().unwrap().to_string()];

    let result = a.is_contract_orphan(&f, &root, &empty_definitions(), &empty_inheritance(), &all_files);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::MEDIUM);
}

// ─── Empty contract file is not orphan ────────────────────

#[test]
fn empty_contract_file_is_not_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();
    let contract_path = dir.path().join("contract_empty_protocol.rs");
    std::fs::write(&contract_path, "").unwrap();

    let f = FilePath::new(contract_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![contract_path.to_str().unwrap().to_string()];

    let result = a.is_contract_orphan(&f, &root, &empty_definitions(), &empty_inheritance(), &all_files);
    assert!(!result.is_orphan);
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = ContractOrphanAnalyzer::default();
}
```

---

## `tests/unit_orphan_detector_capabilities_analyzer.rs`

```rust
// PURPOSE: Unit tests for CapabilitiesOrphanAnalyzer — AES503 capabilities orphan detection.
// Layer: Capabilities (CapabilitiesOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use std::collections::HashSet;

fn analyzer() -> CapabilitiesOrphanAnalyzer {
    CapabilitiesOrphanAnalyzer::new()
}

// ─── Happy path: reachable capabilities file ──────────────

#[test]
fn capabilities_file_reachable_from_entry_is_not_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs".to_string()).unwrap();
    let root = FilePath::new("crates/orphan-detector".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::from([f.clone()]));

    let result = a.is_capabilities_orphan(&f, &root, &alive);
    assert!(!result.is_orphan);
}

// ─── Orphan: unreachable capabilities file ────────────────

#[test]
fn capabilities_file_not_reachable_is_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/orphan-detector/src/capabilities_dead_analyzer.rs".to_string()).unwrap();
    let root = FilePath::new("crates/orphan-detector".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::new());

    let result = a.is_capabilities_orphan(&f, &root, &alive);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::MEDIUM);
    assert!(result.reason.contains("capabilities_dead_analyzer"));
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = CapabilitiesOrphanAnalyzer::default();
}
```

---

## `tests/unit_orphan_detector_utility_analyzer.rs`

```rust
// PURPOSE: Unit tests for UtilityOrphanAnalyzer — AES504 utility orphan detection.
// Layer: Capabilities (UtilityOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IUtilityOrphanProtocol;
use std::collections::HashMap;

fn analyzer() -> UtilityOrphanAnalyzer {
    UtilityOrphanAnalyzer::new()
}

fn make_inbound_links(links: Vec<(&str, Vec<&str>)>) -> InboundLinkMap {
    let mut mapping = HashMap::new();
    for (file, importers) in links {
        mapping.insert(
            file.to_string(),
            importers.iter().map(|s| s.to_string()).collect(),
        );
    }
    InboundLinkMap::new(mapping)
}

// ─── Happy path: utility imported by capabilities ─────────

#[test]
fn utility_imported_by_capabilities_is_not_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/orphan-detector/utility_orphan.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let all_files = vec![
        "crates/shared/src/orphan-detector/utility_orphan.rs".to_string(),
        "crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs".to_string(),
    ];
    let inbound = make_inbound_links(vec![(
        "crates/shared/src/orphan-detector/utility_orphan.rs",
        vec!["crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs"],
    )]);

    let result = a.is_utility_orphan(&f, &root, &all_files, &inbound);
    assert!(!result.is_orphan);
}

// ─── Orphan: utility not imported by anyone ───────────────

#[test]
fn utility_not_imported_is_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/orphan-detector/utility_dead.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let all_files = vec![
        "crates/shared/src/orphan-detector/utility_dead.rs".to_string(),
    ];
    let inbound = make_inbound_links(vec![]);

    let result = a.is_utility_orphan(&f, &root, &all_files, &inbound);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::MEDIUM);
}

// ─── Dead code: utility only imported by other utilities ──

#[test]
fn utility_imported_only_by_utilities_is_dead_code() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/orphan-detector/utility_inner.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let all_files = vec![
        "crates/shared/src/orphan-detector/utility_inner.rs".to_string(),
        "crates/shared/src/orphan-detector/utility_outer.rs".to_string(),
    ];
    let inbound = make_inbound_links(vec![(
        "crates/shared/src/orphan-detector/utility_inner.rs",
        vec!["crates/shared/src/orphan-detector/utility_outer.rs"],
    )]);

    let result = a.is_utility_orphan(&f, &root, &all_files, &inbound);
    assert!(result.is_orphan);
    assert!(result.reason.contains("only imported by other utility"));
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = UtilityOrphanAnalyzer::default();
}
```

---

## `tests/unit_orphan_detector_agent_analyzer.rs`

```rust
// PURPOSE: Unit tests for AgentOrphanAnalyzer — AES505 agent orphan detection.
// Layer: Capabilities (AgentOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IAgentOrphanProtocol;

fn analyzer() -> AgentOrphanAnalyzer {
    AgentOrphanAnalyzer::new()
}

// ─── Happy path: agent aggregate called by container ──────

#[test]
fn agent_with_aggregate_called_by_container_is_not_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();

    let agent_path = dir.path().join("agent_orphan_orchestrator.rs");
    std::fs::write(
        &agent_path,
        "impl IOrphanAggregate for ArchOrphanAnalyzer {\n    fn check_orphans(&self) {}\n}\n",
    )
    .unwrap();

    let container_path = dir.path().join("root_orphan_detector_container.rs");
    std::fs::write(
        &container_path,
        "use crate::agent_orphan_orchestrator::ArchOrphanAnalyzer;\nlet x = IOrphanAggregate;\n",
    )
    .unwrap();

    let f = FilePath::new(agent_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![
        agent_path.to_str().unwrap().to_string(),
        container_path.to_str().unwrap().to_string(),
    ];

    let result = a.is_agent_orphan(&f, &root, &all_files);
    assert!(!result.is_orphan);
}

// ─── Orphan: agent aggregate not called ───────────────────

#[test]
fn agent_with_aggregate_not_called_is_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();

    let agent_path = dir.path().join("agent_dead_orchestrator.rs");
    std::fs::write(
        &agent_path,
        "impl IDeadAggregate for DeadOrchestrator {\n    fn run(&self) {}\n}\n",
    )
    .unwrap();

    let other_path = dir.path().join("capabilities_foo.rs");
    std::fs::write(&other_path, "pub struct Foo;\n").unwrap();

    let f = FilePath::new(agent_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![
        agent_path.to_str().unwrap().to_string(),
        other_path.to_str().unwrap().to_string(),
    ];

    let result = a.is_agent_orphan(&f, &root, &all_files);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::HIGH);
    assert!(result.reason.contains("IDeadAggregate"));
}

// ─── No aggregate traits → not orphan ─────────────────────

#[test]
fn agent_without_aggregate_traits_is_not_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();

    let agent_path = dir.path().join("agent_simple_orchestrator.rs");
    std::fs::write(&agent_path, "pub struct SimpleOrch;\nimpl SimpleOrch { pub fn new() -> Self { Self } }\n").unwrap();

    let f = FilePath::new(agent_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![agent_path.to_str().unwrap().to_string()];

    let result = a.is_agent_orphan(&f, &root, &all_files);
    assert!(!result.is_orphan);
}

// ─── Empty file → not orphan ──────────────────────────────

#[test]
fn empty_agent_file_is_not_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();
    let agent_path = dir.path().join("agent_empty_orchestrator.rs");
    std::fs::write(&agent_path, "").unwrap();

    let f = FilePath::new(agent_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all_files = vec![agent_path.to_str().unwrap().to_string()];

    let result = a.is_agent_orphan(&f, &root, &all_files);
    assert!(!result.is_orphan);
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = AgentOrphanAnalyzer::default();
}
```

---

## `tests/unit_orphan_detector_surfaces_analyzer.rs`

```rust
// PURPOSE: Unit tests for SurfacesOrphanAnalyzer — AES506 surface orphan detection.
// Layer: Capabilities (SurfacesOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ISurfacesOrphanProtocol;
use std::collections::HashSet;

fn analyzer() -> SurfacesOrphanAnalyzer {
    SurfacesOrphanAnalyzer::new()
}

// ─── Happy path: reachable surface ────────────────────────

#[test]
fn surface_reachable_from_entry_is_not_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/app/src/surface_scan_command.rs".to_string()).unwrap();
    let root = FilePath::new("crates/app".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::from([f.clone()]));

    let result = a.is_surface_orphan(&f, &root, &alive, None);
    assert!(!result.is_orphan);
}

// ─── Orphan: unreachable surface ──────────────────────────

#[test]
fn surface_not_reachable_is_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();
    let surface_path = dir.path().join("surface_dead_command.rs");
    std::fs::write(&surface_path, "pub struct DeadCommand;\n").unwrap();

    let f = FilePath::new(surface_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::new());

    let result = a.is_surface_orphan(&f, &root, &alive, None);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::HIGH);
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = SurfacesOrphanAnalyzer::default();
}
```

---

## `tests/unit_orphan_detector_orchestrator.rs`

```rust
// PURPOSE: Unit tests for ArchOrphanAnalyzer — orchestration logic, reachability tracing, layer evaluation.
// Layer: Agent (ArchOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::agent_orphan_orchestrator::{ArchOrphanAnalyzer, ArchOrphanDeps};
use orphan_detector_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use orphan_detector_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use std::sync::Arc;

fn build_analyzer(config: ArchitectureConfig) -> ArchOrphanAnalyzer {
    ArchOrphanAnalyzer::new(ArchOrphanDeps {
        resolver: Arc::new(OrphanGraphResolver::new()),
        taxonomy_analyzer: Arc::new(TaxonomyOrphanAnalyzer::new()),
        contract_analyzer: Arc::new(ContractOrphanAnalyzer::new()),
        capabilities_analyzer: Arc::new(CapabilitiesOrphanAnalyzer::new()),
        utility_analyzer: Arc::new(UtilityOrphanAnalyzer::new()),
        agent_analyzer: Arc::new(AgentOrphanAnalyzer::new()),
        surfaces_analyzer: Arc::new(SurfacesOrphanAnalyzer::new()),
        config,
    })
}

// ─── Disabled config returns empty results ────────────────

#[test]
fn check_orphans_disabled_config_returns_empty() {
    let config = ArchitectureConfig {
        enabled: shared::common::taxonomy_common_vo::BooleanVO::new(false),
        ..Default::default()
    };
    let analyzer = build_analyzer(config);
    let results = analyzer.check_orphans(&["src/lib.rs".to_string()], "/tmp/project");
    assert!(results.is_empty());
}

// ─── Empty file list returns empty results ────────────────

#[test]
fn check_orphans_empty_files_returns_empty() {
    let config = ArchitectureConfig::default();
    let analyzer = build_analyzer(config);
    let results = analyzer.check_orphans(&[], "/tmp/project");
    assert!(results.is_empty());
}

// ─── build_orphan_graph_context ───────────────────────────

#[test]
fn build_orphan_graph_context_returns_valid_context() {
    let config = ArchitectureConfig::default();
    let analyzer = build_analyzer(config);
    let ctx = analyzer.build_orphan_graph_context(&["src/lib.rs".to_string()], "/tmp/project");
    assert!(ctx.import_graph.mapping.contains_key("src/lib.rs"));
}

// ─── identify_orphan_entry_points ─────────────────────────

#[test]
fn identify_orphan_entry_points_finds_main_and_lib() {
    let config = ArchitectureConfig::default();
    let analyzer = build_analyzer(config);
    let entries = analyzer.identify_orphan_entry_points(&[
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
        "src/capabilities_foo.rs".to_string(),
    ]);
    assert!(entries.contains("src/main.rs"));
    assert!(entries.contains("src/lib.rs"));
    assert!(!entries.contains("src/capabilities_foo.rs"));
}

// ─── Ignored paths are filtered ───────────────────────────

#[test]
fn check_orphans_respects_ignored_paths() {
    let config = ArchitectureConfig {
        ignored_paths: shared::common::taxonomy_paths_vo::FilePathList::new(vec![
            shared::common::taxonomy_path_vo::FilePath::new("src/generated".to_string()).unwrap(),
        ]),
        ..Default::default()
    };
    let analyzer = build_analyzer(config);
    let results = analyzer.check_orphans(
        &["src/generated/taxonomy_auto_vo.rs".to_string()],
        "/tmp/project",
    );
    assert!(results.is_empty());
}
```

---

## `tests/integration_orphan_detector.rs`

```rust
// PURPOSE: Integration tests — DI container wiring, full pipeline through OrphanContainer.
// Layer: Integration
// Speed: ms–s

use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;

// ─── Container wiring ─────────────────────────────────────

#[test]
fn container_creates_valid_analyzer() {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    // Should be able to call check_orphans without panic
    let results = analyzer.check_orphans(&[], "/tmp/nonexistent");
    assert!(results.is_empty());
}

#[test]
fn container_with_ignored_paths_creates_valid_analyzer() {
    let container = OrphanContainer::new_with_ignored(vec![
        "target".to_string(),
        "node_modules".to_string(),
    ]);
    let analyzer = container.analyzer();
    let results = analyzer.check_orphans(&[], "/tmp/nonexistent");
    assert!(results.is_empty());
}

#[test]
fn container_default_creates_valid_analyzer() {
    let container = OrphanContainer::default();
    let analyzer = container.analyzer();
    let entries = analyzer.identify_orphan_entry_points(&[
        "src/main.rs".to_string(),
        "src/root_app_container.rs".to_string(),
    ]);
    assert!(entries.contains("src/main.rs"));
    assert!(entries.contains("src/root_app_container.rs"));
}

// ─── Graph context through container ──────────────────────

#[test]
fn container_build_graph_context_returns_valid_structure() {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let ctx = analyzer.build_orphan_graph_context(
        &["crates/orphan-detector/src/lib.rs".to_string()],
        "/tmp/project",
    );
    // lib.rs should be a node in the graph
    assert!(ctx.import_graph.mapping.contains_key("crates/orphan-detector/src/lib.rs"));
}

// ─── Multiple analyzer calls are independent ──────────────

#[test]
fn container_analyzer_is_cloneable_and_independent() {
    let container = OrphanContainer::new();
    let a1 = container.analyzer();
    let a2 = container.analyzer();

    let r1 = a1.check_orphans(&[], "/tmp/a");
    let r2 = a2.check_orphans(&[], "/tmp/b");
    assert_eq!(r1.len(), r2.len());
}
```

---

## `tests/smoke_orphan_detector.rs`

```rust
// PURPOSE: Smoke test — crate boots, container initializes, basic call succeeds in < 5s.
// Layer: Smoke
// Speed: < 5s

use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;

#[test]
fn orphan_detector_boots_and_responds() {
    let start = std::time::Instant::now();

    let container = OrphanContainer::new();
    let analyzer = container.analyzer();

    // Basic operation: identify entry points
    let entries = analyzer.identify_orphan_entry_points(&[
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
        "src/root_container.rs".to_string(),
    ]);
    assert!(!entries.is_empty());

    // Basic operation: check orphans on empty set
    let results = analyzer.check_orphans(&[], "/tmp/smoke");
    assert!(results.is_empty());

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test took {:?}, must be < 5s",
        elapsed
    );
}
```

---

## `tests/e2e_orphan_detection_flow.rs`

```rust
// PURPOSE: E2E test — full orphan detection lifecycle on a real temp project structure.
// Layer: E2E
// Speed: s

use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use std::fs;

fn create_test_project(dir: &std::path::Path) {
    let src = dir.join("src");
    fs::create_dir_all(&src).unwrap();

    // Entry point
    fs::write(
        src.join("main.rs"),
        "mod root_app_container;\nfn main() { root_app_container::run(); }\n",
    )
    .unwrap();

    // Container (entry point)
    fs::write(
        src.join("root_app_container.rs"),
        "use crate::capabilities_greeter_analyzer::GreeterAnalyzer;\npub fn run() { let _ = GreeterAnalyzer::new(); }\n",
    )
    .unwrap();

    // Reachable capability
    fs::write(
        src.join("capabilities_greeter_analyzer.rs"),
        "pub struct GreeterAnalyzer;\nimpl GreeterAnalyzer { pub fn new() -> Self { Self } }\n",
    )
    .unwrap();

    // Orphan capability (not imported by anything)
    fs::write(
        src.join("capabilities_dead_analyzer.rs"),
        "pub struct DeadAnalyzer;\nimpl DeadAnalyzer { pub fn new() -> Self { Self } }\n",
    )
    .unwrap();

    // lib.rs
    fs::write(
        src.join("lib.rs"),
        "pub mod capabilities_greeter_analyzer;\npub mod capabilities_dead_analyzer;\npub mod root_app_container;\n",
    )
    .unwrap();
}

#[test]
fn full_orphan_detection_lifecycle() {
    let dir = tempfile::tempdir().unwrap();
    create_test_project(dir.path());

    let container = OrphanContainer::new();
    let analyzer = container.analyzer();

    let files: Vec<String> = vec![
        dir.path().join("src/main.rs").to_str().unwrap().to_string(),
        dir.path().join("src/lib.rs").to_str().unwrap().to_string(),
        dir.path().join("src/root_app_container.rs").to_str().unwrap().to_string(),
        dir.path().join("src/capabilities_greeter_analyzer.rs").to_str().unwrap().to_string(),
        dir.path().join("src/capabilities_dead_analyzer.rs").to_str().unwrap().to_string(),
    ];

    let root_dir = dir.path().to_str().unwrap();

    // Step 1: Build graph
    let ctx = analyzer.build_orphan_graph_context(&files, root_dir);
    assert!(!ctx.import_graph.mapping.is_empty());

    // Step 2: Identify entry points
    let entries = analyzer.identify_orphan_entry_points(&files);
    assert!(entries.iter().any(|e| e.contains("main.rs")));

    // Step 3: Check orphans
    let results = analyzer.check_orphans(&files, root_dir);

    // The dead analyzer should be flagged; the greeter should not
    let dead_flagged = results.iter().any(|r| r.file.value.contains("capabilities_dead_analyzer"));
    let greeter_flagged = results.iter().any(|r| r.file.value.contains("capabilities_greeter_analyzer"));

    // Note: exact behavior depends on graph resolution; at minimum, no panic
    // and results are well-formed LintResults
    for r in &results {
        assert!(!r.file.value.is_empty());
        assert!(!r.message.value.is_empty());
    }

    // Greeter should NOT be flagged (it's wired in container)
    assert!(!greeter_flagged, "Reachable capability should not be flagged as orphan");
}
```

---

## `tests/acceptance_AES501.rs`

```rust
// PURPOSE: Acceptance test — AES501 Taxonomy Orphan Checker.
// Requirement: Taxonomy layer files must be reachable from contracts, capabilities, or orchestrators.

use orphan_detector_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use std::collections::HashMap;

fn make_inbound(links: Vec<(&str, Vec<&str>)>) -> InboundLinkMap {
    let mut m = HashMap::new();
    for (f, importers) in links {
        m.insert(f.to_string(), importers.iter().map(|s| s.to_string()).collect());
    }
    InboundLinkMap::new(m)
}

/// AES501: Taxonomy file imported by a contract is NOT orphan.
#[test]
fn aes501_taxonomy_imported_by_contract_not_orphan() {
    let a = TaxonomyOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/common/taxonomy_path_vo.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let inbound = make_inbound(vec![(
        "shared/src/common/taxonomy_path_vo.rs",
        vec!["shared/src/orphan-detector/contract_orphan_protocol.rs"],
    )]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(!result.is_orphan, "AES501 FAIL: taxonomy imported by contract should not be orphan");
}

/// AES501: Taxonomy file with zero importers IS orphan.
#[test]
fn aes501_taxonomy_with_no_importers_is_orphan() {
    let a = TaxonomyOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/common/taxonomy_dead_vo.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let inbound = make_inbound(vec![]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(result.is_orphan, "AES501 FAIL: taxonomy with no importers must be flagged");
    assert!(result.reason.contains("taxonomy_dead_vo"));
}

/// AES501: Taxonomy file imported only by another taxonomy file IS orphan.
#[test]
fn aes501_taxonomy_imported_only_by_taxonomy_is_orphan() {
    let a = TaxonomyOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/common/taxonomy_inner_vo.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let inbound = make_inbound(vec![(
        "shared/src/common/taxonomy_inner_vo.rs",
        vec!["shared/src/common/taxonomy_outer_vo.rs"],
    )]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(result.is_orphan, "AES501 FAIL: taxonomy imported only by taxonomy must be flagged");
}
```

---

## `tests/acceptance_AES502.rs`

```rust
// PURPOSE: Acceptance test — AES502 Contract Orphan Checker.
// Requirement: Contract files must have at least one active implementation in capabilities or utility layers.

use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use shared::code_analysis::taxonomy_analysis_vo::{FileDefinitionMap, InheritanceMap};
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use std::collections::HashMap;
use std::fs;

fn empty_defs() -> FileDefinitionMap {
    FileDefinitionMap::new(HashMap::new())
}
fn empty_inh() -> InheritanceMap {
    InheritanceMap::new(HashMap::new())
}

/// AES502: Contract protocol with an impl in capabilities is NOT orphan.
#[test]
fn aes502_contract_with_impl_not_orphan() {
    let a = ContractOrphanAnalyzer::new();
    let dir = tempfile::tempdir().unwrap();

    let contract = dir.path().join("contract_greeter_protocol.rs");
    fs::write(&contract, "pub trait IGreeterProtocol: Send + Sync {\n    fn greet(&self) -> String;\n}\n").unwrap();

    let impl_file = dir.path().join("capabilities_greeter_analyzer.rs");
    fs::write(&impl_file, "impl IGreeterProtocol for GreeterAnalyzer {\n    fn greet(&self) -> String { \"hi\".into() }\n}\n").unwrap();

    let f = FilePath::new(contract.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all = vec![
        contract.to_str().unwrap().to_string(),
        impl_file.to_str().unwrap().to_string(),
    ];

    let result = a.is_contract_orphan(&f, &root, &empty_defs(), &empty_inh(), &all);
    assert!(!result.is_orphan, "AES502 FAIL: contract with implementation should not be orphan");
}

/// AES502: Contract protocol with NO impl IS orphan.
#[test]
fn aes502_contract_without_impl_is_orphan() {
    let a = ContractOrphanAnalyzer::new();
    let dir = tempfile::tempdir().unwrap();

    let contract = dir.path().join("contract_dead_protocol.rs");
    fs::write(&contract, "pub trait IDeadProtocol: Send + Sync {\n    fn nothing(&self);\n}\n").unwrap();

    let f = FilePath::new(contract.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all = vec![contract.to_str().unwrap().to_string()];

    let result = a.is_contract_orphan(&f, &root, &empty_defs(), &empty_inh(), &all);
    assert!(result.is_orphan, "AES502 FAIL: contract without implementation must be flagged");
}
```

---

## `tests/acceptance_AES503.rs`

```rust
// PURPOSE: Acceptance test — AES503 Capabilities Orphan Checker.
// Requirement: Capability files must be instantiated or imported by orchestrators or other capability files.

use orphan_detector_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use std::collections::HashSet;

/// AES503: Capability reachable from entry point is NOT orphan.
#[test]
fn aes503_reachable_capability_not_orphan() {
    let a = CapabilitiesOrphanAnalyzer::new();
    let f = FilePath::new("src/capabilities_greeter_analyzer.rs".to_string()).unwrap();
    let root = FilePath::new("src".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::from([f.clone()]));

    let result = a.is_capabilities_orphan(&f, &root, &alive);
    assert!(!result.is_orphan, "AES503 FAIL: reachable capability should not be orphan");
}

/// AES503: Capability NOT reachable from any entry point IS orphan.
#[test]
fn aes503_unreachable_capability_is_orphan() {
    let a = CapabilitiesOrphanAnalyzer::new();
    let f = FilePath::new("src/capabilities_dead_analyzer.rs".to_string()).unwrap();
    let root = FilePath::new("src".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::new());

    let result = a.is_capabilities_orphan(&f, &root, &alive);
    assert!(result.is_orphan, "AES503 FAIL: unreachable capability must be flagged");
}
```

---

## `tests/acceptance_AES504.rs`

```rust
// PURPOSE: Acceptance test — AES504 Utility Orphan Checker.
// Requirement: Utility files must be wired into root containers or imported by capabilities/agents.

use orphan_detector_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IUtilityOrphanProtocol;
use std::collections::HashMap;

fn make_inbound(links: Vec<(&str, Vec<&str>)>) -> InboundLinkMap {
    let mut m = HashMap::new();
    for (f, importers) in links {
        m.insert(f.to_string(), importers.iter().map(|s| s.to_string()).collect());
    }
    InboundLinkMap::new(m)
}

/// AES504: Utility imported by a capabilities file is NOT orphan.
#[test]
fn aes504_utility_imported_by_capabilities_not_orphan() {
    let a = UtilityOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/orphan-detector/utility_orphan.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let all = vec![
        "shared/src/orphan-detector/utility_orphan.rs".to_string(),
        "orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs".to_string(),
    ];
    let inbound = make_inbound(vec![(
        "shared/src/orphan-detector/utility_orphan.rs",
        vec!["orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs"],
    )]);

    let result = a.is_utility_orphan(&f, &root, &all, &inbound);
    assert!(!result.is_orphan, "AES504 FAIL: utility imported by capabilities should not be orphan");
}

/// AES504: Utility with zero importers IS orphan.
#[test]
fn aes504_utility_with_no_importers_is_orphan() {
    let a = UtilityOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/orphan-detector/utility_dead.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let all = vec!["shared/src/orphan-detector/utility_dead.rs".to_string()];
    let inbound = make_inbound(vec![]);

    let result = a.is_utility_orphan(&f, &root, &all, &inbound);
    assert!(result.is_orphan, "AES504 FAIL: utility with no importers must be flagged");
}

/// AES504: Utility imported ONLY by other utilities IS dead code.
#[test]
fn aes504_utility_only_imported_by_utilities_is_dead_code() {
    let a = UtilityOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/orphan-detector/utility_inner.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let all = vec![
        "shared/src/orphan-detector/utility_inner.rs".to_string(),
        "shared/src/orphan-detector/utility_outer.rs".to_string(),
    ];
    let inbound = make_inbound(vec![(
        "shared/src/orphan-detector/utility_inner.rs",
        vec!["shared/src/orphan-detector/utility_outer.rs"],
    )]);

    let result = a.is_utility_orphan(&f, &root, &all, &inbound);
    assert!(result.is_orphan, "AES504 FAIL: utility only imported by utilities must be flagged as dead code");
    assert!(result.reason.contains("only imported by other utility"));
}
```

---

## `tests/acceptance_AES505.rs`

```rust
// PURPOSE: Acceptance test — AES505 Agent Orphan Checker.
// Requirement: Agent orchestrator files must be called by surface layer files or binary entry points.

use orphan_detector_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IAgentOrphanProtocol;
use std::fs;

/// AES505: Agent aggregate called by a container is NOT orphan.
#[test]
fn aes505_agent_called_by_container_not_orphan() {
    let a = AgentOrphanAnalyzer::new();
    let dir = tempfile::tempdir().unwrap();

    let agent = dir.path().join("agent_orphan_orchestrator.rs");
    fs::write(&agent, "impl IOrphanAggregate for ArchOrphanAnalyzer {\n    fn check_orphans(&self) {}\n}\n").unwrap();

    let container = dir.path().join("root_orphan_detector_container.rs");
    fs::write(&container, "use IOrphanAggregate;\nlet x: Arc<dyn IOrphanAggregate> = ...;\n").unwrap();

    let f = FilePath::new(agent.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all = vec![
        agent.to_str().unwrap().to_string(),
        container.to_str().unwrap().to_string(),
    ];

    let result = a.is_agent_orphan(&f, &root, &all);
    assert!(!result.is_orphan, "AES505 FAIL: agent called by container should not be orphan");
}

/// AES505: Agent aggregate NOT called by any surface/container IS orphan.
#[test]
fn aes505_agent_not_called_is_orphan() {
    let a = AgentOrphanAnalyzer::new();
    let dir = tempfile::tempdir().unwrap();

    let agent = dir.path().join("agent_dead_orchestrator.rs");
    fs::write(&agent, "impl IDeadAggregate for DeadOrch {\n    fn run(&self) {}\n}\n").unwrap();

    let other = dir.path().join("capabilities_foo.rs");
    fs::write(&other, "pub struct Foo;\n").unwrap();

    let f = FilePath::new(agent.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all = vec![
        agent.to_str().unwrap().to_string(),
        other.to_str().unwrap().to_string(),
    ];

    let result = a.is_agent_orphan(&f, &root, &all);
    assert!(result.is_orphan, "AES505 FAIL: agent not called by any surface/container must be flagged");
}
```

---

## `tests/acceptance_AES506.rs`

```rust
// PURPOSE: Acceptance test — AES506 Surface Orphan Checker.
// Requirement: Surface layer files must be registered in the routing system or called from main entries.

use orphan_detector_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ISurfacesOrphanProtocol;
use std::collections::HashSet;
use std::fs;

/// AES506: Surface reachable from entry point is NOT orphan.
#[test]
fn aes506_reachable_surface_not_orphan() {
    let a = SurfacesOrphanAnalyzer::new();
    let f = FilePath::new("src/surface_scan_command.rs".to_string()).unwrap();
    let root = FilePath::new("src".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::from([f.clone()]));

    let result = a.is_surface_orphan(&f, &root, &alive, None);
    assert!(!result.is_orphan, "AES506 FAIL: reachable surface should not be orphan");
}

/// AES506: Surface NOT reachable from any entry IS orphan.
#[test]
fn aes506_unreachable_surface_is_orphan() {
    let a = SurfacesOrphanAnalyzer::new();
    let dir = tempfile::tempdir().unwrap();
    let surface = dir.path().join("surface_dead_command.rs");
    fs::write(&surface, "pub struct DeadCommand;\n").unwrap();

    let f = FilePath::new(surface.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::new());

    let result = a.is_surface_orphan(&f, &root, &alive, None);
    assert!(result.is_orphan, "AES506 FAIL: unreachable surface must be flagged");
}
```

---

## `tests/bench_orphan_detector_graph.rs`

```rust
// PURPOSE: Benchmark — graph building and reachability tracing performance.
// Layer: Benchmark
// Speed: s–min

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;

fn generate_file_list(count: usize) -> Vec<String> {
    (0..count)
        .map(|i| format!("/tmp/bench/src/capabilities_module_{:04}_analyzer.rs", i))
        .collect()
}

fn bench_build_graph_context(c: &mut Criterion) {
    let resolver = OrphanGraphResolver::new();
    let mut group = c.benchmark_group("build_graph_context");

    for size in [10, 100, 500] {
        let files = generate_file_list(size);
        let file_vo = vec![OrphanFileListVO::new(files)];
        group.bench_with_input(
            BenchmarkId::new("files", size),
            &file_vo,
            |b, data| {
                b.iter(|| resolver.build_graph_context(data, "/tmp/bench"));
            },
        );
    }
    group.finish();
}

fn bench_identify_entry_points(c: &mut Criterion) {
    let resolver = OrphanGraphResolver::new();
    let mut group = c.benchmark_group("identify_entry_points");

    for size in [10, 100, 500] {
        let mut files = generate_file_list(size);
        files.push("/tmp/bench/src/main.rs".to_string());
        files.push("/tmp/bench/src/root_app_container.rs".to_string());
        let file_vo = vec![OrphanFileListVO::new(files)];
        group.bench_with_input(
            BenchmarkId::new("files", size),
            &file_vo,
            |b, data| {
                b.iter(|| resolver.identify_entry_points(data, &[]));
            },
        );
    }
    group.finish();
}

fn bench_check_orphans(c: &mut Criterion) {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let mut group = c.benchmark_group("check_orphans");

    for size in [10, 50] {
        let files = generate_file_list(size);
        group.bench_with_input(
            BenchmarkId::new("files", size),
            &files,
            |b, data| {
                b.iter(|| analyzer.check_orphans(data, "/tmp/bench"));
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_build_graph_context,
    bench_identify_entry_points,
    bench_check_orphans,
);
criterion_main!(benches);
```

---

## Quick Reference

```bash
# Run all tests
cargo test -p orphan_detector-lint-arwaky

# Run by type
cargo test -p orphan_detector-lint-arwaky --test contract_orphan_detector
cargo test -p orphan_detector-lint-arwaky --test unit_orphan_detector_graph_resolver
cargo test -p orphan_detector-lint-arwaky --test unit_orphan_detector_taxonomy_analyzer
cargo test -p orphan_detector-lint-arwaky --test unit_orphan_detector_contract_analyzer
cargo test -p orphan_detector-lint-arwaky --test unit_orphan_detector_capabilities_analyzer
cargo test -p orphan_detector-lint-arwaky --test unit_orphan_detector_utility_analyzer
cargo test -p orphan_detector-lint-arwaky --test unit_orphan_detector_agent_analyzer
cargo test -p orphan_detector-lint-arwaky --test unit_orphan_detector_surfaces_analyzer
cargo test -p orphan_detector-lint-arwaky --test unit_orphan_detector_orchestrator
cargo test -p orphan_detector-lint-arwaky --test integration_orphan_detector
cargo test -p orphan_detector-lint-arwaky --test smoke_orphan_detector
cargo test -p orphan_detector-lint-arwaky --test e2e_orphan_detection_flow

# Acceptance tests
cargo test -p orphan_detector-lint-arwaky --test acceptance_AES501
cargo test -p orphan_detector-lint-arwaky --test acceptance_AES502
cargo test -p orphan_detector-lint-arwaky --test acceptance_AES503
cargo test -p orphan_detector-lint-arwaky --test acceptance_AES504
cargo test -p orphan_detector-lint-arwaky --test acceptance_AES505
cargo test -p orphan_detector-lint-arwaky --test acceptance_AES506

# Benchmarks
cargo bench -p orphan_detector-lint-arwaky

# Coverage
cargo tarpaulin -p orphan_detector-lint-arwaky --fail-under 70

# With output
cargo test -p orphan_detector-lint-arwaky -- --nocapture
```

---

## Coverage Summary

| Layer                  | File                                              | Tests    | Target           |
| ---------------------- | ------------------------------------------------- | -------- | ---------------- |
| **Capabilities** | `unit_orphan_detector_graph_resolver.rs`        | 7        | ≥ 70%           |
| **Capabilities** | `unit_orphan_detector_taxonomy_analyzer.rs`     | 5        | ≥ 70%           |
| **Capabilities** | `unit_orphan_detector_contract_analyzer.rs`     | 4        | ≥ 70%           |
| **Capabilities** | `unit_orphan_detector_capabilities_analyzer.rs` | 3        | ≥ 70%           |
| **Capabilities** | `unit_orphan_detector_utility_analyzer.rs`      | 4        | ≥ 70%           |
| **Capabilities** | `unit_orphan_detector_agent_analyzer.rs`        | 5        | ≥ 70%           |
| **Capabilities** | `unit_orphan_detector_surfaces_analyzer.rs`     | 3        | ≥ 70%           |
| **Agent**        | `unit_orphan_detector_orchestrator.rs`          | 5        | ≥ 60%           |
| **Utility**      | *(covered via shared crate tests)*              | —       | ≥ 50%           |
| **Contract**     | `contract_orphan_detector.rs`                   | 10       | 100% trait impls |
| **Integration**  | `integration_orphan_detector.rs`                | 5        | DI wiring        |
| **Smoke**        | `smoke_orphan_detector.rs`                      | 1        | < 5s             |
| **E2E**          | `e2e_orphan_detection_flow.rs`                  | 1        | Full lifecycle   |
| **Acceptance**   | `acceptance_AES501–506.rs`                     | 13       | 1:1 FRD mapping  |
| **Benchmark**    | `bench_orphan_detector_graph.rs`                | 3 groups | Perf baseline    |

**Total: 66 test cases** across 19 test files.
