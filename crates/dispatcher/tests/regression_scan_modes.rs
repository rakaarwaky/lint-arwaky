// Regression tests — single file, subfolder, and workspace scan modes
// for all scanners across Rust, Python, and TypeScript workspaces-bad fixtures.
//
// Two test strategies:
// 1. collect_scan (in-process) — tests workspaces-good (false positive tests)
// 2. CLI subprocess — tests workspaces-bad via `cargo run --release --bin lint-arwaky-cli`
//
// Prevents regressions in: tracing→stderr fix, workspace root detection,
// path normalization, member filtering, single file scan.
use shared::common::taxonomy_path_vo::FilePath;
use std::process::Command;

fn fs()
-> std::sync::Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate> {
    filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()
}

/// Resolve workspace root from CARGO_MANIFEST_DIR (crates/<name>/ → project root).
fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("."))
}

/// In-process scan via collect_scan (works for workspaces-good where 0 violations expected).
fn scan(path: &str) -> Vec<shared::common::ViolationItem> {
    let full_path = workspace_root().join(path);
    let opts = dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(FilePath::new(full_path.to_string_lossy().to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: fs(),
    };
    dispatcher_lint_arwaky::surface_check_action::collect_scan(opts).unwrap_or_default()
}

/// CLI subprocess scan via release binary (for workspaces-bad where violations expected).
fn cli_scan(path: &str) -> String {
    let exe = std::env::current_exe()
        .ok()
        .and_then(|p| {
            p.parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .map(|p| p.join("release/lint-arwaky-cli"))
        })
        .unwrap_or_else(|| std::path::PathBuf::from("target/release/lint-arwaky-cli"));
    let full_path = workspace_root().join(path);
    let output = Command::new(&exe)
        .args([
            "scan",
            full_path.to_str().unwrap_or(path),
            "--format",
            "json",
        ])
        .output()
        .unwrap_or_else(|e| {
            panic!(
                "failed to run CLI at {}: {}. Build with: cargo build --release",
                exe.display(),
                e
            )
        });
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn count_violations(json: &str) -> usize {
    serde_json::from_str::<serde_json::Value>(json)
        .ok()
        .and_then(|v| v.get("total_violations").and_then(|n| n.as_u64()))
        .map(|n| n as usize)
        .unwrap_or(0)
}

fn has_violation_code(json: &str, code: &str) -> bool {
    let val: serde_json::Value = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(_) => return false,
    };
    val.get("results")
        .and_then(|r| r.as_array())
        .map(|results| {
            results
                .iter()
                .any(|r| r.get("code").and_then(|c| c.as_str()) == Some(code))
        })
        .unwrap_or(false)
}

// ═══════════════════════════════════════════════════════════════
// In-process: workspaces-good (false positive — must be 0)
// ═══════════════════════════════════════════════════════════════

#[test]
fn regression_good_rust_single_file() {
    let v = scan("workspaces-good/crates/calculator/src/agent_calculator_orchestrator.rs");
    let naming: Vec<_> = v
        .iter()
        .filter(|r| r.code.code().starts_with("AES10"))
        .collect();
    assert!(
        naming.is_empty(),
        "workspaces-good Rust naming must be 0, got {}",
        naming.len()
    );
}

#[test]
fn regression_good_rust_subfolder() {
    let v = scan("workspaces-good/crates/calculator");
    let naming: Vec<_> = v
        .iter()
        .filter(|r| r.code.code().starts_with("AES10"))
        .collect();
    assert!(
        naming.is_empty(),
        "workspaces-good Rust naming subfolder must be 0, got {}",
        naming.len()
    );
}

#[test]
fn regression_good_python_single_file() {
    let v = scan("workspaces-good/modules/addition/src/capabilities_addition_analyzer.py");
    let naming: Vec<_> = v
        .iter()
        .filter(|r| r.code.code().starts_with("AES10"))
        .collect();
    assert!(
        naming.is_empty(),
        "workspaces-good Python naming must be 0, got {}",
        naming.len()
    );
}

#[test]
fn regression_good_python_subfolder() {
    let v = scan("workspaces-good/modules/addition");
    let naming: Vec<_> = v
        .iter()
        .filter(|r| r.code.code().starts_with("AES10"))
        .collect();
    assert!(
        naming.is_empty(),
        "workspaces-good Python naming subfolder must be 0, got {}",
        naming.len()
    );
}

#[test]
fn regression_good_typescript_single_file() {
    let v = scan("workspaces-good/packages/calculator/src/capabilities_calculator_analyzer.ts");
    let naming: Vec<_> = v
        .iter()
        .filter(|r| r.code.code().starts_with("AES10"))
        .collect();
    assert!(
        naming.is_empty(),
        "workspaces-good TS naming must be 0, got {}",
        naming.len()
    );
}

#[test]
fn regression_good_typescript_subfolder() {
    let v = scan("workspaces-good/packages/calculator");
    let naming: Vec<_> = v
        .iter()
        .filter(|r| r.code.code().starts_with("AES10"))
        .collect();
    assert!(
        naming.is_empty(),
        "workspaces-good TS naming subfolder must be 0, got {}",
        naming.len()
    );
}

// ═══════════════════════════════════════════════════════════════
// CLI subprocess: workspaces-bad (must detect violations)
// ═══════════════════════════════════════════════════════════════

#[test]
fn regression_bad_rust_single_file() {
    let json = cli_scan("workspaces-bad/crates/naming_violations/src/capabilities_user_vo.rs");
    assert!(
        has_violation_code(&json, "AES102"),
        "must detect AES102, json: {}",
        &json[..json.len().min(200)]
    );
}

#[test]
fn regression_bad_rust_subfolder() {
    let json = cli_scan("workspaces-bad/crates/naming_violations");
    assert!(
        count_violations(&json) >= 20,
        "Rust subfolder >=20, got {}",
        count_violations(&json)
    );
}

#[test]
fn regression_bad_rust_workspace() {
    let json = cli_scan("workspaces-bad/crates");
    assert!(
        count_violations(&json) >= 100,
        "Rust workspace >=100, got {}",
        count_violations(&json)
    );
}

#[test]
fn regression_bad_python_single_file() {
    let json = cli_scan("workspaces-bad/modules/naming_violations/src/capabilities_user_vo.py");
    assert!(has_violation_code(&json, "AES102"), "must detect AES102");
}

#[test]
fn regression_bad_python_subfolder() {
    let json = cli_scan("workspaces-bad/modules/naming_violations");
    assert!(
        count_violations(&json) >= 20,
        "Python subfolder >=20, got {}",
        count_violations(&json)
    );
}

#[test]
fn regression_bad_python_workspace() {
    let json = cli_scan("workspaces-bad/modules");
    assert!(
        count_violations(&json) >= 100,
        "Python workspace >=100, got {}",
        count_violations(&json)
    );
}

#[test]
fn regression_bad_typescript_single_file() {
    let json = cli_scan("workspaces-bad/packages/naming_violations/src/capabilities_user_vo.ts");
    assert!(has_violation_code(&json, "AES102"), "must detect AES102");
}

#[test]
fn regression_bad_typescript_subfolder() {
    let json = cli_scan("workspaces-bad/packages/naming_violations");
    assert!(
        count_violations(&json) >= 20,
        "TS subfolder >=20, got {}",
        count_violations(&json)
    );
}

#[test]
fn regression_bad_typescript_workspace() {
    let json = cli_scan("workspaces-bad/packages");
    assert!(
        count_violations(&json) >= 100,
        "TS workspace >=100, got {}",
        count_violations(&json)
    );
}

// ═══════════════════════════════════════════════════════════════
// External member-dirs filter (regression: setup.py at workspace root)
// ═══════════════════════════════════════════════════════════════

fn violation_for(path: &str) -> shared::common::ViolationItem {
    shared::common::ViolationItem {
        code: shared::common::taxonomy_error_vo::ErrorCode::raw("B307"),
        file: FilePath::new(path.to_string()).unwrap(),
        line: shared::common::taxonomy_common_vo::LineNumber::new(1),
        column: shared::common::taxonomy_common_vo::ColumnNumber::new(1),
        message: shared::common::taxonomy_message_vo::LintMessage::new("test"),
        severity: shared::common::taxonomy_severity_vo::Severity::MEDIUM,
    }
}

#[test]
fn regression_external_filter_keeps_member_files_drops_root_files() {
    let ws = workspace_root().join("workspaces-good");
    let ws_str = ws.to_string_lossy().to_string();

    let mut violations = vec![
        // Root-level file (e.g. setup.py) — must be dropped.
        violation_for(&format!("{}/setup.py", ws_str)),
        // Inside a member dir — must be kept.
        violation_for(&format!(
            "{}/modules/addition/src/capabilities_addition_analyzer.py",
            ws_str
        )),
        // Inside another member dir — must be kept.
        violation_for(&format!(
            "{}/crates/calculator/src/agent_calculator_orchestrator.rs",
            ws_str
        )),
        // Inside the third member dir — must be kept.
        violation_for(&format!(
            "{}/packages/calculator/src/capabilities_calculator_analyzer.ts",
            ws_str
        )),
    ];

    dispatcher_lint_arwaky::surface_external_action::filter_outside_member_dirs(
        &mut violations,
        &ws_str,
        fs().as_ref(),
    );

    assert_eq!(violations.len(), 3, "got: {:?}", violations);
    assert!(
        violations.iter().all(|v| {
            let p = std::path::Path::new(&v.file.value);
            ["modules", "crates", "packages"]
                .iter()
                .any(|d| p.components().any(|c| c.as_os_str() == *d))
        }),
        "only member-dir files must remain, got: {:?}",
        violations
    );
}

#[test]
fn regression_external_filter_noop_outside_workspace() {
    // A non-workspace path (no crates/packages/modules above it) must be unfiltered.
    let scratch = std::env::temp_dir();
    let target = scratch.join("lint-arwaky-non-workspace");
    let _ = std::fs::create_dir_all(&target);
    let target_str = target.to_string_lossy().to_string();
    let file_str = target.join("standalone.py").to_string_lossy().to_string();

    let mut violations = vec![violation_for(&file_str)];
    dispatcher_lint_arwaky::surface_external_action::filter_outside_member_dirs(
        &mut violations,
        &target_str,
        fs().as_ref(),
    );
    assert_eq!(violations.len(), 1);
}
