use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer::extract_contract_trait_name;

#[test]
fn extract_contract_rust_trait() {
    let content = "pub trait IMyPort: Send + Sync { fn execute(&self); }";
    let name = extract_contract_trait_name(content);
    assert_eq!(name, Some("IMyPort".to_string()));
}

#[test]
fn extract_contract_ts_export_interface() {
    let content = "export interface IMyProtocol { execute(): void; }";
    let name = extract_contract_trait_name(content);
    assert_eq!(name, Some("IMyProtocol".to_string()));
}

#[test]
fn extract_contract_ts_plain_interface() {
    let content = "interface IMyAggregate { run(): void; }";
    let name = extract_contract_trait_name(content);
    assert_eq!(name, Some("IMyAggregate".to_string()));
}

#[test]
fn extract_contract_python_class() {
    let content = "class IMyProtocol:";
    let name = extract_contract_trait_name(content);
    assert_eq!(name, Some("IMyProtocol".to_string()));
}

#[test]
fn extract_contract_no_trait() {
    let content = "fn helper() -> bool { true }";
    let name = extract_contract_trait_name(content);
    assert_eq!(name, None);
}

#[test]
fn extract_contract_empty_content() {
    let name = extract_contract_trait_name("");
    assert_eq!(name, None);
}

#[test]
fn extract_contract_rust_trait_without_pub() {
    let content = "trait IPrivateContract { fn handle(&self); }";
    let name = extract_contract_trait_name(content);
    assert_eq!(name, Some("IPrivateContract".to_string()));
}
