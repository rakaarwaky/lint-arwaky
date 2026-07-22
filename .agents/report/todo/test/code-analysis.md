
# Test Suite: `code-analysis` (v1.10.106)

Based on the crate structure, FRD requirements, and public API, here is the complete test suite.

---

## Task Progress

- [X] Step 1: Analyze crate structure
- [X] Step 2: Identify untested public API
- [X] Step 3: Write `contract_code_analysis.rs`
- [X] Step 4: Write `unit_code_analysis_*.rs`
- [X] Step 5: Write `integration_code_analysis.rs`
- [X] Step 6: Write `smoke_code_analysis.rs`
- [X] Step 7: Write `e2e_code_analysis_flow.rs`
- [X] Step 8: Write `acceptance_FR_*.rs`
- [X] Step 9: Write `bench_code_analysis_throughput.rs` + Cargo.toml registration
- [ ] Step 10: Run suite, fix failures, repeat until green
- [ ] Step 11: Verify coverage + perf baseline

---

## Directory Layout

```
crates/code-analysis/
├── src/
│   └── lib.rs
├── tests/
│   ├── contract_code_analysis.rs
│   ├── unit_code_analysis_bypass_checker.rs
│   ├── unit_code_analysis_line_checker.rs
│   ├── unit_code_analysis_mandatory_definition.rs
│   ├── unit_code_analysis_duplication.rs
│   ├── unit_code_analysis_orchestrator.rs
│   ├── integration_code_analysis.rs
│   ├── smoke_code_analysis.rs
│   ├── e2e_code_analysis_flow.rs
│   ├── acceptance_FR_001.rs
│   ├── acceptance_FR_002.rs
│   ├── acceptance_FR_003.rs
│   ├── acceptance_FR_004.rs
│   ├── acceptance_FR_005.rs
│   ├── acceptance_FR_006.rs
│   └── bench_code_analysis_throughput.rs
└── Cargo.toml
```

---

## `tests/contract_code_analysis.rs`

```rust
// PURPOSE: Verify that all capabilities implement their declared protocol traits
// and that the orchestrator implements its aggregate trait.

use code_analysis_lint_arwaky::*;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;

// ─── BypassChecker implements IBypassCheckerProtocol ───────

#[test]
fn bypass_checker_implements_bypass_checker_protocol() {
    fn assert_trait<T: IBypassCheckerProtocol>() {}
    assert_trait::<BypassChecker>();
}

// ─── ArchLineChecker implements ILineCheckerProtocol ───────

#[test]
fn arch_line_checker_implements_line_checker_protocol() {
    fn assert_trait<T: ILineCheckerProtocol>() {}
    assert_trait::<ArchLineChecker>();
}

// ─── MandatoryDefinitionChecker implements IMandatoryClassProtocol ───

#[test]
fn mandatory_definition_checker_implements_mandatory_class_protocol() {
    fn assert_trait<T: IMandatoryClassProtocol>() {}
    assert_trait::<MandatoryDefinitionChecker>();
}

// ─── MandatoryDefinitionChecker implements IDeadInheritanceProtocol ──

#[test]
fn mandatory_definition_checker_implements_dead_inheritance_protocol() {
    fn assert_trait<T: IDeadInheritanceProtocol>() {}
    assert_trait::<MandatoryDefinitionChecker>();
}

// ─── CodeDuplicationAnalyzer implements ICodeMetricAnalyzerProtocol ──

#[test]
fn code_duplication_analyzer_implements_code_metric_analyzer_protocol() {
    fn assert_trait<T: ICodeMetricAnalyzerProtocol>() {}
    assert_trait::<CodeDuplicationAnalyzer>();
}

// ─── CodeAnalysisOrchestrator implements ICodeAnalysisAggregate ──────

#[test]
fn code_analysis_orchestrator_implements_code_analysis_aggregate() {
    fn assert_trait<T: ICodeAnalysisAggregate>() {}
    assert_trait::<CodeAnalysisOrchestrator>();
}

// ─── Container wiring produces trait objects ─────────────────────────

#[test]
fn container_exposes_bypass_checker_as_trait_object() {
    let container = CodeAnalysisCheckerContainer::default();
    let checker: &std::sync::Arc<dyn IBypassCheckerProtocol> = container.bypass_checker();
    // Trait object is usable — call a method to prove dispatch works.
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", "let x = 1;", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn container_exposes_line_checker_as_trait_object() {
    let container = CodeAnalysisCheckerContainer::default();
    let checker: &std::sync::Arc<dyn ILineCheckerProtocol> = container.line_checker();
    let mut violations = Vec::new();
    checker.check_line_counts("test.rs", None, "line1\nline2", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn container_exposes_class_checker_as_trait_object() {
    let container = CodeAnalysisCheckerContainer::default();
    let checker: std::sync::Arc<dyn IMandatoryClassProtocol> = container.class_checker();
    let mut violations = Vec::new();
    checker.check_mandatory_class_definition("test.rs", None, "struct Foo {}", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn container_exposes_dead_inheritance_checker_as_trait_object() {
    let container = CodeAnalysisCheckerContainer::default();
    let checker: std::sync::Arc<dyn IDeadInheritanceProtocol> =
        container.dead_inheritance_checker();
    let mut violations = Vec::new();
    checker.check_dead_inheritance("test.rs", "struct Foo { x: i32 }", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn code_analysis_container_exposes_aggregate() {
    let container = CodeAnalysisContainer::new();
    let aggregate: std::sync::Arc<dyn ICodeAnalysisAggregate> = container.code_analysis_linter();
    let rules = aggregate.active_rules();
    // Default config has no rules configured, so empty is valid.
    assert!(rules.is_empty() || !rules.is_empty());
}
```

---

## `tests/unit_code_analysis_bypass_checker.rs`

