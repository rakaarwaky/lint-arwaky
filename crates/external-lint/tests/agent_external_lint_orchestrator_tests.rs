use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use external_lint_lint_arwaky::agent_external_lint_orchestrator::ExternalLintOrchestrator;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_common_vo::ColumnNumber;
use shared::common::taxonomy_common_vo::LineNumber;

use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_vo::LocationList;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_common_vo::BooleanVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_language_detector_port::{
    DetectedLanguages, IExternalLintLanguageDetectorPort,
};
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

static COUNTER: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// Mock adapter that captures the path it was called with and returns
// a preconfigured set of results.  Useful for verifying dispatch logic.

// ---------------------------------------------------------------------------
struct MockAdapter {
    name: &'static str,
    results: LintResultList,
    fail_with: Option<LinterOperationError>,
}

#[async_trait]
impl ILinterAdapterPort for MockAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw(self.name)
    }

    async fn scan(&self, _path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        match &self.fail_with {
            Some(e) => Err(e.clone()),
            None => Ok(self.results.clone()),
        }
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(true))
    }
}

// ---------------------------------------------------------------------------
// Mock language detector that scans temp directories for file extensions

// ---------------------------------------------------------------------------
struct MockLanguageDetector;

#[async_trait]
impl IExternalLintLanguageDetectorPort for MockLanguageDetector {
    async fn detect_languages(&self, path: &FilePath) -> DetectedLanguages {
        let mut has_rs = false;
        let mut has_py = false;
        let mut has_js = false;

        let root = std::path::Path::new(&path.value);
        if root.is_file() {
            if let Some(ext) = root.extension().and_then(|e| e.to_str()) {
                match ext {
                    "rs" => has_rs = true,
                    "py" => has_py = true,
                    "js" | "ts" | "jsx" | "tsx" => has_js = true,
                    _ => {}
                }
            }
        } else if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    let name = p
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    if !matches!(name.as_ref(), "node_modules" | "target" | ".git" | ".jj") {
                        // recurse
                        let sub =
                            FilePath::new(p.to_string_lossy().to_string()).unwrap_or_default();
                        let sub_det = MockLanguageDetector;
                        let sub_lang = sub_det.detect_languages(&sub).await;
                        has_rs |= sub_lang.has_rs.value;
                        has_py |= sub_lang.has_py.value;
                        has_js |= sub_lang.has_js.value;
                    }
                } else if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                    match ext {
                        "rs" => has_rs = true,
                        "py" => has_py = true,
                        "js" | "ts" | "jsx" | "tsx" => has_js = true,
                        _ => {}
                    }
                }
                if has_rs.value && has_py.value && has_js.value {
                    break;
                }
            }
        }

        DetectedLanguages {
            has_rs: BooleanVO::new(has_rs),
            has_py: BooleanVO::new(has_py),
            has_js: BooleanVO::new(has_js),
        }
    }
}

// ---------------------------------------------------------------------------
// Mock selector — uses default adapter mapping

// ---------------------------------------------------------------------------
struct MockSelector;

impl IExternalLintSelectorProtocol for MockSelector {
    fn select_adapters(&self, has_rs: BooleanVO, has_py: BooleanVO, has_js: BooleanVO) -> Vec<String> {
        let mut names = Vec::new();
        if has_rs.value {
            names.extend(["clippy", "rustfmt", "cargo-audit"].map(String::from));
        }
        if has_py.value {
            names.extend(["ruff", "mypy", "bandit"].map(String::from));
        }
        if has_js.value {
            names.extend(["eslint", "prettier", "tsc"].map(String::from));
        }
        names
    }
}

fn make_orchestrator(
    adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
) -> ExternalLintOrchestrator {
    ExternalLintOrchestrator::new(
        adapters,
        Arc::new(MockLanguageDetector),
        Arc::new(MockSelector),
    )
}

fn make_adapters() -> HashMap<String, Arc<dyn ILinterAdapterPort>> {
    let mut m: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();
    m.insert(
        "clippy".into(),
        Arc::new(MockAdapter {
            name: "clippy",
            results: LintResultList::new(vec![LintResult {
                file: FilePath::new("src/main.rs".to_string()).unwrap_or_default(),
                line: LineNumber::new(10),
                column: ColumnNumber::new(5),
                code: ErrorCode::raw("clippy::pedantic"),
                message: LintMessage::new("test clippy finding"),
                source: Some(AdapterName::raw("clippy")),
                severity: Severity::MEDIUM,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            }]),
            fail_with: None,
        }),
    );
    m.insert(
        "ruff".into(),
        Arc::new(MockAdapter {
            name: "ruff",
            results: LintResultList::new(vec![LintResult {
                file: FilePath::new("src/main.py".to_string()).unwrap_or_default(),
                line: LineNumber::new(3),
                column: ColumnNumber::new(1),
                code: ErrorCode::raw("F401"),
                message: LintMessage::new("test ruff finding"),
                source: Some(AdapterName::raw("ruff")),
                severity: Severity::MEDIUM,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            }]),
            fail_with: None,
        }),
    );
    m.insert(
        "eslint".into(),
        Arc::new(MockAdapter {
            name: "eslint",
            results: LintResultList::new(vec![LintResult {
                file: FilePath::new("src/app.ts".to_string()).unwrap_or_default(),
                line: LineNumber::new(42),
                column: ColumnNumber::new(7),
                code: ErrorCode::raw("no-unused-vars"),
                message: LintMessage::new("test eslint finding"),
                source: Some(AdapterName::raw("eslint")),
                severity: Severity::HIGH,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            }]),
            fail_with: None,
        }),
    );
    m
}

