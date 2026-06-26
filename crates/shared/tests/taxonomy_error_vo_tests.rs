use shared_lint_arwaky::common::taxonomy_error_vo::ErrorCode;

#[test]
fn test_error_code_new() {
    let ec = ErrorCode::new("E123").unwrap_or_default();
    assert_eq!(ec.code(), "E123");
    assert!(ec.is_style());
    assert!(!ec.is_logic());
    assert!(!ec.is_security());
    assert!(!ec.is_architecture());

    let ec = ErrorCode::new("W999").unwrap_or_default();
    assert!(ec.is_style());

    let ec = ErrorCode::new("D404").unwrap_or_default();
    assert!(ec.is_style());

    let ec = ErrorCode::new("F001").unwrap_or_default();
    assert!(ec.is_logic());

    let ec = ErrorCode::new("I999").unwrap_or_default();
    assert!(ec.is_logic());

    let ec = ErrorCode::new("B001").unwrap_or_default();
    assert!(ec.is_security());

    let ec = ErrorCode::new("AES123").unwrap_or_default();
    assert!(ec.is_architecture());
}

#[test]
fn test_error_code_invalid() {
    assert!(ErrorCode::new("").is_err());
}
