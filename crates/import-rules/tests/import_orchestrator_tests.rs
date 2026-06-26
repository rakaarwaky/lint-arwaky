use import_rules_lint_arwaky::agent_import_orchestrator::str_or;

#[test]
fn str_or_returns_value_when_some() {
    assert_eq!(str_or(Some("hello"), "fallback"), "hello");
}

#[test]
fn str_or_returns_fallback_when_none() {
    assert_eq!(str_or(None, "fallback"), "fallback");
}

#[test]
fn str_or_works_with_empty_strings() {
    assert_eq!(str_or(Some(""), "fallback"), "");
    assert_eq!(str_or(None, ""), "");
}
