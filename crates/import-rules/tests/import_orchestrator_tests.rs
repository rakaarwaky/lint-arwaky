#[test]
fn str_or_returns_value_when_some() {
    assert_eq!(Some("hello").unwrap_or("fallback"), "hello");
}

#[test]
fn str_or_returns_fallback_when_none() {
    let s: Option<&str> = None;
    assert_eq!(s.unwrap_or("fallback"), "fallback");
}

#[test]
fn str_or_works_with_empty_strings() {
    assert_eq!(Some("").unwrap_or("fallback"), "");
    let s: Option<&str> = None;
    assert_eq!(s.unwrap_or(""), "");
}
