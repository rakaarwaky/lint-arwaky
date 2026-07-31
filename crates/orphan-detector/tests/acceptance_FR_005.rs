// PURPOSE: Acceptance test — FR-005 Contract Orphan Checker (AES502).
// Requirement: Contract files must have at least one active implementation in capabilities or utility layers.

use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use shared::code_analysis::InheritanceMap;
use shared::common::FilePath;
use shared::orphan_detector::IContractOrphanProtocol;
use std::collections::HashMap;
use std::fs;

fn empty_inh() -> InheritanceMap {
    InheritanceMap::new(HashMap::new())
}

/// AES502: Contract protocol with an impl in capabilities is NOT orphan.
#[test]
fn fr005_contract_with_impl_not_orphan() {
    let a = ContractOrphanAnalyzer::default();
    let dir = tempfile::tempdir().unwrap();

    let contract = dir.path().join("contract_greeter_protocol.rs");
    fs::write(
        &contract,
        "pub trait IGreeterProtocol: Send + Sync {\n    fn greet(&self) -> String;\n}\n",
    )
    .unwrap();

    let impl_file = dir.path().join("capabilities_greeter_analyzer.rs");
    fs::write(&impl_file, "impl IGreeterProtocol for GreeterAnalyzer {\n    fn greet(&self) -> String { \"hi\".into() }\n}\n").unwrap();

    let f = FilePath::new(contract.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all = vec![
        contract.to_str().unwrap().to_string(),
        impl_file.to_str().unwrap().to_string(),
    ];

    let result = a.is_contract_orphan(&f, &root, &empty_inh(), &all);
    assert!(
        !result.is_orphan,
        "AES502 FAIL: contract with implementation should not be orphan"
    );
}

/// AES502: Contract protocol with NO impl IS orphan.
#[test]
fn fr005_contract_without_impl_is_orphan() {
    let a = ContractOrphanAnalyzer::default();
    let dir = tempfile::tempdir().unwrap();

    let contract = dir.path().join("contract_dead_protocol.rs");
    fs::write(
        &contract,
        "pub trait IDeadProtocol: Send + Sync {\n    fn nothing(&self);\n}\n",
    )
    .unwrap();

    let f = FilePath::new(contract.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let all = vec![contract.to_str().unwrap().to_string()];

    let result = a.is_contract_orphan(&f, &root, &empty_inh(), &all);
    assert!(
        result.is_orphan,
        "AES502 FAIL: contract without implementation must be flagged"
    );
}
