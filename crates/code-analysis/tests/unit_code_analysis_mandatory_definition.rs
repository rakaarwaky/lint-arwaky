// PURPOSE: Unit tests for MandatoryDefinitionChecker (AES303) —
// mandatory class/struct definition + dead inheritance detection.

use code_analysis_lint_arwaky::MandatoryDefinitionChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::common::taxonomy_common_vo::{BooleanVO, Count};
use shared::common::taxonomy_definition_vo::LayerDefinition;

fn checker() -> MandatoryDefinitionChecker {
    MandatoryDefinitionChecker::new()
}

fn make_def(mandatory: bool) -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO {
            mandatory_class_definition: BooleanVO::new(mandatory),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn check_mandatory(file: &str, def: Option<&LayerDefinition>, content: &str) -> Vec<LintResult> {
    let mut violations = Vec::new();
    checker().check_mandatory_class_definition(file, def, content, &mut violations);
    violations
}

fn check_dead(file: &str, content: &str) -> Vec<LintResult> {
    let mut violations = Vec::new();
    checker().check_dead_inheritance(file, content, &mut violations);
    violations
}

// ─── AES303 Sub-check 1: Mandatory definition ────────────────────────

#[test]
fn rust_struct_satisfies_mandatory() {
    let def = make_def(true);
    let content = "pub struct Foo {\n    x: i32,\n}";
    let violations = check_mandatory("capabilities_foo.rs", Some(&def), content);
    assert!(violations.is_empty());
}

#[test]
fn rust_enum_satisfies_mandatory() {
    let def = make_def(true);
    let content = "pub enum Color {\n    Red,\n    Green,\n}";
    let violations = check_mandatory("capabilities_foo.rs", Some(&def), content);
    assert!(violations.is_empty());
}

#[test]
fn rust_trait_satisfies_mandatory() {
    let def = make_def(true);
    let content = "pub trait MyTrait {\n    fn do_thing(&self);\n}";
    let violations = check_mandatory("capabilities_foo.rs", Some(&def), content);
    assert!(violations.is_empty());
}

#[test]
fn rust_type_alias_satisfies_mandatory() {
    let def = make_def(true);
    let content = "pub type Result<T> = std::result::Result<T, Error>;";
    let violations = check_mandatory("capabilities_foo.rs", Some(&def), content);
    assert!(violations.is_empty());
}

#[test]
fn missing_definition_emits_aes303() {
    let def = make_def(true);
    let content = "pub fn helper() -> i32 {\n    42\n}";
    let violations = check_mandatory("capabilities_foo.rs", Some(&def), content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES303");
    assert!(violations[0].message.value.contains("MANDATORY_DEFINITION"));
}

#[test]
fn js_class_satisfies_mandatory() {
    let def = make_def(true);
    let content = "export class MyService {\n  run() {}\n}";
    let violations = check_mandatory("capabilities_foo.ts", Some(&def), content);
    assert!(violations.is_empty());
}

#[test]
fn ts_interface_satisfies_mandatory() {
    let def = make_def(true);
    let content = "export interface IService {\n  run(): void;\n}";
    let violations = check_mandatory("capabilities_foo.ts", Some(&def), content);
    assert!(violations.is_empty());
}

#[test]
fn python_class_satisfies_mandatory() {
    let def = make_def(true);
    let content = "class MyService:\n    def run(self):\n        pass";
    let violations = check_mandatory("capabilities_foo.py", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── Edge Case: mandatory_class_definition disabled → skip ───────────

#[test]
fn disabled_mandatory_check_skips() {
    let def = make_def(false);
    let content = "pub fn helper() -> i32 { 42 }";
    let violations = check_mandatory("capabilities_foo.rs", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── Edge Case: No definition provided → skip ────────────────────────

#[test]
fn no_definition_skips_mandatory_check() {
    let content = "pub fn helper() -> i32 { 42 }";
    let violations = check_mandatory("capabilities_foo.rs", None, content);
    assert!(violations.is_empty());
}

// ─── Edge Case: Barrel files skipped ─────────────────────────────────

#[test]
fn mod_rs_skipped_for_mandatory() {
    let def = make_def(true);
    let content = "// re-exports only";
    let violations = check_mandatory("mod.rs", Some(&def), content);
    assert!(violations.is_empty());
}

#[test]
fn lib_rs_skipped_for_mandatory() {
    let def = make_def(true);
    let content = "pub mod foo;";
    let violations = check_mandatory("lib.rs", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── Edge Case: Constant files skipped ───────────────────────────────

#[test]
fn constant_file_skipped() {
    let def = make_def(true);
    let content = "pub const MAX: usize = 100;";
    let violations = check_mandatory("taxonomy_foo_constant.rs", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── AES303 Sub-check 2: Dead inheritance ────────────────────────────

#[test]
fn unit_struct_without_impl_flagged() {
    let content = "pub struct Foo;";
    let violations = check_dead("test.rs", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES303");
    assert!(violations[0].message.value.contains("DEAD_INHERITANCE"));
}

#[test]
fn unit_struct_with_impl_not_flagged() {
    let content = "pub struct Foo;\n\nimpl Foo {\n    pub fn new() -> Self { Self }\n}";
    let violations = check_dead("test.rs", content);
    assert!(violations.is_empty());
}

#[test]
fn normal_struct_not_flagged() {
    let content = "pub struct Foo {\n    x: i32,\n}";
    let violations = check_dead("test.rs", content);
    assert!(violations.is_empty());
}

#[test]
fn tuple_struct_not_flagged() {
    let content = "pub struct Wrapper(i32);";
    let violations = check_dead("test.rs", content);
    assert!(violations.is_empty());
}

#[test]
fn python_empty_class_pass_flagged() {
    let content = "class Foo: pass";
    let violations = check_dead("test.py", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES303");
}

#[test]
fn python_empty_class_multiline_flagged() {
    let content = "class Foo:\n    pass";
    let violations = check_dead("test.py", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES303");
}

#[test]
fn js_empty_class_flagged() {
    let content = "class Foo {}";
    let violations = check_dead("test.ts", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES303");
}

#[test]
fn js_export_empty_class_flagged() {
    let content = "export class Foo {}";
    let violations = check_dead("test.ts", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES303");
}

#[test]
fn ts_empty_interface_flagged() {
    let content = "interface Foo {}";
    let violations = check_dead("test.ts", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES303");
}

// ─── Edge Case: cfg(test) blocks skipped in dead inheritance ─────────

#[test]
fn cfg_test_block_skipped_in_dead_inheritance() {
    let content = r#"
#[cfg(test)]
mod tests {
    struct TestHelper;
}
"#;
    let violations = check_dead("test.rs", content);
    assert!(violations.is_empty());
}

// ─── Edge Case: pub(crate) visibility stripped ───────────────────────

#[test]
fn pub_crate_unit_struct_detected() {
    let content = "pub(crate) struct Foo;";
    let violations = check_dead("test.rs", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES303");
}
