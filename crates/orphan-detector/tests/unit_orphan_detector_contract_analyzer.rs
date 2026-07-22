// PURPOSE: Unit tests for ContractOrphanAnalyzer — AES502 contract orphan detection.
// Layer: Capabilities (ContractOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use shared::common::taxonomy_severity_vo::Severity;
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

    let result = a.is_contract_orphan(
        &f,
        &root,
        &empty_definitions(),
        &empty_inheritance(),
        &all_files,
    );
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

    let result = a.is_contract_orphan(
        &f,
        &root,
        &empty_definitions(),
        &empty_inheritance(),
        &all_files,
    );
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

    let result = a.is_contract_orphan(
        &f,
        &root,
        &empty_definitions(),
        &empty_inheritance(),
        &all_files,
    );
    assert!(!result.is_orphan);
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = ContractOrphanAnalyzer::default();
}
