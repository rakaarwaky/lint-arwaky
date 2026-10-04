// Regression tests — single file, subfolder, and workspace scan modes
// for all scanners across Rust, Python, and TypeScript workspaces-bad fixtures.
//
// Two test strategies:
// 1. collect_scan (in-process) — tests workspaces-good (false positive tests)
// 2. CLI subprocess — tests workspaces-bad via `cargo run --release --bin lint-arwaky-cli`
//
// Prevents regressions in: tracing→stderr fix, workspace root detection,
// path normalization, member filtering, single file scan.
use shared_common::taxonomy_path_vo::FilePath;
use std::process::Command;

mod common;

fn fs() -> std::sync::Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate>
{
    filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()
}

/// Seam bundle for ScanOptions: io + workspace + parser + aggregate.
fn seam() -> std::sync::Arc<dispatcher_lint_arwaky::surface_check_action::FilesystemSeam> {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    std::sync::Arc::new(
        dispatcher_lint_arwaky::surface_check_action::FilesystemSeam {
            workspace: c.workspace(),
            parser: c.parser(),
            aggregate: c.orchestrator(),
        },
    )
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
fn scan(path: &str) -> Vec<shared_common::ViolationItem> {
    let full_path = workspace_root().join(path);
    let root_str = full_path.to_string_lossy().to_string();
    let opts = dispatcher_lint_arwaky::surface_check_action::ScanOptions {
        path: Some(FilePath::new(full_path.to_string_lossy().to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: seam(),
        scan_aggregates: Some(common::build_scan_aggregates(&root_str)),
    };
    dispatcher_lint_arwaky::surface_check_action::collect_scan(opts).unwrap_or_default()
}

/// Locate a built `lint-arwaky-cli` binary for the subprocess-fallback tests.
///
/// Probed, in order:
///   1. `<dir-of-this-test-binary>/{release,debug}/lint-arwaky-cli`
///   2. `<workspace-root>/target/{release,debug}/lint-arwaky-cli`
///
/// (1) covers a build that produced the test binary and the CLI together, and
/// (2) covers a plain `cargo build` into the default target dir — which is
/// what the CI coverage job does, because cargo-llvm-cov owns its own
/// `target/llvm-cov-target` and gives no guarantee about what survives in it.
///
/// Pinning a single `release/` path, as this used to, made these tests fail in
/// any build that is not a release build of the same target dir.
/// `--lib --tests` never builds bin targets, so the caller is still responsible
/// for having built one: `cargo build --release --bin lint-arwaky-cli`.
fn cli_path() -> std::path::PathBuf {
    let beside_this_test = std::env::current_exe().ok().and_then(|p| {
        p.parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
    });
    let default_target = workspace_root().join("target");
    let mut probed = Vec::new();
    for dir in [beside_this_test, Some(default_target)]
        .into_iter()
        .flatten()
    {
        for profile in ["release", "debug"] {
            let candidate = dir.join(profile).join("lint-arwaky-cli");
            if candidate.is_file() {
                return candidate;
            }
            probed.push(candidate);
        }
    }
    panic!(
        "no lint-arwaky-cli binary found. Probed: {}. Build one with: \
         cargo build --release --bin lint-arwaky-cli",
        probed
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

/// CLI subprocess scan (for workspaces-bad where violations expected).
fn cli_scan(path: &str) -> String {
    let exe = cli_path();
    let full_path = workspace_root().join(path);
    let output = Command::new(&exe)
        .args([
            "scan",
            full_path.to_str().unwrap_or(path),
            "--format",
            "json",
        ])
        .output()
        .unwrap_or_else(|e| panic!("failed to run CLI at {}: {}", exe.display(), e));
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn count_violations(json: &str) -> usize {
    // Count the results array. The scan JSON carries one object per violation
    // and no summary block, so a count key would have to be invented here — and
    // an absent key would make this return 0 and quietly pass every count
    // assertion below. So a missing or malformed `results` panics instead.
    let value: serde_json::Value = serde_json::from_str(json)
        .unwrap_or_else(|e| panic!("scan output is not JSON: {e}\n{json}"));
    value
        .get("results")
        .and_then(|r| r.as_array())
        .unwrap_or_else(|| panic!("scan JSON has no `results` array: {json}"))
        .len()
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

fn violation_for(path: &str) -> shared_common::ViolationItem {
    shared_common::ViolationItem {
        code: shared_common::taxonomy_error_vo::ErrorCode::raw("B307"),
        file: FilePath::new(path.to_string()).unwrap(),
        line: shared_common::taxonomy_common_vo::LineNumber::new(1),
        column: shared_common::taxonomy_common_vo::ColumnNumber::new(1),
        message: shared_common::taxonomy_message_vo::LintMessage::new("test"),
        severity: shared_common::taxonomy_severity_vo::Severity::MEDIUM,
        violation_name: String::new(),
        why: String::new(),
        fix: String::new(),
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
