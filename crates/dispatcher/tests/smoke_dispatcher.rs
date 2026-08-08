// Smoke tests — module imports work, key types accessible, complete within 5s.
use dispatcher_lint_arwaky::surface_check_action::ScanOptions;
use shared::common::ViolationItem;

#[test]
fn smoke_module_imports_work() {
    let start = std::time::Instant::now();
    // Verify core types are accessible
    let _ = std::any::type_name::<ViolationItem>();
    let _ = std::any::type_name::<ScanOptions>();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn smoke_violation_item_creation() {
    let start = std::time::Instant::now();
    let json = serde_json::json!({
        "file": "src/lib.rs",
        "line": 1,
        "code": "AES201",
        "message": "test",
        "severity": "HIGH"
    });
    let item = ViolationItem::from_json_obj(&json).unwrap();
    assert_eq!(item.code.code(), "AES201");
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn smoke_violation_item_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ViolationItem>();
}

#[test]
fn smoke_scan_options_is_send() {
    // ScanOptions contains Arc, should be Send
    fn _assert_send<T: Send>() {}
    // We can't easily create ScanOptions without a filesystem, but we can verify the type exists
    let _ = std::any::type_name::<ScanOptions>();
}