fn make_temp_dir(files: &[&str]) -> std::path::PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("lint_test_{}_{}", std::process::id(), id));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for f in files {
        let path = dir.join(f);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, "").unwrap();
    }
    dir
}

// ---------------------------------------------------------------------------
// Language detection tests (using temp directories)

// ---------------------------------------------------------------------------

#[tokio::test]
async fn detects_rust_project() {
    let dir = make_temp_dir(&["main.rs", "lib.rs"]);
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(make_adapters());
    let results = orch.scan_all(&path).await;
    // Only clippy adapter is set up for rust; should produce 1 result
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].source.as_ref().unwrap().value(), "clippy");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn detects_python_project() {
    let dir = make_temp_dir(&["app.py", "utils.py"]);
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(make_adapters());
    let results = orch.scan_all(&path).await;
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].source.as_ref().unwrap().value(), "ruff");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn detects_javascript_project() {
    let dir = make_temp_dir(&["app.ts", "component.tsx", "style.js"]);
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(make_adapters());
    let results = orch.scan_all(&path).await;
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].source.as_ref().unwrap().value(), "eslint");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn detects_multi_language_project() {
    let dir = make_temp_dir(&["main.rs", "app.py", "ui.tsx"]);
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(make_adapters());
    let results = orch.scan_all(&path).await;
    // All 3 adapters should fire
    assert_eq!(results.len(), 3);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn detects_single_file_by_extension_rust() {
    let dir = make_temp_dir(&["main.rs"]);
    let file_path = dir.join("main.rs");
    let path = FilePath::new(file_path.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(make_adapters());
    let results = orch.scan_all(&path).await;
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].source.as_ref().unwrap().value(), "clippy");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn detects_single_file_by_extension_python() {
    let dir = make_temp_dir(&["module.py"]);
    let file_path = dir.join("module.py");
    let path = FilePath::new(file_path.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(make_adapters());
    let results = orch.scan_all(&path).await;
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].source.as_ref().unwrap().value(), "ruff");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn detects_single_file_by_extension_javascript() {
    let dir = make_temp_dir(&["component.tsx"]);
    let file_path = dir.join("component.tsx");
    let path = FilePath::new(file_path.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(make_adapters());
    let results = orch.scan_all(&path).await;
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].source.as_ref().unwrap().value(), "eslint");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn skips_node_modules_and_target() {
    let dir = make_temp_dir(&[
        "main.rs",
        "node_modules/package/index.js",
        "target/debug/build/out.rs",
        ".git/config",
    ]);
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(make_adapters());
    let results = orch.scan_all(&path).await;
    // Should still detect Rust (main.rs in root), not JS (only in node_modules)
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].source.as_ref().unwrap().value(), "clippy");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn empty_project_returns_no_results() {
    let dir = make_temp_dir(&[]);
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(HashMap::new());
    let results = orch.scan_all(&path).await;
    assert_eq!(results.len(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn adapter_not_in_map_is_skipped_gracefully() {
    let dir = make_temp_dir(&["main.rs"]);
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    // Only register ruff — no clippy for this Rust project
    let mut adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();
    adapters.insert(
        "ruff".into(),
        Arc::new(MockAdapter {
            name: "ruff",
            results: LintResultList::default(),
            fail_with: None,
        }),
    );
    let orch = make_orchestrator(adapters);
    let results = orch.scan_all(&path).await;
    // Rust adapters (clippy, rustfmt, cargo-audit) aren't registered → no results
    assert_eq!(results.len(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn merges_results_from_multiple_adapters() {
    let dir = make_temp_dir(&["main.rs", "app.py"]);
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(make_adapters());
    let results = orch.scan_all(&path).await;
    assert_eq!(results.len(), 2);
    let sources: Vec<&str> = results
        .values
        .iter()
        .map(|r| r.source.as_ref().unwrap().value())
        .collect();
    assert!(sources.contains(&"clippy"));
    assert!(sources.contains(&"ruff"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn adapter_names_returns_registered_names() {
    let adapters = make_adapters();
    let orch = make_orchestrator(adapters);
    let mut names = orch.adapter_names();
    names.sort();
    assert_eq!(names, vec!["clippy", "eslint", "ruff"]);
}

#[tokio::test]
async fn adapter_failure_does_not_crash_scan() {
    // An adapter that fails should not prevent other adapters from running
    let mut adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();
    adapters.insert(
        "clippy".into(),
        Arc::new(MockAdapter {
            name: "clippy",
            results: LintResultList::default(),
            fail_with: Some(LinterOperationError::Adapter(AdapterError::new(
                AdapterName::raw("clippy"),
                ErrorMessage::new("No such file or directory".to_string()),
            ))),
        }),
    );
    adapters.insert(
        "ruff".into(),
        Arc::new(MockAdapter {
            name: "ruff",
            results: LintResultList::new(vec![LintResult {
                file: FilePath::new("test.py".to_string()).unwrap_or_default(),
                line: LineNumber::new(1),
                column: ColumnNumber::new(1),
                code: ErrorCode::raw("E999"),
                message: LintMessage::new("syntax error"),
                source: Some(AdapterName::raw("ruff")),
                severity: Severity::HIGH,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            }]),
            fail_with: None,
        }),
    );

    let dir = make_temp_dir(&["main.py"]);
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let orch = make_orchestrator(adapters);
    let results = orch.scan_all(&path).await;
    // Should still get results from ruff even though clippy failed
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].source.as_ref().unwrap().value(), "ruff");
    let _ = std::fs::remove_dir_all(&dir);
}
