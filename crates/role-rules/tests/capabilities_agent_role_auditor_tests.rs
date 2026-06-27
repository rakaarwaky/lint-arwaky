use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn make_source(file: &str, content: &str, language: &str) -> SourceContentVO {
    let fp = FilePath::new(file.to_string()).unwrap_or_default();
    SourceContentVO::new(fp, ContentString::new(content.to_string()), language)
}

// ─── check_file_size_limit ──────────────────────────────────────────────────

#[test]
fn file_within_limit_no_violation() {
    let checker = AgentRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "agent_orchestrator.rs",
        "line1\nline2\nline3",
        "rust",
    );
    checker.check_file_size_limit(&src, 10, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn file_exceeding_limit_emits_violation() {
    let checker = AgentRoleChecker::new();
    let mut violations = Vec::new();
    let lines: Vec<&str> = (0..25).map(|_i| &"x\n"[..]).collect();
    let content = lines.join("");
    let src = make_source("agent_orchestrator.rs", &content, "rust");
    checker.check_file_size_limit(&src, 20, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.to_string().contains("AES405"));
}

#[test]
fn zero_max_lines_always_triggers() {
    let checker = AgentRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source("agent_tiny.rs", "just one line", "rust");
    checker.check_file_size_limit(&src, 0, &mut violations);
    assert_eq!(violations.len(), 1);
}

// ─── check_any_type_annotation ──────────────────────────────────────────────

#[test]
fn no_any_annotation_no_violation() {
    let checker = AgentRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "agent_container.rs",
        "let x: Foo = bar();\nfn helper(x: &str) -> bool { true }",
        "rust",
    );
    checker.check_any_type_annotation(&src, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn detects_colon_any_annotation() {
    let checker = AgentRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "agent_container.rs",
        "let x: any = bar();\nlet y: Any = baz();",
        "rust",
    );
    checker.check_any_type_annotation(&src, &mut violations);
    assert_eq!(violations.len(), 2);
    assert!(violations.iter().all(|v| v.code.to_string().contains("AES405")));
}

#[test]
fn detects_arrow_any_return_type() {
    let checker = AgentRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "agent_container.rs",
        "fn get_thing() -> any { todo!() }\nfn get_other() -> Any { todo!() }",
        "rust",
    );
    checker.check_any_type_annotation(&src, &mut violations);
    assert_eq!(violations.len(), 2);
}

#[test]
fn detects_any_generic() {
    let checker = AgentRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "agent_container.ts",
        "let x: Any<Foo> = bar();\nlet y: any[Bar] = baz();",
        "javascript",
    );
    checker.check_any_type_annotation(&src, &mut violations);
    assert_eq!(violations.len(), 2);
}

#[test]
fn multiple_lines_all_annotated() {
    let checker = AgentRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "agent_helper.rs",
        "fn f1() -> any {}\nfn f2(x: any) {}\nfn f3() -> Any {}",
        "rust",
    );
    checker.check_any_type_annotation(&src, &mut violations);
    assert_eq!(violations.len(), 3);
}
