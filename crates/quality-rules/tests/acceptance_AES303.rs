// PURPOSE: Acceptance test AES303 — missing definitions (mandatory class/struct/enum/trait)
use shared::cli_commands::LintResult;
use shared::common::{BooleanVO, LayerDefinition};
use shared::quality_rules::{
    CodeAnalysisRuleVO, IDeadInheritanceProtocol, IMandatoryClassProtocol,
};

use quality_rules_lint_arwaky::capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;

fn checker() -> MandatoryDefinitionChecker {
    MandatoryDefinitionChecker::new()
}

fn def_with_mandatory(enabled: bool) -> LayerDefinition {
    LayerDefinition {
        code_analysis: CodeAnalysisRuleVO {
            mandatory_class_definition: BooleanVO::new(enabled),
            ..Default::default()
        },
        exceptions: shared::common::PatternList { values: vec![] },
        ..Default::default()
    }
}

// ── Mandatory definition check (sub-check 1) ────────────────

#[test]
fn file_without_struct_or_class_produces_aes303() {
    let def = def_with_mandatory(true);
    let content = "let x = 1;\nlet y = 2;\nfn helper() {}\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_mandatory_class_definition(
        "src/capabilities/my_logic.rs",
        Some(&def),
        content,
        &mut violations,
    );

    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES303"));
    assert_eq!(violations[0].severity, shared::common::Severity::HIGH);
}

#[test]
fn file_with_struct_no_violation() {
    let def = def_with_mandatory(true);
    let content = "pub struct MyStruct {\n    pub field: i32,\n}\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_mandatory_class_definition(
        "src/capabilities/my_struct.rs",
        Some(&def),
        content,
        &mut violations,
    );

    assert!(violations.is_empty());
}

#[test]
fn file_with_enum_no_violation() {
    let def = def_with_mandatory(true);
    let content = "pub enum Color {\n    Red,\n    Green,\n    Blue,\n}\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_mandatory_class_definition(
        "src/capabilities/color.rs",
        Some(&def),
        content,
        &mut violations,
    );

    assert!(violations.is_empty());
}

#[test]
fn file_with_trait_no_violation() {
    let def = def_with_mandatory(true);
    let content = "pub trait MyTrait {\n    fn do_something(&self);\n}\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_mandatory_class_definition(
        "src/capabilities/my_trait.rs",
        Some(&def),
        content,
        &mut violations,
    );

    assert!(violations.is_empty());
}

#[test]
fn file_with_python_class_no_violation() {
    let def = def_with_mandatory(true);
    let content = "class MyClass:\n    def __init__(self):\n        self.x = 1\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_mandatory_class_definition(
        "src/capabilities/my_class.py",
        Some(&def),
        content,
        &mut violations,
    );

    assert!(violations.is_empty());
}

#[test]
fn file_with_ts_interface_no_violation() {
    let def = def_with_mandatory(true);
    let content = "export interface MyInterface {\n    foo: string;\n}\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_mandatory_class_definition(
        "src/capabilities/my_interface.ts",
        Some(&def),
        content,
        &mut violations,
    );

    assert!(violations.is_empty());
}

// ── Dead inheritance check (sub-check 2) ────────────────────

#[test]
fn unit_struct_without_impl_produces_aes303_dead_inheritance() {
    let content = "struct Placeholder;\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_dead_inheritance("src/capabilities/placeholder.rs", content, &mut violations);

    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES303"));
    assert!(violations[0].message.value.contains("Unit struct"));
}

#[test]
fn unit_struct_with_impl_no_violation() {
    let content =
        "struct Placeholder;\n\nimpl Placeholder {\n    fn new() -> Self { Placeholder }\n}\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_dead_inheritance("src/capabilities/placeholder.rs", content, &mut violations);

    assert!(violations.is_empty());
}

#[test]
fn python_empty_class_produces_aes303() {
    let content = "class Empty:\n    pass\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_dead_inheritance("src/capabilities/empty.py", content, &mut violations);

    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES303"));
    assert!(violations[0].message.value.contains("Empty Python class"));
}

#[test]
fn js_empty_class_produces_aes303() {
    let content = "class Empty {}\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_dead_inheritance("src/capabilities/empty.ts", content, &mut violations);

    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES303"));
}

#[test]
fn js_empty_interface_produces_aes303() {
    let content = "interface Empty {}\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker().check_dead_inheritance("src/capabilities/empty.ts", content, &mut violations);

    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES303"));
}
