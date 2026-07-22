// PURPOSE: Acceptance test for FR-003: Mandatory Definitions (AES303)
// Source files must declare at least one primary symbol.

use code_analysis_lint_arwaky::MandatoryDefinitionChecker;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::common::taxonomy_common_vo::BooleanVO;
use shared::common::taxonomy_definition_vo::LayerDefinition;

fn make_def() -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO {
            mandatory_class_definition: BooleanVO::new(true),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// FR-003: Rust file with struct passes
#[test]
fn fr_003_rust_struct_passes() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "pub struct MyType {\n    field: i32,\n}";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition(
        "capabilities_foo.rs",
        Some(&def),
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

/// FR-003: Rust file with enum passes
#[test]
fn fr_003_rust_enum_passes() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "pub enum Status {\n    Active,\n    Inactive,\n}";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition(
        "capabilities_foo.rs",
        Some(&def),
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

/// FR-003: Rust file with trait passes
#[test]
fn fr_003_rust_trait_passes() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "pub trait Handler {\n    fn handle(&self);\n}";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition(
        "capabilities_foo.rs",
        Some(&def),
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

/// FR-003: Rust file with type alias passes
#[test]
fn fr_003_rust_type_passes() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "pub type HandlerFn = Box<dyn Fn()>;";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition(
        "capabilities_foo.rs",
        Some(&def),
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

/// FR-003: File without definitions fails with AES303
#[test]
fn fr_003_no_definition_fails_aes303() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "pub fn helper() -> i32 {\n    42\n}";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition(
        "capabilities_foo.rs",
        Some(&def),
        content,
        &mut violations,
    );

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES303");
    assert!(violations[0].message.value.contains("MANDATORY_DEFINITION"));
}

/// FR-003: Python class passes
#[test]
fn fr_003_python_class_passes() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "class Service:\n    def run(self):\n        return True";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition(
        "capabilities_foo.py",
        Some(&def),
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

/// FR-003: TypeScript interface passes
#[test]
fn fr_003_typescript_interface_passes() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "export interface IService {\n    run(): void;\n}";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition(
        "capabilities_foo.ts",
        Some(&def),
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

/// FR-003: Empty unit struct flagged as dead inheritance
#[test]
fn fr_003_empty_unit_struct_flagged() {
    let checker = MandatoryDefinitionChecker::new();
    let content = "pub struct Empty;";

    let mut violations = Vec::new();
    checker.check_dead_inheritance("test.rs", content, &mut violations);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES303");
}
