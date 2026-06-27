use external_lint_lint_arwaky::infrastructure_rs_audit_adapter::CargoAuditAdapter;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_path_vo::FilePath;
use std::sync::Arc;

struct IdentityPathNorm;

impl IPathNormalizationPort for IdentityPathNorm {
    fn normalize_path(&self, path: FilePath) -> FilePath {
        path
    }
    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        _context_path: Option<FilePath>,
    ) -> FilePath {
        path
    }
}

fn make_adapter() -> CargoAuditAdapter {
    CargoAuditAdapter::new(Arc::new(IdentityPathNorm))
}

fn make_path(p: &str) -> FilePath {
    FilePath::new(p.to_string()).unwrap_or_default()
}

#[tokio::test]
async fn name_returns_cargo_audit() {
    let adapter = make_adapter();
    assert_eq!(adapter.name().value(), "cargo-audit");
}

#[tokio::test]
async fn scan_returns_empty_when_no_cargo_lock() {
    let adapter = make_adapter();
    let dir = std::env::temp_dir().join(format!("audit_nolock_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    // No Cargo.lock in this directory
    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert!(results.is_empty(), "expected no results for dir without Cargo.lock");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn scan_returns_empty_for_nonexistent_path() {
    let adapter = make_adapter();
    let path = make_path("/nonexistent/path/xyz_audit_test");
    let results = adapter.scan(&path).await.unwrap();
    assert!(results.is_empty());
}

#[tokio::test]
async fn scan_with_empty_cargo_lock_does_not_panic() {
    let adapter = make_adapter();
    let dir = std::env::temp_dir().join(format!("audit_empty_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    // Create empty Cargo.lock — parser will fail gracefully
    std::fs::write(dir.join("Cargo.lock"), "").unwrap();
    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert!(results.is_empty(), "expected empty results for empty Cargo.lock");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn apply_fix_returns_true() {
    let adapter = make_adapter();
    let path = make_path("Cargo.lock");
    let status = adapter.apply_fix(&path).await.unwrap();
    assert!(status.value(), "cargo-audit apply_fix should return true (noop)");
}
