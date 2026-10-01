// Smoke tests — verify shared crate core VOs and contracts load within 5s.
#[test]
fn shared_common_vos_construct() {
    let start = std::time::Instant::now();
    let _fp = shared_common::FilePath::new("/test/path.rs".to_string()).unwrap();
    let _code = shared_common::ErrorCode::raw("AES101");
    let _sev = shared_common::Severity::MEDIUM;
    let _msg = shared_common::LintMessage::new("test".to_string());
    let _ln = shared_common::LineNumber::new(1);
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
    let _lang = shared_config_system::ConfigLanguage::Rust;
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
    let _violation = shared_role_rules::AesRoleViolation::ConstantPurity {
        reason: Some(shared_common::LintMessage::new("test".to_string())),
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
    let _fe = shared_filesystem::FileEntry {
        path: PathBuf::from("src/main.rs"),
        extension: "rs".to_string(),
        language: shared_common::Language::Rust,
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
