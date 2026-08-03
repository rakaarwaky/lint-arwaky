// Unit tests for MandatoryDefinitionChecker — AES303 mandatory class definition + dead inheritance.
use quality_rules_lint_arwaky::capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
use shared::common::{BooleanVO, LayerDefinition};
use shared::quality_rules::{
    CodeAnalysisRuleVO, IDeadInheritanceProtocol, IMandatoryClassProtocol,
};

fn checker() -> MandatoryDefinitionChecker {
    MandatoryDefinitionChecker::new()
}

fn def_with_mandatory(enabled: bool) -> LayerDefinition {
    LayerDefinition {
        code_analysis: CodeAnalysisRuleVO {
            mandatory_class_definition: BooleanVO::new(enabled),
            ..Default::default()
        },
        ..Default::default()
    }
}

// ── IMandatoryClassProtocol tests ──

#[test]
fn construction_succeeds() {
    let _ = checker();
}

#[test]
fn file_with_struct_no_violation() {
    let mut violations = Vec::new();
    let def = def_with_mandatory(true);
    let content = "pub struct Foo {\n    pub bar: i32,\n}\n";
    checker().check_mandatory_class_definition(
        "src/lib.rs",
        Some(&def),
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

#[test]
fn file_without_definition_produces_violation() {
    let mut violations = Vec::new();
    let def = def_with_mandatory(true);
    let content = "let x = 1;\nprintln!(\"hello\");\n";
    checker().check_mandatory_class_definition(
        "src/util.rs",
        Some(&def),
        content,
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES303"));
}

#[test]
fn barrel_file_skipped() {
    let mut violations = Vec::new();
    let def = def_with_mandatory(true);
    let content = "";
    checker().check_mandatory_class_definition(
        "src/mod.rs",
        Some(&def),
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

#[test]
fn file_with_trait_no_violation() {
    let mut violations = Vec::new();
    let def = def_with_mandatory(true);
    let content = "pub trait Foo {\n    fn bar(&self);\n}\n";
    checker().check_mandatory_class_definition(
        "src/contract.rs",
        Some(&def),
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

// ── IDeadInheritanceProtocol tests ──

#[test]
fn unit_struct_without_impl_produces_violation() {
    let mut violations = Vec::new();
    let content = "struct Foo;\n";
    checker().check_dead_inheritance("src/lib.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].message.value.contains("Unit struct"));
}

#[test]
fn unit_struct_with_impl_no_violation() {
    let mut violations = Vec::new();
    let content = "struct Foo;\n\nimpl Foo {\n    fn new() -> Self { Foo }\n}\n";
    checker().check_dead_inheritance("src/lib.rs", content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn unit_struct_with_derive_no_violation() {
    let mut violations = Vec::new();
    let content = "#[derive(Debug)]\nstruct Foo;\n";
    checker().check_dead_inheritance("src/lib.rs", content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn python_empty_class_produces_violation() {
    let mut violations = Vec::new();
    let content = "class Foo:\n    pass\n";
    checker().check_dead_inheritance("src/lib.py", content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].message.value.contains("Empty Python class"));
}

#[test]
fn python_class_with_body_no_violation() {
    let mut violations = Vec::new();
    let content = "class Foo:\n    x = 1\n";
    checker().check_dead_inheritance("src/lib.py", content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn python_abstract_class_skipped() {
    let mut violations = Vec::new();
    let content = "class Foo(ABC):\n    pass\n";
    checker().check_dead_inheritance("src/lib.py", content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn js_empty_class_produces_violation() {
    let mut violations = Vec::new();
    let content = "class Foo {}\n";
    checker().check_dead_inheritance("src/lib.ts", content, &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn cfg_test_block_skipped() {
    let mut violations = Vec::new();
    let content = "#[cfg(test)]\nmod tests {\n    struct Foo;\n}\n";
    checker().check_dead_inheritance("src/lib.rs", content, &mut violations);
    assert!(violations.is_empty());
}