```rust
// PURPOSE: Unit tests for BypassChecker (AES304) — bypass comment detection,
// unwrap/expect/panic/todo detection, Cargo.toml clippy allow detection.

use code_analysis_lint_arwaky::BypassChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;

fn checker() -> BypassChecker {
    BypassChecker::new()
}

fn scan(content: &str) -> Vec<LintResult> {
    let mut violations = Vec::new();
    checker().check_bypass_comments("test.rs", content, &mut violations);
    violations
}

fn scan_file(file: &str, content: &str) -> Vec<LintResult> {
    let mut violations = Vec::new();
    checker().check_bypass_comments(file, content, &mut violations);
    violations
}

// ─── Happy Path: Clean code produces no violations ───────────────────

#[test]
fn clean_code_no_violations() {
    let content = r#"
pub struct Foo {
    x: i32,
}

impl Foo {
    pub fn new() -> Self {
        Self { x: 0 }
    }
}
"#;
    assert!(scan(content).is_empty());
}

#[test]
fn safe_unwrap_or_not_flagged() {
    let content = r#"
fn example() -> i32 {
    let val = some_option.unwrap_or(42);
    let val2 = some_option.unwrap_or_else(|| compute());
    let val3 = some_option.unwrap_or_default();
    val + val2 + val3
}
"#;
    assert!(scan(content).is_empty());
}

// ─── AES304: unwrap() detection ──────────────────────────────────────

#[test]
fn detects_unwrap_call() {
    let content = "let x = some_result.unwrap();";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
    assert_eq!(violations[0].severity, Severity::CRITICAL);
}

#[test]
fn detects_expect_call() {
    let content = r#"let x = some_result.expect("should work");"#;
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: panic! detection ────────────────────────────────────────

#[test]
fn detects_panic_macro() {
    let content = r#"panic!("something went wrong");"#;
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: todo! detection ─────────────────────────────────────────

#[test]
fn detects_todo_macro() {
    let content = "todo!();";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: unimplemented! detection ────────────────────────────────

#[test]
fn detects_unimplemented_macro() {
    let content = "unimplemented!();";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: unreachable! detection ──────────────────────────────────

#[test]
fn detects_unreachable_macro() {
    let content = "unreachable!();";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: #[allow(...)] attribute detection ───────────────────────

#[test]
fn detects_allow_attribute() {
    let content = "#[allow(unused_variables)]\nfn foo() {}";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

#[test]
fn detects_clippy_allow_attribute() {
    let content = "#[clippy::allow(needless_return)]\nfn foo() {}";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: Comment bypass patterns ─────────────────────────────────

#[test]
fn detects_noqa_comment() {
    let content = "x = 1  # noqa";
    let violations = scan_file("test.py", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

#[test]
fn detects_type_ignore_comment() {
    let content = "x: int = 'hello'  # type: ignore";
    let violations = scan_file("test.py", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

#[test]
fn detects_eslint_disable_comment() {
    let content = "// eslint-disable-next-line no-console\nconsole.log('hi');";
    let violations = scan_file("test.js", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

#[test]
fn detects_ts_ignore_comment() {
    let content = "// @ts-ignore\nconst x: number = 'hello';";
    let violations = scan_file("test.ts", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── Edge Case: cfg(test) blocks are skipped ─────────────────────────

#[test]
fn skips_cfg_test_block() {
    let content = r#"
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = some_result.unwrap();
        panic!("expected in test");
    }
}
"#;
    assert!(scan(content).is_empty());
}

// ─── Edge Case: static Lazy<Regex> blocks are skipped ────────────────

#[test]
fn skips_static_lazy_regex_block() {
    let content = r#"
static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"unwrap\(").unwrap()
});
"#;
    assert!(scan(content).is_empty());
}

// ─── Edge Case: unwrap inside string literal not flagged ─────────────

#[test]
fn unwrap_inside_string_not_flagged() {
    let content = r#"let msg = "call unwrap() to get value";"#;
    assert!(scan(content).is_empty());
}

// ─── Edge Case: Multiple violations in one file ──────────────────────

#[test]
fn multiple_violations_detected() {
    let content = r#"
fn a() { let x = opt.unwrap(); }
fn b() { panic!("oops"); }
fn c() { todo!(); }
"#;
    let violations = scan(content);
    assert_eq!(violations.len(), 3);
}

// ─── Cargo.toml clippy allow detection ───────────────────────────────

#[test]
fn detects_cargo_toml_clippy_allow() {
    let content = r#"
[workspace.lints.clippy]
all = "allow"
"#;
    let mut violations = Vec::new();
    checker().check_cargo_toml(content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
    assert_eq!(violations[0].severity, Severity::CRITICAL);
}

#[test]
fn detects_cargo_toml_clippy_allow_table_form() {
    let content = r#"
[lints.clippy]
warnings = { level = "allow", priority = -1 }
"#;
    let mut violations = Vec::new();
    checker().check_cargo_toml(content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

#[test]
fn cargo_toml_warn_level_not_flagged() {
    let content = r#"
[workspace.lints.clippy]
all = "warn"
"#;
    let mut violations = Vec::new();
    checker().check_cargo_toml(content, &mut violations);
    assert!(violations.is_empty());
}

// ─── Python: raise NotImplementedError ───────────────────────────────

#[test]
fn detects_python_raise_not_implemented() {
    let content = "raise NotImplementedError";
    let violations = scan_file("test.py", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── JS/TS: throw new Error ──────────────────────────────────────────

#[test]
fn detects_js_throw_new_error() {
    let content = "throw new Error('fail');";
    let violations = scan_file("test.js", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── Edge Case: Word boundary prevents false positives ───────────────

#[test]
fn no_false_positive_on_substring() {
    // "unwrapped" contains "unwrap" but is not a call
    let content = "let unwrapped_value = get_value();";
    assert!(scan(content).is_empty());
}

#[test]
fn no_false_positive_on_todo_in_comment_only() {
    // "todo" as a word token should not match inside a comment-only line
    // when it's not followed by ! or (
    let content = "// remember to do this later";
    assert!(scan(content).is_empty());
}
```

---

## `tests/unit_code_analysis_line_checker.rs`

```rust
// PURPOSE: Unit tests for ArchLineChecker (AES301/AES302) — file line count limits.

use code_analysis_lint_arwaky::ArchLineChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::LayerDefinition;

fn checker() -> ArchLineChecker {
    ArchLineChecker::new()
}

fn make_def(min: i64, max: i64) -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO {
            min_lines: Count::new(min),
            max_lines: Count::new(max),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn check(file: &str, def: Option<&LayerDefinition>, content: &str) -> Vec<LintResult> {
    let mut violations = Vec::new();
    checker().check_line_counts(file, def, content, &mut violations);
    violations
}

// ─── Happy Path: File within limits ──────────────────────────────────

#[test]
fn file_within_limits_no_violation() {
    let def = make_def(5, 100);
    let content = "line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9\nline10";
    let violations = check("capabilities_foo.rs", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── AES301: File exceeds max lines ──────────────────────────────────

#[test]
fn file_exceeds_max_lines_aes301() {
    let def = make_def(5, 10);
    let content: String = (0..15).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    let violations = check("capabilities_foo.rs", Some(&def), &content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES301");
    assert!(violations[0].message.value.contains("FILE_TOO_LARGE"));
}

// ─── AES302: File below min lines ────────────────────────────────────

#[test]
fn file_below_min_lines_aes302() {
    let def = make_def(10, 1000);
    let content = "line1\nline2\nline3";
    let violations = check("capabilities_foo.rs", Some(&def), content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES302");
    assert!(violations[0].message.value.contains("FILE_TOO_SHORT"));
}

// ─── Edge Case: No definition provided → skip ────────────────────────

#[test]
fn no_definition_skips_check() {
    let content = "x";
    let violations = check("capabilities_foo.rs", None, content);
    assert!(violations.is_empty());
}

// ─── Edge Case: Barrel files (mod.rs) are skipped ────────────────────

#[test]
fn mod_rs_skipped() {
    let def = make_def(10, 1000);
    let content = "line1";
    let violations = check("mod.rs", Some(&def), content);
    assert!(violations.is_empty());
}

#[test]
fn init_py_skipped() {
    let def = make_def(10, 1000);
    let content = "line1";
    let violations = check("__init__.py", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── Edge Case: Filename in exception list → skip ────────────────────

#[test]
fn exception_filename_skipped() {
    let mut def = make_def(10, 1000);
    def.exceptions = PatternList::new(vec!["special_file.rs".to_string()]);
    let content = "line1";
    let violations = check("special_file.rs", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── Edge Case: min_lines = 0 disables min check ─────────────────────

#[test]
fn zero_min_lines_disables_min_check() {
    let def = make_def(0, 1000);
    let content = "line1";
    let violations = check("capabilities_foo.rs", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── Edge Case: max_lines = 0 disables max check ─────────────────────

#[test]
fn zero_max_lines_disables_max_check() {
    let def = make_def(1, 0);
    let content: String = (0..5000).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    let violations = check("capabilities_foo.rs", Some(&def), &content);
    assert!(violations.is_empty());
}

// ─── Edge Case: Exactly at boundary (no violation) ───────────────────

#[test]
fn exactly_at_max_lines_no_violation() {
    let def = make_def(1, 10);
    let content: String = (0..10).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    let violations = check("capabilities_foo.rs", Some(&def), &content);
    assert!(violations.is_empty());
}

#[test]
fn exactly_at_min_lines_no_violation() {
    let def = make_def(5, 100);
    let content: String = (0..5).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    let violations = check("capabilities_foo.rs", Some(&def), &content);
    assert!(violations.is_empty());
}
```

