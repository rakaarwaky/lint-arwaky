use code_analysis_lint_arwaky::capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;

// ─── check_mandatory_class_definition ───────────────────────────────────────

#[test]
fn skips_mod_rs() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    checker.check_mandatory_class_definition(
        "src/mod.rs",
        None,
        "use std::collections;",
        &mut violations,
    );
    assert!(violations.is_empty());
}

#[test]
fn skips_init_py() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    checker.check_mandatory_class_definition("src/__init__.py", None, "", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn skips_constant_file() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    checker.check_mandatory_class_definition(
        "taxonomy_colors_constant.rs",
        None,
        "pub const X: usize = 1;",
        &mut violations,
    );
    assert!(violations.is_empty());
}

#[test]
fn no_definition_no_check() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    checker.check_mandatory_class_definition(
        "src/foo.rs",
        None,
        "// just a comment",
        &mut violations,
    );
    assert!(violations.is_empty());
}

#[test]
fn disabled_mandatory_class_no_violation() {
    use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
    use shared::taxonomy_common_vo::{BooleanVO, Count, PatternList};
    use shared::taxonomy_definition_vo::LayerDefinition;

    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    let def = LayerDefinition {
        code_analysis: CodeAnalysisRuleVO {
            min_lines: Count::new(1),
            max_lines: Count::new(100),
            mandatory_class_definition: BooleanVO::new(false),
            ..Default::default()
        },
        exceptions: PatternList::default(),
        ..Default::default()
    };
    checker.check_mandatory_class_definition(
        "src/foo.rs",
        Some(&def),
        "fn helper() {}",
        &mut violations,
    );
    assert!(violations.is_empty());
}

#[test]
fn missing_class_emits_violation() {
    use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
    use shared::taxonomy_common_vo::{BooleanVO, Count, PatternList};
    use shared::taxonomy_definition_vo::LayerDefinition;

    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    let def = LayerDefinition {
        code_analysis: CodeAnalysisRuleVO {
            min_lines: Count::new(1),
            max_lines: Count::new(100),
            mandatory_class_definition: BooleanVO::new(true),
            ..Default::default()
        },
        exceptions: PatternList::default(),
        ..Default::default()
    };
    checker.check_mandatory_class_definition(
        "src/foo.rs",
        Some(&def),
        "fn helper() {}",
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.to_string().contains("AES303"));
}

#[test]
fn has_pub_struct_no_violation() {
    use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
    use shared::taxonomy_common_vo::{BooleanVO, Count, PatternList};
    use shared::taxonomy_definition_vo::LayerDefinition;

    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    let def = LayerDefinition {
        code_analysis: CodeAnalysisRuleVO {
            min_lines: Count::new(1),
            max_lines: Count::new(100),
            mandatory_class_definition: BooleanVO::new(true),
            ..Default::default()
        },
        exceptions: PatternList::default(),
        ..Default::default()
    };
    checker.check_mandatory_class_definition(
        "src/foo.rs",
        Some(&def),
        "pub struct Foo;",
        &mut violations,
    );
    assert!(violations.is_empty());
}

#[test]
fn exception_list_skips_file() {
    use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
    use shared::taxonomy_common_vo::{BooleanVO, Count, PatternList};
    use shared::taxonomy_definition_vo::LayerDefinition;

    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    let def = LayerDefinition {
        code_analysis: CodeAnalysisRuleVO {
            min_lines: Count::new(1),
            max_lines: Count::new(100),
            mandatory_class_definition: BooleanVO::new(true),
            ..Default::default()
        },
        exceptions: PatternList::new(vec!["foo.rs".to_string()]),
        ..Default::default()
    };
    checker.check_mandatory_class_definition(
        "src/foo.rs",
        Some(&def),
        "fn helper() {}",
        &mut violations,
    );
    assert!(violations.is_empty());
}

// ─── check_dead_inheritance ─────────────────────────────────────────────────

#[test]
fn unit_struct_without_impl_emits_violation() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    let content = "struct Foo;\npub fn bar() {}";
    checker.check_dead_inheritance("test.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.to_string().contains("AES303"));
}

#[test]
fn unit_struct_followed_by_impl_no_violation() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    let content = "struct Foo;\nimpl Foo {\n    fn bar() {}\n}";
    checker.check_dead_inheritance("test.rs", content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn python_empty_class_pass_emits_violation() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    checker.check_dead_inheritance("test.py", "class Foo: pass", &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn python_empty_class_with_newline_pass_emits_violation() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    checker.check_dead_inheritance("test.py", "class Foo:\n    pass", &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn python_non_empty_class_no_violation() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    checker.check_dead_inheritance(
        "test.py",
        "class Foo:\n    def bar(self): pass",
        &mut violations,
    );
    assert!(violations.is_empty());
}

#[test]
fn js_empty_class_emits_violation() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    checker.check_dead_inheritance("test.ts", "class Foo {}", &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn js_non_empty_class_no_violation() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    checker.check_dead_inheritance("test.ts", "class Foo {\n    bar() {}\n}", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn skips_cfg_test_blocks() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    let content = "#[cfg(test)]\nmod tests {\n    pub struct Empty;\n}\nfn real_fn() {}";
    checker.check_dead_inheritance("test.rs", content, &mut violations);
    // Empty struct inside cfg(test) should not be flagged
    assert!(violations.is_empty());
}

#[test]
fn normal_file_without_unit_structs_no_violation() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    let content = "pub fn helper() {\n    let x = 1;\n}";
    checker.check_dead_inheritance("test.rs", content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn python_ellipsis_body_emits_violation() {
    let checker = MandatoryDefinitionChecker::new();
    let mut violations = Vec::new();
    checker.check_dead_inheritance("test.py", "class Foo:\n    ...", &mut violations);
    assert_eq!(violations.len(), 1);
}
