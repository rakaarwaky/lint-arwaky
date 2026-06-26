use role_rules_lint_arwaky::capabilities_infrastructure_role_auditor::InfrastructureRoleChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn make_source(file: &str, content: &str, language: &str) -> SourceContentVO {
    let fp = FilePath::new(file.to_string()).unwrap_or_default();
    SourceContentVO::new(fp, ContentString::new(content.to_string()), language)
}

#[test]
fn rust_with_port_import_no_violation() {
    let checker = InfrastructureRoleChecker::new();
    let mut violations = Vec::new();
    let source = make_source(
        "infrastructure_test_adapter.rs",
        "use shared::contract::some_port::SomeTrait;\nimpl SomeTrait for TestAdapter {}",
        "rust",
    );
    checker.check_port_implementation(&source, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn rust_without_port_import_emits_violation() {
    let checker = InfrastructureRoleChecker::new();
    let mut violations = Vec::new();
    let source = make_source("infrastructure_random.rs", "fn helper() -> i32 { 42 }", "rust");
    checker.check_port_implementation(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.to_string().contains("AES404"));
}

#[test]
fn python_with_port_import_no_violation() {
    let checker = InfrastructureRoleChecker::new();
    let mut violations = Vec::new();
    let source = make_source(
        "infrastructure_test_adapter.py",
        "from somewhere import _port_interface\nclass TestAdapter(_port_interface): pass",
        "python",
    );
    checker.check_port_implementation(&source, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn js_without_port_import_emits_violation() {
    let checker = InfrastructureRoleChecker::new();
    let mut violations = Vec::new();
    let source = make_source("infrastructure_foo.js", "function helper() { return 42; }", "javascript");
    checker.check_port_implementation(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.to_string().contains("AES404"));
}

#[test]
fn python_without_port_import_emits_violation() {
    let checker = InfrastructureRoleChecker::new();
    let mut violations = Vec::new();
    let source = make_source("infrastructure_random.py", "x = 1", "python");
    checker.check_port_implementation(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.to_string().contains("AES404"));
}