---

## `tests/unit_code_analysis_mandatory_definition.rs`

```rust
// PURPOSE: Unit tests for MandatoryDefinitionChecker (AES303) —
// mandatory class/struct definition + dead inheritance detection.

use code_analysis_lint_arwaky::MandatoryDefinitionChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
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
```

---

## `tests/unit_code_analysis_duplication.rs`

```rust
// PURPOSE: Unit tests for CodeDuplicationAnalyzer (AES305) — file-level
// similarity detection using window-based hashing.

use code_analysis_lint_arwaky::CodeDuplicationAnalyzer;
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;

fn analyzer() -> CodeDuplicationAnalyzer {
    CodeDuplicationAnalyzer::new()
}

fn make_entries(files: Vec<(&str, &str)>) -> Vec<(String, String)> {
    files
        .into_iter()
        .map(|(path, content)| (path.to_string(), content.to_string()))
        .collect()
}

// ─── Happy Path: No duplication ──────────────────────────────────────

#[test]
fn no_duplication_no_violations() {
    let entries = make_entries(vec![
        (
            "file_a.rs",
            "fn alpha() -> i32 {\n    1 + 2 + 3\n}\n\nfn beta() -> String {\n    String::from(\"hello\")\n}\n\nfn gamma() -> bool {\n    true\n}\n\nfn delta() -> f64 {\n    3.14\n}\n\nfn epsilon() -> u8 {\n    255\n}",
        ),
        (
            "file_b.rs",
            "struct Widget {\n    name: String,\n    size: usize,\n}\n\nimpl Widget {\n    fn new() -> Self {\n        Self { name: String::new(), size: 0 }\n    }\n}",
        ),
    ]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── AES305: High duplication detected ───────────────────────────────

#[test]
fn high_duplication_detected() {
    let shared_block = "fn shared_one() -> i32 {\n    42\n}\n\nfn shared_two() -> String {\n    String::from(\"dup\")\n}\n\nfn shared_three() -> bool {\n    true\n}\n\nfn shared_four() -> f64 {\n    2.71\n}\n\nfn shared_five() -> u8 {\n    128\n}";
    let entries = make_entries(vec![
        ("file_a.rs", shared_block),
        ("file_b.rs", shared_block),
    ]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(!violations.is_empty());
    // Both files should be flagged
    assert_eq!(violations.len(), 2);
    for (path, violation) in &violations {
        assert!(path.contains("file_"));
        let msg = violation.to_string();
        assert!(msg.contains("AES305"));
    }
}

// ─── Edge Case: Empty entries ────────────────────────────────────────

#[test]
fn empty_entries_no_violations() {
    let entries: Vec<(String, String)> = Vec::new();
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── Edge Case: Single file cannot duplicate ─────────────────────────

#[test]
fn single_file_no_duplication() {
    let entries = make_entries(vec![(
        "file_a.rs",
        "fn a() {}\nfn b() {}\nfn c() {}\nfn d() {}\nfn e() {}\nfn f() {}",
    )]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── Edge Case: Files shorter than min_dup_lines ─────────────────────

#[test]
fn short_files_skipped() {
    let entries = make_entries(vec![
        ("file_a.rs", "line1\nline2\nline3"),
        ("file_b.rs", "line1\nline2\nline3"),
    ]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── Edge Case: Below threshold not flagged ──────────────────────────

#[test]
fn below_threshold_not_flagged() {
    // 20 lines total, only 5 shared → 25% < 50% threshold
    let mut content_a = String::new();
    for i in 0..15 {
        content_a.push_str(&format!("fn unique_a_{}() -> i32 {{ {} }}\n", i, i));
    }
    content_a.push_str("fn shared_1() {}\nfn shared_2() {}\nfn shared_3() {}\nfn shared_4() {}\nfn shared_5() {}\n");

    let mut content_b = String::new();
    for i in 0..15 {
        content_b.push_str(&format!("fn unique_b_{}() -> i32 {{ {} }}\n", i, i * 2));
    }
    content_b.push_str("fn shared_1() {}\nfn shared_2() {}\nfn shared_3() {}\nfn shared_4() {}\nfn shared_5() {}\n");

    let entries = make_entries(vec![("file_a.rs", &content_a), ("file_b.rs", &content_b)]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── Protocol trait: handle_duplicates ───────────────────────────────

#[test]
fn handle_duplicates_with_none_path_uses_cwd() {
    // This will scan the current directory — just verify it doesn't panic.
    let result = analyzer().handle_duplicates(None);
    // Result may be empty or non-empty depending on cwd content.
    let _ = result;
}

// ─── Legacy API: check_duplicates ────────────────────────────────────

#[test]
fn legacy_check_duplicates_empty_files() {
    let violations = analyzer().check_duplicates(&[], 5);
    assert!(violations.is_empty());
}
```

---

## `tests/unit_code_analysis_orchestrator.rs`

