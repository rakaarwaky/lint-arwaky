// Smoke tests — verify shared crate core VOs and contracts load within 5s.
#[test]
fn shared_common_vos_construct() {
    let start = std::time::Instant::now();
    let _fp = shared_lint_arwaky::common::FilePath::new("/test/path.rs".to_string()).unwrap();
    let _code = shared_lint_arwaky::common::ErrorCode::raw("AES101");
    let _sev = shared_lint_arwaky::common::Severity::MEDIUM;
    let _msg = shared_lint_arwaky::common::LintMessage::new("test".to_string());
    let _ln = shared_lint_arwaky::common::LineNumber::new(1);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn shared_config_system_vos_construct() {
    let start = std::time::Instant::now();
    let _lang = shared_lint_arwaky::config_system::ConfigLanguage::Rust;
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn shared_role_violation_vo_construct() {
    let start = std::time::Instant::now();
    let _violation = shared_lint_arwaky::role_rules::AesRoleViolation::ConstantPurity {
        reason: Some(shared_lint_arwaky::common::LintMessage::new("test".to_string())),
    };
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn shared_filesystem_vos_construct() {
    use std::path::PathBuf;
    let start = std::time::Instant::now();
    let _fe = shared_lint_arwaky::filesystem::FileEntry {
        path: PathBuf::from("src/main.rs"),
        extension: "rs".to_string(),
        language: shared_lint_arwaky::common::Language::Rust,
        size: 1024,
        content: String::new(),
        parse_ok: true,
        parse_metadata: None,
    };
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