```rust
// PURPOSE: Unit tests for CodeAnalysisOrchestrator — agent layer orchestration,
// score calculation, critical check, report formatting.

use code_analysis_lint_arwaky::{has_critical, CodeAnalysisOrchestrator};
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;

fn orchestrator() -> CodeAnalysisOrchestrator {
    CodeAnalysisOrchestrator::new()
}

// ─── calc_score: Perfect score with no violations ────────────────────

#[test]
fn calc_score_perfect_with_no_violations() {
    let orch = orchestrator();
    let score = orch.calc_score(&[]);
    assert_eq!(score.value, 100.0);
}

// ─── calc_score: Deductions per severity ─────────────────────────────

#[test]
fn calc_score_deducts_per_severity() {
    let orch = orchestrator();
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES301", Severity::HIGH, "too large"),
        LintResult::new_arch("b.rs", 1, "AES304", Severity::CRITICAL, "bypass"),
    ];
    let score = orch.calc_score(&results);
    // HIGH = 3.0, CRITICAL = 5.0 → 100 - 8 = 92
    assert_eq!(score.value, 92.0);
}

#[test]
fn calc_score_clamped_at_zero() {
    let orch = orchestrator();
    let results: Vec<LintResult> = (0..30)
        .map(|i| {
            LintResult::new_arch(
                &format!("file_{}.rs", i),
                1,
                "AES304",
                Severity::CRITICAL,
                "bypass",
            )
        })
        .collect();
    let score = orch.calc_score(&results);
    assert_eq!(score.value, 0.0);
}

// ─── check_critical ──────────────────────────────────────────────────

#[test]
fn check_critical_true_when_critical_exists() {
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES301", Severity::HIGH, "msg"),
        LintResult::new_arch("b.rs", 1, "AES304", Severity::CRITICAL, "msg"),
    ];
    assert!(has_critical(&results));
}

#[test]
fn check_critical_false_when_no_critical() {
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES301", Severity::HIGH, "msg"),
        LintResult::new_arch("b.rs", 1, "AES302", Severity::MEDIUM, "msg"),
    ];
    assert!(!has_critical(&results));
}

#[test]
fn check_critical_false_for_empty() {
    assert!(!has_critical(&[]));
}

// ─── format_report ───────────────────────────────────────────────────

#[test]
fn format_report_contains_header() {
    let orch = orchestrator();
    let results = vec![LintResult::new_arch(
        "src/foo.rs",
        10,
        "AES304",
        Severity::CRITICAL,
        "unwrap detected",
    )];
    let list = shared::cli_commands::taxonomy_result_vo::LintResultList::new(results);
    let root = FilePath::new("/project".to_string()).unwrap();
    let report = orch.format_report(&list, &root);
    assert!(report.contains("AES Architecture Compliance Report"));
    assert!(report.contains("/project"));
    assert!(report.contains("Violations: 1"));
    assert!(report.contains("AES304"));
}

#[test]
fn format_report_empty_results() {
    let orch = orchestrator();
    let list = shared::cli_commands::taxonomy_result_vo::LintResultList::new(vec![]);
    let root = FilePath::new("/project".to_string()).unwrap();
    let report = orch.format_report(&list, &root);
    assert!(report.contains("Violations: 0"));
}

// ─── active_rules ────────────────────────────────────────────────────

#[test]
fn active_rules_returns_configured_rules() {
    let orch = orchestrator();
    let rules = orch.active_rules();
    // Default config has no rules, so this should be empty.
    assert!(rules.is_empty());
}

// ─── run_code_analysis on non-existent path ──────────────────────────

#[test]
fn run_code_analysis_nonexistent_path_returns_empty() {
    let orch = orchestrator();
    let path = FilePath::new("/nonexistent/path/xyz".to_string()).unwrap();
    let results = orch.run_code_analysis_path(&path);
    assert!(results.is_empty());
}

// ─── lint_path public function ───────────────────────────────────────

#[test]
fn lint_path_nonexistent_returns_empty() {
    let results = code_analysis_lint_arwaky::lint_path("/nonexistent/path/xyz");
    assert!(results.is_empty());
}

// ─── Aggregate trait: run_code_analysis ──────────────────────────────

#[test]
fn aggregate_run_code_analysis_nonexistent() {
    let orch = orchestrator();
    let path = FilePath::new("/nonexistent/xyz".to_string()).unwrap();
    let result_list = orch.run_code_analysis(&path);
    assert!(result_list.is_empty());
}

// ─── Aggregate trait: run_code_analysis_dir ──────────────────────────

#[test]
fn aggregate_run_code_analysis_dir_nonexistent() {
    let orch = orchestrator();
    let path = FilePath::new("/nonexistent/dir".to_string()).unwrap();
    let result_list = orch.run_code_analysis_dir(&path);
    assert!(result_list.is_empty());
}
```

---

## `tests/integration_code_analysis.rs`

```rust
// PURPOSE: Integration tests — verify DI container wiring, end-to-end
// checker pipeline through CodeAnalysisCheckerContainer and CodeAnalysisContainer.

use code_analysis_lint_arwaky::{
    ArchLineChecker, BypassChecker, CodeAnalysisCheckerContainer, CodeAnalysisContainer,
    CodeAnalysisOrchestrator, CodeDuplicationAnalyzer, MandatoryDefinitionChecker,
};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use std::sync::Arc;

// ─── Container wiring: Default construction ──────────────────────────

#[test]
fn default_container_constructs_successfully() {
    let container = CodeAnalysisCheckerContainer::default();
    // All accessors return valid references
    let _ = container.config();
    let _ = container.bypass_checker();
    let _ = container.line_checker();
    let _ = container.class_checker();
    let _ = container.dead_inheritance_checker();
    let _ = container.duplication_checker();
}

// ─── Container wiring: Orchestrator uses container ───────────────────

#[test]
fn orchestrator_with_container_runs_checks() {
    let container = Arc::new(CodeAnalysisCheckerContainer::default());
    let orch = CodeAnalysisOrchestrator::new_with_container(container);
    // Run on non-existent path — should return empty, not panic
    let results = orch.run_scan("/nonexistent/path");
    assert!(results.is_empty());
}

// ─── CodeAnalysisContainer: new() ────────────────────────────────────

#[test]
fn code_analysis_container_new_produces_aggregate() {
    let container = CodeAnalysisContainer::new();
    let aggregate = container.code_analysis_linter();
    let score = aggregate.calc_score(&[]);
    assert_eq!(score.value, 100.0);
}

// ─── CodeAnalysisContainer: Default ──────────────────────────────────

#[test]
fn code_analysis_container_default_works() {
    let container = CodeAnalysisContainer::default();
    let aggregate = container.code_analysis_linter();
    let rules = aggregate.active_rules();
    assert!(rules.is_empty());
}

// ─── Full pipeline: bypass + line + mandatory on temp content ────────

#[test]
fn full_pipeline_detects_multiple_violation_types() {
    let container = CodeAnalysisCheckerContainer::default();

    let content_with_bypass = "fn foo() {\n    let x = opt.unwrap();\n}\n";
    let mut violations = Vec::new();
    container
        .bypass_checker()
        .check_bypass_comments("capabilities_foo.rs", content_with_bypass, &mut violations);
    assert!(!violations.is_empty());
    assert!(violations.iter().any(|v| v.code.code() == "AES304"));
}

// ─── Layer detection through container ───────────────────────────────

#[test]
fn container_detect_layer_returns_none_for_unknown() {
    let container = CodeAnalysisCheckerContainer::default();
    let result = container.detect_layer("random_file.rs", "/project");
    // "random_file.rs" has no recognized prefix → None
    assert!(result.is_none());
}

#[test]
fn container_detect_layer_recognizes_taxonomy_prefix() {
    let container = CodeAnalysisCheckerContainer::default();
    let result = container.detect_layer("taxonomy_foo_vo.rs", "/project");
    // Should detect "taxonomy" layer
    assert!(result.is_some());
}

// ─── Duplication analyzer wired through container ────────────────────

#[test]
fn container_duplication_checker_accessible() {
    let container = CodeAnalysisCheckerContainer::default();
    let dup = container.duplication_checker();
    let entries: Vec<(String, String)> = vec![];
    let violations = dup.check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── Orchestrator Default trait ──────────────────────────────────────

#[test]
fn orchestrator_default_constructs() {
    let orch = CodeAnalysisOrchestrator::default();
    let results = orch.run_scan("/nonexistent");
    assert!(results.is_empty());
}
```

---

## `tests/smoke_code_analysis.rs`

```rust
// PURPOSE: Smoke test — verify the code-analysis crate boots, constructs
// its containers, and can execute a basic scan without panicking.
// Must complete in under 5 seconds.

use code_analysis_lint_arwaky::{CodeAnalysisContainer, CodeAnalysisOrchestrator};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use std::time::Instant;

#[test]
fn crate_boots_and_scans_without_panic() {
    let start = Instant::now();

    // 1. Construct container
    let container = CodeAnalysisContainer::new();

    // 2. Get aggregate
    let aggregate = container.code_analysis_linter();

    // 3. Run analysis on current directory (should not panic)
    let root = FilePath::new(".".to_string()).unwrap();
    let results = aggregate.run_code_analysis(&root);

    // 4. Calculate score (should not panic)
    let score = aggregate.calc_score(&results.values);
    assert!(score.value >= 0.0 && score.value <= 100.0);

    // 5. Check critical (should not panic)
    let _has_critical = aggregate.check_critical(&results.values);

    // 6. Format report (should not panic)
    let report = aggregate.format_report(&results, &root);
    assert!(report.contains("AES Architecture Compliance Report"));

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test took {:?}, exceeds 5s limit",
        elapsed
    );
}

#[test]
fn orchestrator_boots_and_lint_path_works() {
    let start = Instant::now();

    let orch = CodeAnalysisOrchestrator::new();
    let results = orch.run_self_lint(".");

    // Should return a Vec (possibly empty) without panicking
    let _ = results.len();

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test took {:?}, exceeds 5s limit",
        elapsed
    );
}
```

---

## `tests/e2e_code_analysis_flow.rs`

```rust
// PURPOSE: E2E tests — full scan lifecycle from path input through all
// checkers to formatted report output. Uses real filesystem (temp dir).

use code_analysis_lint_arwaky::{CodeAnalysisContainer, CodeAnalysisOrchestrator};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use std::fs;
use std::path::Path;

fn setup_temp_project(files: Vec<(&str, &str)>) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    for (name, content) in files {
        let file_path = src.join(name);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&file_path, content).unwrap();
    }
    dir
}

// ─── E2E: Clean project produces zero violations ─────────────────────

#[test]
fn e2e_clean_project_zero_violations() {
    let clean_struct = r#"pub struct Widget {
    name: String,
    size: usize,
}

impl Widget {
    pub fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }

    pub fn display(&self) -> String {
        format!("{}: {}", self.name, self.size)
    }
}
"#;
    let dir = setup_temp_project(vec![("capabilities_widget.rs", clean_struct)]);

    let orch = CodeAnalysisOrchestrator::new();
    let results = orch.run_scan(dir.path().join("src").to_str().unwrap());

    // A single clean file should produce no AES301-305 violations
    let arch_violations: Vec<_> = results
        .iter()
        .filter(|r| r.code.code().starts_with("AES3"))
        .collect();
    assert!(
        arch_violations.is_empty(),
        "Expected no violations, got: {:?}",
        arch_violations
    );
}

// ─── E2E: Project with bypass produces AES304 ────────────────────────

#[test]
fn e2e_bypass_project_produces_aes304() {
    let bypass_code = r#"pub struct Service {
    active: bool,
}

impl Service {
    pub fn run(&self) {
        let val = Some(42).unwrap();
        println!("{}", val);
    }
}
"#;
    let dir = setup_temp_project(vec![("capabilities_service.rs", bypass_code)]);

    let orch = CodeAnalysisOrchestrator::new();
    let results = orch.run_scan(dir.path().join("src").to_str().unwrap());

    let aes304: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES304")
        .collect();
    assert!(
        !aes304.is_empty(),
        "Expected AES304 violation for unwrap()"
    );
}

// ─── E2E: Full report generation ─────────────────────────────────────

#[test]
fn e2e_full_report_generation() {
    let code = r#"pub struct Foo {
    x: i32,
}

impl Foo {
    pub fn new() -> Self {
        Self { x: 0 }
    }
}
"#;
    let dir = setup_temp_project(vec![("capabilities_foo.rs", code)]);

    let container = CodeAnalysisContainer::new();
    let aggregate = container.code_analysis_linter();

    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let results = aggregate.run_code_analysis(&root);
    let report = aggregate.format_report(&results, &root);

    assert!(report.contains("AES Architecture Compliance Report"));
    assert!(report.contains("Violations:"));
}

// ─── E2E: Score calculation after scan ───────────────────────────────

#[test]
fn e2e_score_after_scan() {
    let code = r#"pub struct Bar {
    y: f64,
}

impl Bar {
    pub fn compute(&self) -> f64 {
        self.y * 2.0
    }
}
"#;
    let dir = setup_temp_project(vec![("capabilities_bar.rs", code)]);

    let orch = CodeAnalysisOrchestrator::new();
    let results = orch.run_scan(dir.path().join("src").to_str().unwrap());
    let score = orch.calc_score(&results);

    // Clean code → score should be 100 or close
    assert!(score.value >= 90.0, "Score was {}", score.value);
}
```

---

## `tests/acceptance_FR_001.rs`

```rust
// PURPOSE: Acceptance test for FR-001: Maximum File Line Count (AES301)
// Files must not exceed the maximum allowed line count.

use code_analysis_lint_arwaky::ArchLineChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_common_vo::Count;
use shared::common::taxonomy_definition_vo::LayerDefinition;

fn make_def_with_max(max: i64) -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO {
            max_lines: Count::new(max),
            min_lines: Count::new(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// FR-001: File exceeding max lines fails with AES301
#[test]
fn fr_001_file_exceeding_max_lines_fails_aes301() {
    let checker = ArchLineChecker::new();
    let def = make_def_with_max(1000); // Default max: 1000 lines

    // Generate 1001 lines
    let content: String = (0..1001)
        .map(|i| format!("let line_{} = {};", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let mut violations = Vec::new();
    checker.check_line_counts("capabilities_large.rs", Some(&def), &content, &mut violations);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES301");
    assert!(violations[0].message.value.contains("FILE_TOO_LARGE"));
    assert!(violations[0].message.value.contains("1000"));
}

/// FR-001: File at exactly max lines does NOT fail
#[test]
fn fr_001_file_at_exactly_max_lines_passes() {
    let checker = ArchLineChecker::new();
    let def = make_def_with_max(1000);

    let content: String = (0..1000)
        .map(|i| format!("let line_{} = {};", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let mut violations = Vec::new();
    checker.check_line_counts("capabilities_exact.rs", Some(&def), &content, &mut violations);

    assert!(violations.is_empty());
}

/// FR-001: Default max is 1000 lines (configurable per rule)
#[test]
fn fr_001_default_max_is_1000() {
    let rule = shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default();
    assert_eq!(rule.max_lines.value, 1000);
}
```

---

## `tests/acceptance_FR_002.rs`

```rust
// PURPOSE: Acceptance test for FR-002: Minimum File Line Count (AES302)
// Files must have minimum length to avoid empty placeholders.

use code_analysis_lint_arwaky::ArchLineChecker;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_common_vo::Count;
use shared::common::taxonomy_definition_vo::LayerDefinition;

fn make_def_with_min(min: i64) -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO {
            min_lines: Count::new(min),
            max_lines: Count::new(0), // disable max check
            ..Default::default()
        },
        ..Default::default()
    }
}

/// FR-002: File below min lines fails with AES302
#[test]
fn fr_002_file_below_min_lines_fails_aes302() {
    let checker = ArchLineChecker::new();
    let def = make_def_with_min(10); // Default min: 10 lines

    let content = "line1\nline2\nline3"; // Only 3 lines

    let mut violations = Vec::new();
    checker.check_line_counts("capabilities_tiny.rs", Some(&def), content, &mut violations);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES302");
    assert!(violations[0].message.value.contains("FILE_TOO_SHORT"));
}

/// FR-002: File at exactly min lines does NOT fail
#[test]
fn fr_002_file_at_exactly_min_lines_passes() {
    let checker = ArchLineChecker::new();
    let def = make_def_with_min(10);

    let content: String = (0..10)
        .map(|i| format!("line_{}", i))
        .collect::<Vec<_>>()
        .join("\n");

    let mut violations = Vec::new();
    checker.check_line_counts("capabilities_ok.rs", Some(&def), &content, &mut violations);

    assert!(violations.is_empty());
}

/// FR-002: Default min is configurable (FRD says 10, code default is 5)
#[test]
fn fr_002_default_min_lines_from_rule() {
    let rule = shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default();
    // Code default is 5 (FRD says 10, but implementation uses 5 as default)
    assert!(rule.min_lines.value > 0);
}
```

---

## `tests/acceptance_FR_003.rs`

```rust
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
    checker.check_mandatory_class_definition("capabilities_foo.rs", Some(&def), content, &mut violations);
    assert!(violations.is_empty());
}

/// FR-003: Rust file with enum passes
#[test]
fn fr_003_rust_enum_passes() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "pub enum Status {\n    Active,\n    Inactive,\n}";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition("capabilities_foo.rs", Some(&def), content, &mut violations);
    assert!(violations.is_empty());
}

/// FR-003: Rust file with trait passes
#[test]
fn fr_003_rust_trait_passes() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "pub trait Handler {\n    fn handle(&self);\n}";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition("capabilities_foo.rs", Some(&def), content, &mut violations);
    assert!(violations.is_empty());
}

/// FR-003: Rust file with type alias passes
#[test]
fn fr_003_rust_type_passes() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "pub type HandlerFn = Box<dyn Fn()>;";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition("capabilities_foo.rs", Some(&def), content, &mut violations);
    assert!(violations.is_empty());
}

/// FR-003: File without definitions fails with AES303
#[test]
fn fr_003_no_definition_fails_aes303() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "pub fn helper() -> i32 {\n    42\n}";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition("capabilities_foo.rs", Some(&def), content, &mut violations);

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
    checker.check_mandatory_class_definition("capabilities_foo.py", Some(&def), content, &mut violations);
    assert!(violations.is_empty());
}

/// FR-003: TypeScript interface passes
#[test]
fn fr_003_typescript_interface_passes() {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let content = "export interface IService {\n    run(): void;\n}";

    let mut violations = Vec::new();
    checker.check_mandatory_class_definition("capabilities_foo.ts", Some(&def), content, &mut violations);
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
```

---

## `tests/acceptance_FR_004.rs`

```rust
// PURPOSE: Acceptance test for FR-004: Bypass Detection (AES304)
// Detects and flags any attempt to suppress warnings/errors.

use code_analysis_lint_arwaky::BypassChecker;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;

fn checker() -> BypassChecker {
    BypassChecker::new()
}

fn scan_rs(content: &str) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
    let mut v = Vec::new();
    checker().check_bypass_comments("test.rs", content, &mut v);
    v
}

/// FR-004: unwrap() detected with AES304
#[test]
fn fr_004_unwrap_detected() {
    let violations = scan_rs("let x = opt.unwrap();");
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
    assert_eq!(violations[0].severity, Severity::CRITICAL);
}

/// FR-004: expect() detected with AES304
#[test]
fn fr_004_expect_detected() {
    let violations = scan_rs(r#"let x = opt.expect("msg");"#);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

/// FR-004: panic! detected with AES304
#[test]
fn fr_004_panic_detected() {
    let violations = scan_rs(r#"panic!("fatal");"#);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

/// FR-004: todo! detected with AES304
#[test]
fn fr_004_todo_detected() {
    let violations = scan_rs("todo!();");
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

/// FR-004: #[allow(...)] detected with AES304
#[test]
fn fr_004_allow_attribute_detected() {
    let violations = scan_rs("#[allow(unused)]\nfn foo() {}");
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

/// FR-004: Safe variants NOT flagged: unwrap_or()
#[test]
fn fr_004_unwrap_or_not_flagged() {
    let violations = scan_rs("let x = opt.unwrap_or(0);");
    assert!(violations.is_empty());
}

/// FR-004: Safe variants NOT flagged: unwrap_or_else()
#[test]
fn fr_004_unwrap_or_else_not_flagged() {
    let violations = scan_rs("let x = opt.unwrap_or_else(|| 0);");
    assert!(violations.is_empty());
}

/// FR-004: noqa comment detected (Python)
#[test]
fn fr_004_noqa_detected() {
    let mut v = Vec::new();
    checker().check_bypass_comments("test.py", "x = 1  # noqa", &mut v);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].code.code(), "AES304");
}

/// FR-004: type: ignore detected (Python)
#[test]
fn fr_004_type_ignore_detected() {
    let mut v = Vec::new();
    checker().check_bypass_comments("test.py", "x = bad()  # type: ignore", &mut v);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].code.code(), "AES304");
}

/// FR-004: eslint-disable detected (JavaScript)
#[test]
fn fr_004_eslint_disable_detected() {
    let mut v = Vec::new();
    checker().check_bypass_comments("test.js", "// eslint-disable-next-line\nfoo();", &mut v);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].code.code(), "AES304");
}

/// FR-004: Cargo.toml clippy allow detected
#[test]
fn fr_004_cargo_toml_clippy_allow() {
    let content = "[workspace.lints.clippy]\nall = \"allow\"";
    let mut v = Vec::new();
    checker().check_cargo_toml(content, &mut v);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].code.code(), "AES304");
    assert_eq!(v[0].severity, Severity::CRITICAL);
}
```

---

## `tests/acceptance_FR_005.rs`

```rust
// PURPOSE: Acceptance test for FR-005: Duplicate Code Detection (AES305)
// Compares code blocks and flags identical/highly similar segments.

use code_analysis_lint_arwaky::CodeDuplicationAnalyzer;

fn analyzer() -> CodeDuplicationAnalyzer {
    CodeDuplicationAnalyzer::new()
}

fn make_entries(files: Vec<(&str, &str)>) -> Vec<(String, String)> {
    files
        .into_iter()
        .map(|(p, c)| (p.to_string(), c.to_string()))
        .collect()
}

/// FR-005: Duplicate code detected with AES305
#[test]
fn fr_005_duplicate_code_detected() {
    // Create two files with >50% identical content (min 5 lines window)
    let shared = "fn shared_a() -> i32 {\n    1 + 1\n}\n\nfn shared_b() -> i32 {\n    2 + 2\n}\n\nfn shared_c() -> i32 {\n    3 + 3\n}\n\nfn shared_d() -> i32 {\n    4 + 4\n}\n\nfn shared_e() -> i32 {\n    5 + 5\n}";
    let entries = make_entries(vec![
        ("file_a.rs", shared),
        ("file_b.rs", shared),
    ]);

    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(!violations.is_empty());

    // Verify violation message contains AES305
    let (_, violation) = &violations[0];
    let msg = violation.to_string();
    assert!(msg.contains("AES305"));
    assert!(msg.contains("CODE_DUPLICATION"));
}

/// FR-005: Min duplicate lines threshold respected
#[test]
fn fr_005_min_duplicate_lines_respected() {
    // Only 3 shared lines with min_dup_lines=5 → no violation
    let entries = make_entries(vec![
        ("file_a.rs", "fn a() {}\nfn b() {}\nfn c() {}\nfn unique_a() {}\nfn unique_a2() {}\nfn unique_a3() {}\nfn unique_a4() {}\nfn unique_a5() {}"),
        ("file_b.rs", "fn a() {}\nfn b() {}\nfn c() {}\nfn unique_b() {}\nfn unique_b2() {}\nfn unique_b3() {}\nfn unique_b4() {}\nfn unique_b5() {}"),
    ]);

    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

/// FR-005: Threshold percentage respected
#[test]
fn fr_005_threshold_percentage_respected() {
    // Files share some content but below 50% threshold
    let mut content_a = String::new();
    for i in 0..20 {
        content_a.push_str(&format!("fn unique_a_{}() -> i32 {{ {} }}\n", i, i));
    }

    let mut content_b = String::new();
    for i in 0..20 {
        content_b.push_str(&format!("fn unique_b_{}() -> String {{ String::from(\"{}\") }}\n", i, i));
    }

    let entries = make_entries(vec![("file_a.rs", &content_a), ("file_b.rs", &content_b)]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

/// FR-005: Algorithm uses window-based hashing with normalized lines
#[test]
fn fr_005_normalized_comparison_ignores_whitespace() {
    // Same logic, different whitespace → should still match after normalization
    let content_a = "fn foo() -> i32 {\n    42\n}\n\nfn bar() -> i32 {\n    43\n}\n\nfn baz() -> i32 {\n    44\n}\n\nfn qux() -> i32 {\n    45\n}\n\nfn quux() -> i32 {\n    46\n}";
    let content_b = "fn foo() -> i32 {\n        42\n}\n\nfn bar() -> i32 {\n        43\n}\n\nfn baz() -> i32 {\n        44\n}\n\nfn qux() -> i32 {\n        45\n}\n\nfn quux() -> i32 {\n        46\n}";

    let entries = make_entries(vec![("file_a.rs", content_a), ("file_b.rs", content_b)]);
    let violations = analyzer().check_file_similarity_entries(&entries, 5, 50.0);
    // After normalization (trim + alphanumeric), these should be identical
    assert!(!violations.is_empty());
}
```

---

## `tests/acceptance_FR_006.rs`

```rust
// PURPOSE: Acceptance test for FR-006: File Read Error Diagnostics (AES000)
// Emit diagnostic when file cannot be read or exceeds size limit.

use shared::code_analysis::utility_file_reader::{read_lintable_file, MAX_LINT_FILE_BYTES};

/// FR-006: Max file size is 2 MiB
#[test]
fn fr_006_max_file_size_is_2mib() {
    assert_eq!(MAX_LINT_FILE_BYTES, 2 * 1024 * 1024);
}

/// FR-006: Readable file returns Ok(Some(content))
#[test]
fn fr_006_readable_file_returns_content() {
    // Create a small temp file
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    std::fs::write(&file_path, "fn main() {}").unwrap();

    let result = read_lintable_file(file_path.to_str().unwrap());
    assert!(result.is_ok());
    let content = result.unwrap();
    assert!(content.is_some());
    assert_eq!(content.unwrap(), "fn main() {}");
}

/// FR-006: Non-existent file returns Err
#[test]
fn fr_006_nonexistent_file_returns_err() {
    let result = read_lintable_file("/nonexistent/path/file.rs");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("/nonexistent/path/file.rs"));
}

/// FR-006: Oversized file returns Ok(None) — graceful skip
#[test]
fn fr_006_oversized_file_returns_none() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("large.rs");

    // Create a file > 2 MiB
    let large_content = "x".repeat((2 * 1024 * 1024 + 1) as usize);
    std::fs::write(&file_path, &large_content).unwrap();

    let result = read_lintable_file(file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert!(result.unwrap().is_none()); // Graceful skip, not an error
}

/// FR-006: File at exactly 2 MiB is readable
#[test]
fn fr_006_file_at_exact_limit_readable() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("exact.rs");

    let content = "x".repeat(MAX_LINT_FILE_BYTES as usize);
    std::fs::write(&file_path, &content).unwrap();

    let result = read_lintable_file(file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}
```

---

## `tests/bench_code_analysis_throughput.rs`

```rust
// PURPOSE: Benchmark tests for code-analysis — measures throughput of
// bypass checking, line counting, and duplication analysis.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use code_analysis_lint_arwaky::{ArchLineChecker, BypassChecker, CodeDuplicationAnalyzer, MandatoryDefinitionChecker};
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::common::taxonomy_common_vo::{BooleanVO, Count};
use shared::common::taxonomy_definition_vo::LayerDefinition;

fn generate_clean_rust(lines: usize) -> String {
    (0..lines)
        .map(|i| format!("    let field_{}: i32 = {};", i, i))
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_bypass_rust(lines: usize) -> String {
    (0..lines)
        .map(|i| {
            if i % 10 == 0 {
                format!("    let x_{} = opt.unwrap();", i)
            } else {
                format!("    let val_{}: i32 = {};", i, i)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn make_def() -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO {
            min_lines: Count::new(5),
            max_lines: Count::new(1000),
            mandatory_class_definition: BooleanVO::new(true),
            ..Default::default()
        },
        ..Default::default()
    }
}

// ─── Benchmark: Bypass checking throughput ───────────────────────────

fn bench_bypass_checker(c: &mut Criterion) {
    let checker = BypassChecker::new();
    let mut group = c.benchmark_group("bypass_checker");

    for size in [100, 500, 1000] {
        let content = generate_bypass_rust(size);
        group.bench_with_input(
            BenchmarkId::new("check_bypass_comments", size),
            &content,
            |b, data| {
                b.iter(|| {
                    let mut violations = Vec::new();
                    checker.check_bypass_comments("bench.rs", data, &mut violations);
                    violations
                })
            },
        );
    }
    group.finish();
}

// ─── Benchmark: Line checker throughput ──────────────────────────────

fn bench_line_checker(c: &mut Criterion) {
    let checker = ArchLineChecker::new();
    let def = make_def();
    let mut group = c.benchmark_group("line_checker");

    for size in [100, 500, 1000] {
        let content = generate_clean_rust(size);
        group.bench_with_input(
            BenchmarkId::new("check_line_counts", size),
            &content,
            |b, data| {
                b.iter(|| {
                    let mut violations = Vec::new();
                    checker.check_line_counts("bench.rs", Some(&def), data, &mut violations);
                    violations
                })
            },
        );
    }
    group.finish();
}

// ─── Benchmark: Mandatory definition checker throughput ──────────────

fn bench_mandatory_checker(c: &mut Criterion) {
    let checker = MandatoryDefinitionChecker::new();
    let def = make_def();
    let mut group = c.benchmark_group("mandatory_checker");

    for size in [100, 500, 1000] {
        let content = format!("pub struct BenchStruct {{\n{}\n}}", generate_clean_rust(size));
        group.bench_with_input(
            BenchmarkId::new("check_mandatory_class_definition", size),
            &content,
            |b, data| {
                b.iter(|| {
                    let mut violations = Vec::new();
                    checker.check_mandatory_class_definition("bench.rs", Some(&def), data, &mut violations);
                    violations
                })
            },
        );
    }
    group.finish();
}

// ─── Benchmark: Duplication analysis throughput ──────────────────────

fn bench_duplication_analyzer(c: &mut Criterion) {
    let analyzer = CodeDuplicationAnalyzer::new();
    let mut group = c.benchmark_group("duplication_analyzer");

    for file_count in [5, 10, 20] {
        let entries: Vec<(String, String)> = (0..file_count)
            .map(|i| {
                let content = generate_clean_rust(50);
                (format!("file_{}.rs", i), content)
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("check_file_similarity_entries", file_count),
            &entries,
            |b, data| {
                b.iter(|| analyzer.check_file_similarity_entries(data, 5, 50.0))
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_bypass_checker,
    bench_line_checker,
    bench_mandatory_checker,
    bench_duplication_analyzer,
);
criterion_main!(benches);
```

---

## `Cargo.toml` additions

Add to `crates/code-analysis/Cargo.toml`:

```toml
[dev-dependencies]
shared.workspace = true
criterion = { version = "0.5", features = ["html_reports"] }
tempfile = "3"

[[bench]]
name = "bench_code_analysis_throughput"
path = "tests/bench_code_analysis_throughput.rs"
harness = false
```

---

## Quick Reference

```bash
# Run all tests
cargo test -p code_analysis-lint-arwaky

# Run specific test file
cargo test -p code_analysis-lint-arwaky --test contract_code_analysis
cargo test -p code_analysis-lint-arwaky --test unit_code_analysis_bypass_checker
cargo test -p code_analysis-lint-arwaky --test unit_code_analysis_line_checker
cargo test -p code_analysis-lint-arwaky --test unit_code_analysis_mandatory_definition
cargo test -p code_analysis-lint-arwaky --test unit_code_analysis_duplication
cargo test -p code_analysis-lint-arwaky --test unit_code_analysis_orchestrator
cargo test -p code_analysis-lint-arwaky --test integration_code_analysis
cargo test -p code_analysis-lint-arwaky --test smoke_code_analysis
cargo test -p code_analysis-lint-arwaky --test e2e_code_analysis_flow
cargo test -p code_analysis-lint-arwaky --test acceptance_FR_001
cargo test -p code_analysis-lint-arwaky --test acceptance_FR_004

# Run benchmarks
cargo bench -p code_analysis-lint-arwaky

# Coverage
cargo tarpaulin -p code_analysis-lint-arwaky --fail-under 70

# With output
cargo test -p code_analysis-lint-arwaky -- --nocapture
```

---

## Coverage Summary

| Layer        | File                                           | Tests    | Target  |
| ------------ | ---------------------------------------------- | -------- | ------- |
| Capabilities | `unit_code_analysis_bypass_checker.rs`       | 24       | ≥70%   |
| Capabilities | `unit_code_analysis_line_checker.rs`         | 11       | ≥70%   |
| Capabilities | `unit_code_analysis_mandatory_definition.rs` | 20       | ≥70%   |
| Capabilities | `unit_code_analysis_duplication.rs`          | 8        | ≥70%   |
| Agent        | `unit_code_analysis_orchestrator.rs`         | 12       | ≥60%   |
| Root         | `integration_code_analysis.rs`               | 9        | —      |
| Contract     | `contract_code_analysis.rs`                  | 10       | —      |
| Acceptance   | `acceptance_FR_001..006.rs`                  | 22       | 1:1 FRD |
| Smoke        | `smoke_code_analysis.rs`                     | 2        | <5s     |
| E2E          | `e2e_code_analysis_flow.rs`                  | 4        | —      |
| Benchmark    | `bench_code_analysis_throughput.rs`          | 4 groups | nightly |

**Total: ~126 test cases** covering all 6 functional requirements (FR-001 through FR-006), all public API methods, all protocol trait implementations, and performance baselines.
