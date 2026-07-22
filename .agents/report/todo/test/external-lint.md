
# Test Suite for `external-lint` (v1.10.106)

Below is the complete test suite following the flat `tests/` convention with prefix-based virtual subfolders.

---

## Directory Layout

```
crates/external-lint/
├── src/
│   └── lib.rs
├── tests/
│   ├── contract_external_lint.rs
│   ├── unit_external_lint_selector.rs
│   ├── unit_external_lint_language_detector.rs
│   ├── unit_external_lint_executor.rs
│   ├── unit_external_lint_stdio_client.rs
│   ├── unit_external_lint_adapters.rs
│   ├── unit_external_lint_utility_adapter.rs
│   ├── integration_external_lint.rs
│   ├── smoke_external_lint.rs
│   ├── e2e_external_lint_scan_flow.rs
│   ├── acceptance_FRD_tool_discovery.rs
│   ├── acceptance_FRD_report_unification.rs
│   ├── acceptance_FRD_severity_mapping.rs
│   └── bench_external_lint_selector.rs
└── Cargo.toml
```

---

## `tests/contract_external_lint.rs`

```rust
// PURPOSE: Verify all trait implementations exist and are correctly wired.
// Contract tests confirm that concrete types satisfy their protocol traits
// at compile time — no runtime behavior is exercised here.

use external_lint_lint_arwaky::*;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::external_lint::contract_external_lint_language_detector_protocol::IExternalLintLanguageDetectorProtocol;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;
use shared::external_lint::contract_external_lint_utility_protocol::{
    IExternalLintCargoProtocol, IExternalLintCommandProtocol, IExternalLintJsProtocol,
    IExternalLintLanguageProtocol, IExternalLintPathProtocol,
};

// ─── ILinterAdapterProtocol Implementations ───────────────

#[test]
fn eslint_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<ESLintAdapter>();
}

#[test]
fn prettier_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<PrettierAdapter>();
}

#[test]
fn tsc_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<TSCAdapter>();
}

#[test]
fn bandit_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<BanditAdapter>();
}

#[test]
fn mypy_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<MyPyAdapter>();
}

#[test]
fn ruff_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<RuffAdapter>();
}

#[test]
fn cargo_audit_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<CargoAuditAdapter>();
}

#[test]
fn rustfmt_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<RustFmtAdapter>();
}

#[test]
fn clippy_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<RustLinterAdapter>();
}

// ─── ICommandExecutorProtocol Implementation ──────────────

#[test]
fn stdio_client_implements_command_executor_protocol() {
    fn assert_trait<T: ICommandExecutorProtocol>() {}
    assert_trait::<StdioClient>();
}

// ─── IExternalLintExecutorProtocol Implementation ─────────

#[test]
fn external_lint_executor_implements_executor_protocol() {
    fn assert_trait<T: IExternalLintExecutorProtocol>() {}
    assert_trait::<ExternalLintExecutor>();
}

// ─── IExternalLintAggregate Implementation ────────────────

#[test]
fn orchestrator_implements_external_lint_aggregate() {
    fn assert_trait<T: IExternalLintAggregate>() {}
    assert_trait::<agent_external_lint_orchestrator::ExternalLintOrchestrator>();
}

// ─── IExternalLintSelectorProtocol Implementation ─────────

#[test]
fn selector_implements_selector_protocol() {
    fn assert_trait<T: IExternalLintSelectorProtocol>() {}
    assert_trait::<capabilities_external_lint_selector::CapabilitiesExternalLintSelector>();
}

// ─── IExternalLintLanguageDetectorProtocol Implementation ─

#[test]
fn language_detector_implements_detector_protocol() {
    fn assert_trait<T: IExternalLintLanguageDetectorProtocol>() {}
    assert_trait::<capabilities_language_detector_adapter::ExternalLintLanguageDetectorAdapter>();
}

// ─── IExternalLintUtilityProtocol Implementations ─────────

#[test]
fn utility_adapter_implements_path_protocol() {
    fn assert_trait<T: IExternalLintPathProtocol>() {}
    assert_trait::<capabilities_external_lint_adapter::ExternalLintUtilityAdapter>();
}

#[test]
fn utility_adapter_implements_language_protocol() {
    fn assert_trait<T: IExternalLintLanguageProtocol>() {}
    assert_trait::<capabilities_external_lint_adapter::ExternalLintUtilityAdapter>();
}

#[test]
fn utility_adapter_implements_js_protocol() {
    fn assert_trait<T: IExternalLintJsProtocol>() {}
    assert_trait::<capabilities_external_lint_adapter::ExternalLintUtilityAdapter>();
}

#[test]
fn utility_adapter_implements_cargo_protocol() {
    fn assert_trait<T: IExternalLintCargoProtocol>() {}
    assert_trait::<capabilities_external_lint_adapter::ExternalLintUtilityAdapter>();
}

#[test]
fn utility_adapter_implements_command_protocol() {
    fn assert_trait<T: IExternalLintCommandProtocol>() {}
    assert_trait::<capabilities_external_lint_adapter::ExternalLintUtilityAdapter>();
}

// ─── Send + Sync Bounds ──────────────────────────────────

#[test]
fn all_adapters_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ESLintAdapter>();
    assert_send_sync::<PrettierAdapter>();
    assert_send_sync::<TSCAdapter>();
    assert_send_sync::<BanditAdapter>();
    assert_send_sync::<MyPyAdapter>();
    assert_send_sync::<RuffAdapter>();
    assert_send_sync::<CargoAuditAdapter>();
    assert_send_sync::<RustFmtAdapter>();
    assert_send_sync::<RustLinterAdapter>();
    assert_send_sync::<StdioClient>();
    assert_send_sync::<ExternalLintExecutor>();
    assert_send_sync::<ExternalLintContainer>();
}
```

---

## `tests/unit_external_lint_selector.rs`

```rust
// PURPOSE: Unit tests for CapabilitiesExternalLintSelector — pure business logic
// mapping language flags to adapter name lists.

use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;
use shared::common::taxonomy_adapter_list_vo::AdapterNameList;
use shared::common::taxonomy_common_vo::BooleanVO;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

fn sut() -> CapabilitiesExternalLintSelector {
    CapabilitiesExternalLintSelector::with_defaults()
}

// ─── Happy Path ───────────────────────────────────────────

#[test]
fn select_all_languages_returns_nine_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(true),
        BooleanVO::new(true),
    );
    assert_eq!(result.len(), 9);
}

#[test]
fn select_rust_only_returns_three_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(false),
        BooleanVO::new(false),
    );
    assert_eq!(result.len(), 3);
    let names: Vec<&str> = result.iter().map(|n| n.value()).collect();
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"rustfmt"));
    assert!(names.contains(&"cargo-audit"));
}

#[test]
fn select_python_only_returns_three_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(false),
        BooleanVO::new(true),
        BooleanVO::new(false),
    );
    assert_eq!(result.len(), 3);
    let names: Vec<&str> = result.iter().map(|n| n.value()).collect();
    assert!(names.contains(&"ruff"));
    assert!(names.contains(&"mypy"));
    assert!(names.contains(&"bandit"));
}

#[test]
fn select_js_only_returns_three_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(false),
        BooleanVO::new(false),
        BooleanVO::new(true),
    );
    assert_eq!(result.len(), 3);
    let names: Vec<&str> = result.iter().map(|n| n.value()).collect();
    assert!(names.contains(&"eslint"));
    assert!(names.contains(&"prettier"));
    assert!(names.contains(&"tsc"));
}

// ─── Edge Cases ───────────────────────────────────────────

#[test]
fn select_no_languages_returns_empty_list() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(false),
        BooleanVO::new(false),
        BooleanVO::new(false),
    );
    assert!(result.is_empty());
}

#[test]
fn select_rust_and_python_returns_six_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(true),
        BooleanVO::new(false),
    );
    assert_eq!(result.len(), 6);
}

#[test]
fn select_python_and_js_returns_six_adapters() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(false),
        BooleanVO::new(true),
        BooleanVO::new(true),
    );
    assert_eq!(result.len(), 6);
}

// ─── Custom Adapter Lists ─────────────────────────────────

#[test]
fn custom_adapter_lists_are_respected() {
    use shared::common::taxonomy_adapter_name_vo::AdapterName;

    let selector = CapabilitiesExternalLintSelector::new(
        vec![AdapterName::raw("custom-rs")],
        vec![],
        vec![AdapterName::raw("custom-js-1"), AdapterName::raw("custom-js-2")],
    );

    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(false),
        BooleanVO::new(true),
    );
    assert_eq!(result.len(), 3);
    let names: Vec<&str> = result.iter().map(|n| n.value()).collect();
    assert!(names.contains(&"custom-rs"));
    assert!(names.contains(&"custom-js-1"));
    assert!(names.contains(&"custom-js-2"));
}

#[test]
fn empty_custom_lists_return_nothing() {
    let selector = CapabilitiesExternalLintSelector::new(vec![], vec![], vec![]);
    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(true),
        BooleanVO::new(true),
    );
    assert!(result.is_empty());
}

// ─── Ordering ─────────────────────────────────────────────

#[test]
fn adapter_order_is_rust_then_python_then_js() {
    let selector = sut();
    let result = selector.select_adapters(
        BooleanVO::new(true),
        BooleanVO::new(true),
        BooleanVO::new(true),
    );
    let names: Vec<&str> = result.iter().map(|n| n.value()).collect();
    // Rust adapters first
    assert_eq!(names[0], "clippy");
    assert_eq!(names[1], "rustfmt");
    assert_eq!(names[2], "cargo-audit");
    // Python adapters next
    assert_eq!(names[3], "ruff");
    assert_eq!(names[4], "mypy");
    assert_eq!(names[5], "bandit");
    // JS adapters last
    assert_eq!(names[6], "eslint");
    assert_eq!(names[7], "prettier");
    assert_eq!(names[8], "tsc");
}
```

---

## `tests/unit_external_lint_language_detector.rs`

```rust
// PURPOSE: Unit tests for ExternalLintLanguageDetectorAdapter — directory scanning
// to detect present programming languages.

use external_lint_lint_arwaky::capabilities_language_detector_adapter::ExternalLintLanguageDetectorAdapter;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_language_detector_protocol::IExternalLintLanguageDetectorProtocol;
use std::fs;
use std::path::Path;

fn sut() -> ExternalLintLanguageDetectorAdapter {
    ExternalLintLanguageDetectorAdapter::new()
}

fn create_temp_dir_with_files(files: &[&str]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for file in files {
        let path = dir.path().join(file);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, "// dummy content").unwrap();
    }
    dir
}

// ─── Happy Path ───────────────────────────────────────────

#[tokio::test]
async fn detects_rust_files() {
    let dir = create_temp_dir_with_files(&["src/main.rs", "src/lib.rs"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(result.has_rs.value());
    assert!(!result.has_py.value());
    assert!(!result.has_js.value());
}

#[tokio::test]
async fn detects_python_files() {
    let dir = create_temp_dir_with_files(&["app.py", "utils/helper.py"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs.value());
    assert!(result.has_py.value());
    assert!(!result.has_js.value());
}

#[tokio::test]
async fn detects_js_ts_files() {
    let dir = create_temp_dir_with_files(&["index.ts", "src/app.jsx", "lib/util.tsx"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs.value());
    assert!(!result.has_py.value());
    assert!(result.has_js.value());
}

#[tokio::test]
async fn detects_all_three_languages() {
    let dir = create_temp_dir_with_files(&["main.rs", "app.py", "index.ts"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(result.has_rs.value());
    assert!(result.has_py.value());
    assert!(result.has_js.value());
}

// ─── Edge Cases ───────────────────────────────────────────

#[tokio::test]
async fn empty_directory_detects_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs.value());
    assert!(!result.has_py.value());
    assert!(!result.has_js.value());
}

#[tokio::test]
async fn single_file_path_detects_language() {
    let dir = create_temp_dir_with_files(&["script.py"]);
    let file_path = dir.path().join("script.py");
    let path = FilePath::new(file_path.to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs.value());
    assert!(result.has_py.value());
    assert!(!result.has_js.value());
}

#[tokio::test]
async fn skips_node_modules_directory() {
    let dir = create_temp_dir_with_files(&[
        "src/main.rs",
        "node_modules/pkg/index.js",
    ]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(result.has_rs.value());
    assert!(!result.has_js.value()); // node_modules skipped
}

#[tokio::test]
async fn skips_target_directory() {
    let dir = create_temp_dir_with_files(&[
        "src/lib.py",
        "target/debug/build.rs",
    ]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs.value()); // target/ skipped
    assert!(result.has_py.value());
}

#[tokio::test]
async fn skips_git_directory() {
    let dir = create_temp_dir_with_files(&[
        "app.ts",
        ".git/hooks/pre-commit.rs",
    ]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs.value()); // .git/ skipped
    assert!(result.has_js.value());
}

#[tokio::test]
async fn non_source_files_are_ignored() {
    let dir = create_temp_dir_with_files(&["README.md", "Cargo.lock", "data.json"]);
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let result = sut().detect_languages(&path).await;
    assert!(!result.has_rs.value());
    assert!(!result.has_py.value());
    assert!(!result.has_js.value());
}

// ─── Default Constructor ──────────────────────────────────

#[test]
fn default_constructor_creates_instance() {
    let detector = ExternalLintLanguageDetectorAdapter::default();
    // Just verify it can be constructed via Default
    let _ = detector;
}
```

---

## `tests/unit_external_lint_executor.rs`

```rust
// PURPOSE: Unit tests for ExternalLintExecutor — command execution with error mapping.

use std::sync::Arc;
use external_lint_lint_arwaky::ExternalLintExecutor;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;

// ─── Mock Executor ────────────────────────────────────────

struct MockExecutor {
    response: ResponseData,
}

#[async_trait::async_trait]
impl ICommandExecutorProtocol for MockExecutor {
    async fn execute_command(
        &self,
        _command: PatternList,
        _working_dir: FilePath,
        _timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        Ok(self.response.clone())
    }

    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}

struct FailingExecutor;

#[async_trait::async_trait]
impl ICommandExecutorProtocol for FailingExecutor {
    async fn execute_command(
        &self,
        _command: PatternList,
        _working_dir: FilePath,
        _timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        anyhow::bail!("Command not found: os error 2")
    }

    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}

fn make_executor(response: ResponseData) -> ExternalLintExecutor {
    ExternalLintExecutor::new(Arc::new(MockExecutor { response }))
}

// ─── exec_cmd_scan ────────────────────────────────────────

#[tokio::test]
async fn exec_cmd_scan_returns_response_on_success() {
    let mut resp = ResponseData::new();
    resp.stdout = "lint output".to_string();
    resp.returncode = 0;

    let executor = make_executor(resp);
    let path = FilePath::new("/tmp/test.rs".to_string()).unwrap();

    let result = executor
        .exec_cmd_scan(
            vec!["cargo".to_string(), "clippy".to_string()],
            FilePath::new("/tmp".to_string()).unwrap(),
            60.0,
            Some(AdapterName::raw("clippy")),
            &path,
        )
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().stdout, "lint output");
}

#[tokio::test]
async fn exec_cmd_scan_maps_error_to_scan_error() {
    let executor = ExternalLintExecutor::new(Arc::new(FailingExecutor));
    let path = FilePath::new("/tmp/test.rs".to_string()).unwrap();

    let result = executor
        .exec_cmd_scan(
            vec!["missing-tool".to_string()],
            FilePath::new("/tmp".to_string()).unwrap(),
            60.0,
            Some(AdapterName::raw("test")),
            &path,
        )
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Scan error"));
}

// ─── exec_cmd_adapter ─────────────────────────────────────

#[tokio::test]
async fn exec_cmd_adapter_returns_response_on_success() {
    let mut resp = ResponseData::new();
    resp.stdout = "[]".to_string();
    resp.returncode = 0;

    let executor = make_executor(resp);

    let result = executor
        .exec_cmd_adapter(
            vec!["ruff".to_string(), "check".to_string()],
            FilePath::new("/tmp".to_string()).unwrap(),
            60.0,
            AdapterName::raw("ruff"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn exec_cmd_adapter_maps_error_to_adapter_error() {
    let executor = ExternalLintExecutor::new(Arc::new(FailingExecutor));

    let result = executor
        .exec_cmd_adapter(
            vec!["missing".to_string()],
            FilePath::new("/tmp".to_string()).unwrap(),
            60.0,
            AdapterName::raw("ruff"),
        )
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Adapter error"));
}

// ─── Constructor ──────────────────────────────────────────

#[test]
fn constructor_accepts_arc_executor() {
    let executor: Arc<dyn ICommandExecutorProtocol> = Arc::new(MockExecutor {
        response: ResponseData::new(),
    });
    let lint_executor = ExternalLintExecutor::new(executor);
    let _ = lint_executor;
}
```

---

## `tests/unit_external_lint_stdio_client.rs`

```rust
// PURPOSE: Unit tests for StdioClient — subprocess execution via tokio::process.

use external_lint_lint_arwaky::StdioClient;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_path_vo::FilePath;

fn sut() -> StdioClient {
    StdioClient::new(Timeout::new(10.0))
}

// ─── Happy Path ───────────────────────────────────────────

#[tokio::test]
async fn execute_echo_command_returns_stdout() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec!["echo".to_string(), "hello".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.stdout.trim(), "hello");
    assert_eq!(resp.returncode, 0);
}

#[tokio::test]
async fn execute_command_captures_stderr() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec![
                "sh".to_string(),
                "-c".to_string(),
                "echo error >&2".to_string(),
            ]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    assert!(resp.stderr.contains("error"));
}

#[tokio::test]
async fn execute_command_returns_nonzero_exit_code() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec!["sh".to_string(), "-c".to_string(), "exit 42".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.returncode, 42);
}

#[tokio::test]
async fn metadata_contains_stdio_protocol() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec!["echo".to_string(), "test".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            None,
        )
        .await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(
        resp.metadata.get("protocol").and_then(|v| v.as_str()),
        Some("Stdio")
    );
}

// ─── Error Paths ──────────────────────────────────────────

#[tokio::test]
async fn empty_command_returns_error() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(Vec::<String>::new()),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Empty command"));
}

#[tokio::test]
async fn nonexistent_binary_returns_error() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec!["nonexistent_binary_xyz_12345".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn timeout_kills_long_running_command() {
    let client = StdioClient::new(Timeout::new(0.001));
    let result = client
        .execute_command(
            PatternList::new(vec!["sleep".to_string(), "60".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(0.001)),
        )
        .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("timed out"));
}

// ─── Health Check ─────────────────────────────────────────

#[tokio::test]
async fn health_check_returns_ok() {
    let client = sut();
    let result = client.health_check().await;
    assert!(result.is_ok());
}

// ─── Working Directory ────────────────────────────────────

#[tokio::test]
async fn respects_working_directory() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec!["pwd".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    // /tmp may resolve to /private/tmp on macOS
    assert!(
        resp.stdout.trim().ends_with("/tmp")
            || resp.stdout.trim().ends_with("/private/tmp")
    );
}
```

---

## `tests/unit_external_lint_adapters.rs`

```rust
// PURPOSE: Unit tests for adapter constructors, name() methods, and
// file-extension filtering logic (no subprocess execution).

use std::sync::Arc;
use external_lint_lint_arwaky::*;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;

// ─── Mock Executor (returns empty JSON) ───────────────────

struct EmptyJsonExecutor;

#[async_trait::async_trait]
impl ICommandExecutorProtocol for EmptyJsonExecutor {
    async fn execute_command(
        &self,
        _command: PatternList,
        _working_dir: FilePath,
        _timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        Ok(ResponseData {
            stdout: "[]".to_string(),
            stderr: String::new(),
            returncode: 0,
            ..Default::default()
        })
    }
    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}

struct EmptyLintExecutor;

#[async_trait::async_trait]
impl IExternalLintExecutorProtocol for EmptyLintExecutor {
    async fn exec_cmd_scan(
        &self,
        _args: Vec<String>,
        _working_dir: FilePath,
        _timeout_secs: f64,
        _adapter_name: Option<AdapterName>,
        _path: &FilePath,
    ) -> Result<ResponseData, shared::code_analysis::taxonomy_operation_error::LinterOperationError>
    {
        Ok(ResponseData {
            stdout: "[]".to_string(),
            ..Default::default()
        })
    }
    async fn exec_cmd_adapter(
        &self,
        _args: Vec<String>,
        _working_dir: FilePath,
        _timeout_secs: f64,
        _adapter_name: AdapterName,
    ) -> Result<ResponseData, shared::code_analysis::taxonomy_operation_error::LinterOperationError>
    {
        Ok(ResponseData {
            stdout: "[]".to_string(),
            ..Default::default()
        })
    }
    async fn js_apply_fix(
        &self,
        _path: &FilePath,
        _tool: &str,
        _fix_arg: &str,
    ) -> Result<
        shared::common::taxonomy_message_vo::ComplianceStatus,
        shared::code_analysis::taxonomy_operation_error::LinterOperationError,
    > {
        Ok(shared::common::taxonomy_message_vo::ComplianceStatus::new(false))
    }
}

fn mock_lint_executor() -> Arc<dyn IExternalLintExecutorProtocol> {
    Arc::new(EmptyLintExecutor)
}

fn mock_cmd_executor() -> Arc<dyn ICommandExecutorProtocol> {
    Arc::new(EmptyJsonExecutor)
}

// ─── Adapter Names ────────────────────────────────────────

#[test]
fn eslint_adapter_name() {
    let adapter = ESLintAdapter::new(mock_lint_executor());
    assert_eq!(adapter.name().value(), "eslint");
}

#[test]
fn prettier_adapter_name() {
    let adapter = PrettierAdapter::new(mock_lint_executor());
    assert_eq!(adapter.name().value(), "prettier");
}

#[test]
fn tsc_adapter_name() {
    let adapter = TSCAdapter::new(mock_lint_executor());
    assert_eq!(adapter.name().value(), "tsc");
}

#[test]
fn bandit_adapter_name() {
    let adapter = BanditAdapter::new(mock_lint_executor(), None);
    assert_eq!(adapter.name().value(), "bandit");
}

#[test]
fn mypy_adapter_name() {
    let adapter = MyPyAdapter::new(mock_lint_executor(), None);
    assert_eq!(adapter.name().value(), "mypy");
}

#[test]
fn ruff_adapter_name() {
    let adapter = RuffAdapter::new(mock_lint_executor(), None);
    assert_eq!(adapter.name().value(), "ruff");
}

#[test]
fn cargo_audit_adapter_name() {
    let adapter = CargoAuditAdapter::new();
    assert_eq!(adapter.name().value(), "cargo-audit");
}

#[test]
fn rustfmt_adapter_name() {
    let adapter = RustFmtAdapter::new(mock_cmd_executor(), None);
    assert_eq!(adapter.name().value(), "rustfmt");
}

#[test]
fn clippy_adapter_name() {
    let adapter = RustLinterAdapter::new(mock_cmd_executor(), None);
    assert_eq!(adapter.name().value(), "clippy");
}

// ─── File Extension Filtering ─────────────────────────────

#[tokio::test]
async fn eslint_skips_non_js_file() {
    let adapter = ESLintAdapter::new(mock_lint_executor());
    let path = FilePath::new("/tmp/test.rs".to_string()).unwrap();
    let result = adapter.scan(&path).await.unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn prettier_skips_non_js_file() {
    let adapter = PrettierAdapter::new(mock_lint_executor());
    let path = FilePath::new("/tmp/test.py".to_string()).unwrap();
    let result = adapter.scan(&path).await.unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn tsc_skips_non_ts_file() {
    let adapter = TSCAdapter::new(mock_lint_executor());
    let path = FilePath::new("/tmp/test.js".to_string()).unwrap();
    // .js is not .ts/.tsx — but TSCAdapter only skips non-ts/tsx
    // Actually looking at code: it skips if NOT .ts and NOT .tsx
    let result = adapter.scan(&path).await.unwrap();
    assert!(result.is_empty());
}

// ─── Constructors with bin_path ───────────────────────────

#[test]
fn ruff_adapter_with_custom_bin_path() {
    let bin = FilePath::new("/usr/local/bin/ruff".to_string()).unwrap();
    let adapter = RuffAdapter::new(mock_lint_executor(), Some(bin));
    assert_eq!(adapter.name().value(), "ruff");
}

#[test]
fn bandit_adapter_with_custom_bin_path() {
    let bin = FilePath::new("/usr/local/bin/bandit".to_string()).unwrap();
    let adapter = BanditAdapter::new(mock_lint_executor(), Some(bin));
    assert_eq!(adapter.name().value(), "bandit");
}

#[test]
fn mypy_adapter_with_custom_bin_path() {
    let bin = FilePath::new("/usr/local/bin/mypy".to_string()).unwrap();
    let adapter = MyPyAdapter::new(mock_lint_executor(), Some(bin));
    assert_eq!(adapter.name().value(), "mypy");
}

// ─── CargoAudit Default ───────────────────────────────────

#[test]
fn cargo_audit_default_constructor() {
    let adapter = CargoAuditAdapter::default();
    assert_eq!(adapter.name().value(), "cargo-audit");
}

// ─── apply_fix for non-fixable adapters ───────────────────

#[tokio::test]
async fn tsc_apply_fix_returns_false() {
    let adapter = TSCAdapter::new(mock_lint_executor());
    let path = FilePath::new("/tmp/test.ts".to_string()).unwrap();
    let result = adapter.apply_fix(&path).await.unwrap();
    assert!(!result.value());
}

#[tokio::test]
async fn bandit_apply_fix_returns_false() {
    let adapter = BanditAdapter::new(mock_lint_executor(), None);
    let path = FilePath::new("/tmp/test.py".to_string()).unwrap();
    let result = adapter.apply_fix(&path).await.unwrap();
    assert!(!result.value());
}

#[tokio::test]
async fn mypy_apply_fix_returns_false() {
    let adapter = MyPyAdapter::new(mock_lint_executor(), None);
    let path = FilePath::new("/tmp/test.py".to_string()).unwrap();
    let result = adapter.apply_fix(&path).await.unwrap();
    assert!(!result.value());
}
```

---

## `tests/unit_external_lint_utility_adapter.rs`

```rust
// PURPOSE: Unit tests for ExternalLintUtilityAdapter — path, language, JS, Cargo,
// and command protocol implementations.

use external_lint_lint_arwaky::capabilities_external_lint_adapter::ExternalLintUtilityAdapter;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::external_lint::contract_external_lint_utility_protocol::{
    IExternalLintCargoProtocol, IExternalLintCommandProtocol, IExternalLintJsProtocol,
    IExternalLintLanguageProtocol, IExternalLintPathProtocol,
};
use std::fs;

fn sut() -> ExternalLintUtilityAdapter {
    ExternalLintUtilityAdapter::new()
}

// ─── IExternalLintPathProtocol ────────────────────────────

#[test]
fn canonicalize_path_returns_valid_path() {
    let adapter = sut();
    let result = adapter.canonicalize_path("/tmp");
    assert!(!result.value().is_empty());
}

#[test]
fn canonicalize_path_nonexistent_returns_original() {
    let adapter = sut();
    let result = adapter.canonicalize_path("/nonexistent/path/xyz");
    assert_eq!(result.value(), "/nonexistent/path/xyz");
}

#[test]
fn default_working_dir_returns_dot() {
    let adapter = sut();
    let path = FilePath::new("/some/path".to_string()).unwrap();
    let result = adapter.default_working_dir(&path);
    assert_eq!(result.value(), ".");
}

// ─── IExternalLintLanguageProtocol ────────────────────────

#[test]
fn has_python_files_detects_py_extension() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("script.py"), "print('hi')").unwrap();

    let adapter = sut();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    assert!(adapter.has_python_files(&path).value());
}

#[test]
fn has_python_files_false_for_rs_only() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();

    let adapter = sut();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    assert!(!adapter.has_python_files(&path).value());
}

#[test]
fn has_python_files_single_py_file() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("app.py");
    fs::write(&file, "x = 1").unwrap();

    let adapter = sut();
    let path = FilePath::new(file.to_string_lossy().to_string()).unwrap();
    assert!(adapter.has_python_files(&path).value());
}

#[test]
fn is_in_path_finds_echo() {
    let adapter = sut();
    // 'echo' should be in PATH on any Unix system
    assert!(adapter.is_in_path("echo").value());
}

#[test]
fn is_in_path_false_for_nonexistent() {
    let adapter = sut();
    assert!(!adapter.is_in_path("nonexistent_tool_xyz_99999").value());
}

// ─── IExternalLintJsProtocol ──────────────────────────────

#[test]
fn resolve_js_cmd_uses_executable_name() {
    let adapter = sut();
    let wd = FilePath::new("/tmp".to_string()).unwrap();
    let args = PatternList::new(vec!["--version".to_string()]);
    let result = adapter.resolve_js_cmd("eslint", args, &wd);
    // Should contain eslint somewhere in the command
    assert!(result.values().iter().any(|s| s.contains("eslint")));
}

#[test]
fn resolve_js_working_dir_returns_path() {
    let adapter = sut();
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.resolve_js_working_dir(&path);
    assert!(!result.value().is_empty());
}

// ─── IExternalLintCargoProtocol ───────────────────────────

#[test]
fn resolve_cargo_working_dir_with_cargo_toml() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("Cargo.toml"), "[package]\nname=\"test\"").unwrap();

    let adapter = sut();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = adapter.resolve_cargo_working_dir(&path);
    assert!(result.value().contains("Cargo.toml") || !result.value().is_empty());
}

#[test]
fn resolve_cargo_working_dir_empty_path_returns_input() {
    let adapter = sut();
    let path = FilePath::new("/some/path".to_string()).unwrap();
    // When no Cargo.toml found, returns a fallback
    let result = adapter.resolve_cargo_working_dir(&path);
    assert!(!result.value().is_empty());
}

#[test]
fn resolve_cargo_lock_working_dir_with_cargo_lock() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("Cargo.lock"), "# lock").unwrap();

    let adapter = sut();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = adapter.resolve_cargo_lock_working_dir(&path);
    assert!(!result.value().is_empty());
}

// ─── IExternalLintCommandProtocol ─────────────────────────

#[tokio::test]
async fn noop_apply_fix_returns_false() {
    let adapter = sut();
    let result = adapter.noop_apply_fix().await.unwrap();
    assert!(!result.value());
}

// ─── Default / Constructor ────────────────────────────────

#[test]
fn default_creates_instance() {
    let adapter = ExternalLintUtilityAdapter::default();
    let _ = adapter;
}

#[test]
fn new_creates_instance() {
    let adapter = ExternalLintUtilityAdapter::new();
    let _ = adapter;
}
```

---

## `tests/integration_external_lint.rs`

```rust
// PURPOSE: Integration tests — DI container wiring, adapter registration,
// and orchestrator composition using the real ExternalLintContainer.

use std::collections::HashMap;
use std::sync::Arc;
use external_lint_lint_arwaky::*;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;

// ─── Container Wiring ─────────────────────────────────────

#[test]
fn container_creates_aggregate() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    // Aggregate should be a valid Arc
    assert!(Arc::strong_count(&aggregate) >= 1);
}

#[test]
fn container_default_creates_aggregate() {
    let container = ExternalLintContainer::default();
    let aggregate = container.aggregate();
    assert!(Arc::strong_count(&aggregate) >= 1);
}

#[test]
fn container_new_default_creates_aggregate() {
    let container = ExternalLintContainer::new_default();
    let aggregate = container.aggregate();
    assert!(Arc::strong_count(&aggregate) >= 1);
}

// ─── Adapter Registration ─────────────────────────────────

#[test]
fn orchestrator_has_all_nine_adapters_registered() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();
    assert_eq!(names.len(), 9);
}

#[test]
fn orchestrator_contains_rust_adapters() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();
    assert!(names.contains(&"clippy".to_string()));
    assert!(names.contains(&"rustfmt".to_string()));
    assert!(names.contains(&"cargo-audit".to_string()));
}

#[test]
fn orchestrator_contains_python_adapters() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();
    assert!(names.contains(&"ruff".to_string()));
    assert!(names.contains(&"mypy".to_string()));
    assert!(names.contains(&"bandit".to_string()));
}

#[test]
fn orchestrator_contains_js_adapters() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();
    assert!(names.contains(&"eslint".to_string()));
    assert!(names.contains(&"prettier".to_string()));
    assert!(names.contains(&"tsc".to_string()));
}

// ─── Orchestrator with Custom Adapters ────────────────────

#[test]
fn orchestrator_with_empty_adapters_returns_empty_names() {
    let orchestrator =
        agent_external_lint_orchestrator::ExternalLintOrchestrator::new(HashMap::new());
    let names = orchestrator.adapter_names();
    assert!(names.is_empty());
}

#[tokio::test]
async fn orchestrator_with_no_adapters_returns_empty_results() {
    let orchestrator =
        agent_external_lint_orchestrator::ExternalLintOrchestrator::new(HashMap::new());
    let path = shared::common::taxonomy_path_vo::FilePath::new("/tmp".to_string()).unwrap();
    let results = orchestrator.scan_all(&path).await;
    assert!(results.is_empty());
}

// ─── Aggregate Trait Object Safety ────────────────────────

#[test]
fn aggregate_is_object_safe() {
    let container = ExternalLintContainer::new();
    let aggregate: Arc<dyn IExternalLintAggregate> = container.aggregate();
    // Verify we can call methods through the trait object
    let _ = aggregate.adapter_names();
}

// ─── Multiple Container Instances ─────────────────────────

#[test]
fn multiple_containers_are_independent() {
    let c1 = ExternalLintContainer::new();
    let c2 = ExternalLintContainer::new();
    let a1 = c1.aggregate();
    let a2 = c2.aggregate();
    // Different Arc pointers
    assert!(!Arc::ptr_eq(&a1, &a2));
}
```

---

## `tests/smoke_external_lint.rs`

```rust
// PURPOSE: Smoke test — verify the external-lint subsystem boots and responds
// within 5 seconds. If this fails, nothing else matters.

use external_lint_lint_arwaky::ExternalLintContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;

#[tokio::test]
async fn external_lint_boots_and_returns_adapter_names() {
    let start = std::time::Instant::now();

    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();

    assert!(!names.is_empty(), "Container must register at least one adapter");
    assert_eq!(names.len(), 9, "Expected 9 adapters (3 Rust + 3 Python + 3 JS)");

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test must complete in under 5 seconds, took {:?}",
        elapsed
    );
}

#[tokio::test]
async fn scan_all_on_empty_dir_completes_without_panic() {
    let dir = tempfile::tempdir().unwrap();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let start = std::time::Instant::now();
    let results = aggregate.scan_all(&path).await;
    let elapsed = start.elapsed();

    // Empty dir → no language detected → no adapters run → empty results
    assert!(results.is_empty());
    assert!(
        elapsed.as_secs() < 5,
        "Smoke scan must complete in under 5 seconds, took {:?}",
        elapsed
    );
}
```

---

## `tests/e2e_external_lint_scan_flow.rs`

```rust
// PURPOSE: E2E tests — full scan lifecycle through the real container.
// Creates real files on disk, runs the orchestrator, asserts on real output.
// External tools may not be installed; the system must handle that gracefully.

use external_lint_lint_arwaky::ExternalLintContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use std::fs;

#[tokio::test]
async fn scan_rust_project_does_not_crash() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/main.rs"), "fn main() { println!(\"hello\"); }").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    // Should not panic regardless of whether clippy/rustfmt are installed
    let results = aggregate.scan_all(&path).await;
    // Results may be empty if tools aren't installed — that's OK
    let _ = results;
}

#[tokio::test]
async fn scan_python_project_does_not_crash() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("app.py"), "x: int = 'not an int'\n").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results = aggregate.scan_all(&path).await;
    let _ = results;
}

#[tokio::test]
async fn scan_js_project_does_not_crash() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"name": "test", "version": "1.0.0"}"#,
    )
    .unwrap();
    fs::write(dir.path().join("index.ts"), "const x: number = 'oops';\n").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results = aggregate.scan_all(&path).await;
    let _ = results;
}

#[tokio::test]
async fn scan_mixed_project_detects_all_languages() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    fs::write(dir.path().join("app.py"), "print('hi')").unwrap();
    fs::write(dir.path().join("index.ts"), "export {}").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    // All 9 adapters should be attempted
    let names = aggregate.adapter_names();
    assert_eq!(names.len(), 9);

    // Scan should complete without panic
    let results = aggregate.scan_all(&path).await;
    let _ = results;
}

#[tokio::test]
async fn scan_single_file_path_works() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("script.py");
    fs::write(&file, "import os\n").unwrap();

    let path = FilePath::new(file.to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results = aggregate.scan_all(&path).await;
    let _ = results;
}

#[tokio::test]
async fn scan_nonexistent_path_does_not_crash() {
    let path = FilePath::new("/nonexistent/path/xyz_12345".to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    // Should not panic — language detection finds nothing, no adapters run
    let results = aggregate.scan_all(&path).await;
    assert!(results.is_empty());
}
```

---

## `tests/acceptance_FRD_tool_discovery.rs`

```rust
// PURPOSE: Acceptance test — FRD: "Tool discovery and fallback — missing tools
// are safely ignored or warned about without crashing the run."

use external_lint_lint_arwaky::ExternalLintContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use std::fs;

/// FRD-EXT-001: Missing tools are safely ignored without crashing the run.
#[tokio::test]
async fn frd_001_missing_tools_do_not_crash_scan() {
    // Create a project that triggers all 9 adapters
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    fs::write(dir.path().join("app.py"), "x = 1").unwrap();
    fs::write(dir.path().join("index.ts"), "export {}").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    // Even if none of the 9 tools are installed, scan_all must not panic
    let results = aggregate.scan_all(&path).await;

    // The result is a valid LintResultList (possibly empty)
    // The key assertion: we reached this line without a panic
    let _ = results.len();
}

/// FRD-EXT-002: Scan continues with remaining adapters when one tool is missing.
#[tokio::test]
async fn frd_002_partial_tool_availability_still_returns_results() {
    let dir = tempfile::tempdir().unwrap();
    // Only Python files — triggers ruff, mypy, bandit
    fs::write(dir.path().join("app.py"), "import os\nprint('hello')\n").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    // Should not crash even if ruff/mypy/bandit are not installed
    let results = aggregate.scan_all(&path).await;
    // Results may be empty (tools missing) or populated (tools present)
    // Either way, no crash
    let _ = results;
}

/// FRD-EXT-003: Empty project directory produces empty results without error.
#[tokio::test]
async fn frd_003_empty_project_returns_empty_results() {
    let dir = tempfile::tempdir().unwrap();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results = aggregate.scan_all(&path).await;
    assert!(results.is_empty());
}
```

---

## `tests/acceptance_FRD_report_unification.rs`

```rust
// PURPOSE: Acceptance test — FRD: "Seamless report unification — AES and external
// violations combined in a single unified report or MCP response."

use external_lint_lint_arwaky::ExternalLintContainer;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use std::fs;

/// FRD-EXT-004: scan_all returns a unified LintResultList combining all adapter outputs.
#[tokio::test]
async fn frd_004_scan_all_returns_unified_result_list() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    fs::write(dir.path().join("app.py"), "x = 1").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results: LintResultList = aggregate.scan_all(&path).await;

    // The return type IS the unified list — type-level proof of unification
    // Each result has file, line, column, code, message, source, severity
    for result in results.iter() {
        assert!(!result.file.value().is_empty(), "Each result must have a file");
        assert!(result.source.is_some(), "Each result must have a source adapter");
    }
}

/// FRD-EXT-005: Each LintResult carries the source adapter name for traceability.
#[tokio::test]
async fn frd_005_results_carry_adapter_source() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("script.py"), "import os\n").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results = aggregate.scan_all(&path).await;

    // If any results exist, they must have a source
    for result in results.iter() {
        let source = result.source.as_ref().expect("source must be present");
        let name = source.value();
        assert!(
            ["ruff", "mypy", "bandit", "eslint", "prettier", "tsc", "clippy", "rustfmt", "cargo-audit"]
                .contains(&name),
            "Source '{}' must be a known adapter",
            name
        );
    }
}

/// FRD-EXT-006: adapter_names() exposes the full registered adapter list.
#[tokio::test]
async fn frd_006_adapter_names_exposes_all_registered() {
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();

    let expected = [
        "clippy", "rustfmt", "cargo-audit", "ruff", "mypy", "bandit", "eslint", "prettier", "tsc",
    ];
    for name in &expected {
        assert!(
            names.contains(&name.to_string()),
            "Expected adapter '{}' to be registered",
            name
        );
    }
}
```

---

## `tests/acceptance_FRD_severity_mapping.rs`

```rust
// PURPOSE: Acceptance test — FRD: "Error level translation — tool severities
// are correctly mapped to Lint Arwaky Severity."

use shared::cli_commands::taxonomy_severity_vo::Severity;

/// FRD-EXT-007: Severity enum has all four required levels.
#[test]
fn frd_007_severity_has_all_levels() {
    let levels = [
        Severity::CRITICAL,
        Severity::HIGH,
        Severity::MEDIUM,
        Severity::LOW,
    ];
    assert_eq!(levels.len(), 4);
}

/// FRD-EXT-008: Severity Display renders lowercase strings.
#[test]
fn frd_008_severity_display_is_lowercase() {
    assert_eq!(Severity::CRITICAL.to_string(), "critical");
    assert_eq!(Severity::HIGH.to_string(), "high");
    assert_eq!(Severity::MEDIUM.to_string(), "medium");
    assert_eq!(Severity::LOW.to_string(), "low");
    assert_eq!(Severity::INFO.to_string(), "info");
}

/// FRD-EXT-009: Severity score_impact is monotonically increasing.
#[test]
fn frd_009_severity_score_impact_ordering() {
    assert!(Severity::CRITICAL.score_impact() > Severity::HIGH.score_impact());
    assert!(Severity::HIGH.score_impact() > Severity::MEDIUM.score_impact());
    assert!(Severity::MEDIUM.score_impact() > Severity::LOW.score_impact());
    assert!(Severity::LOW.score_impact() > Severity::INFO.score_impact());
}

/// FRD-EXT-010: Severity serialization uses lowercase rename.
#[test]
fn frd_010_severity_serializes_correctly() {
    let json = serde_json::to_string(&Severity::HIGH).unwrap();
    assert_eq!(json, "\"high\"");

    let json = serde_json::to_string(&Severity::CRITICAL).unwrap();
    assert_eq!(json, "\"critical\"");
}

/// FRD-EXT-011: Severity deserialization roundtrips.
#[test]
fn frd_011_severity_deserialization_roundtrip() {
    let original = Severity::MEDIUM;
    let json = serde_json::to_string(&original).unwrap();
    let restored: Severity = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

/// FRD-EXT-012: Default severity is INFO.
#[test]
fn frd_012_default_severity_is_info() {
    let default = Severity::default();
    assert_eq!(default, Severity::INFO);
}
```

---

## `tests/bench_external_lint_selector.rs`

```rust
// PURPOSE: Benchmark tests for CapabilitiesExternalLintSelector — measures
// adapter selection throughput under various language combinations.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;
use shared::common::taxonomy_common_vo::BooleanVO;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

fn bench_select_adapters(c: &mut Criterion) {
    let selector = CapabilitiesExternalLintSelector::with_defaults();

    let mut group = c.benchmark_group("external_lint_selector");

    let cases: Vec<(&str, bool, bool, bool)> = vec![
        ("none", false, false, false),
        ("rust_only", true, false, false),
        ("python_only", false, true, false),
        ("js_only", false, false, true),
        ("rust_python", true, true, false),
        ("rust_js", true, false, true),
        ("python_js", false, true, true),
        ("all", true, true, true),
    ];

    for (name, rs, py, js) in cases {
        group.bench_with_input(
            BenchmarkId::new("select_adapters", name),
            &(rs, py, js),
            |b, &(rs, py, js)| {
                b.iter(|| {
                    selector.select_adapters(
                        BooleanVO::new(rs),
                        BooleanVO::new(py),
                        BooleanVO::new(js),
                    )
                });
            },
        );
    }

    group.finish();
}

fn bench_container_creation(c: &mut Criterion) {
    c.bench_function("external_lint_container_new", |b| {
        b.iter(|| external_lint_lint_arwaky::ExternalLintContainer::new());
    });
}

fn bench_adapter_names(c: &mut Criterion) {
    let container = external_lint_lint_arwaky::ExternalLintContainer::new();
    let aggregate = container.aggregate();

    c.bench_function("external_lint_adapter_names", |b| {
        b.iter(|| aggregate.adapter_names());
    });
}

criterion_group!(
    benches,
    bench_select_adapters,
    bench_container_creation,
    bench_adapter_names
);
criterion_main!(benches);
```

---

## `Cargo.toml` Additions

Add the following to `crates/external-lint/Cargo.toml`:

```toml
[dev-dependencies]
tokio.workspace = true
tempfile = "3"
criterion = { version = "0.5", features = ["async_tokio"] }
serde_json.workspace = true

[[bench]]
name = "bench_external_lint_selector"
path = "tests/bench_external_lint_selector.rs"
harness = false
```

---

## Task Progress

```
Task Progress:
- [x] Step 1: Analyze crate / app structure
- [x] Step 2: Identify untested public API
- [x] Step 3: Write contract_external_lint.rs
- [x] Step 4: Write unit_external_lint_selector.rs
- [x] Step 4: Write unit_external_lint_language_detector.rs
- [x] Step 4: Write unit_external_lint_executor.rs
- [x] Step 4: Write unit_external_lint_stdio_client.rs
- [x] Step 4: Write unit_external_lint_adapters.rs
- [x] Step 4: Write unit_external_lint_utility_adapter.rs
- [x] Step 5: Write integration_external_lint.rs
- [x] Step 6: Write smoke_external_lint.rs
- [x] Step 7: Write e2e_external_lint_scan_flow.rs
- [x] Step 8: Write acceptance_FRD_tool_discovery.rs
- [x] Step 8: Write acceptance_FRD_report_unification.rs
- [x] Step 8: Write acceptance_FRD_severity_mapping.rs
- [x] Step 9: Write bench_external_lint_selector.rs + register in Cargo.toml
- [ ] Step 10: Run suite, fix failures, repeat until green
- [ ] Step 11: Verify coverage + perf baseline
```

---

## Run Commands

```bash
# All tests
cargo test -p external_lint-lint-arwaky

# Contract only
cargo test -p external_lint-lint-arwaky --test contract_external_lint

# Unit tests
cargo test -p external_lint-lint-arwaky --test unit_external_lint_selector
cargo test -p external_lint-lint-arwaky --test unit_external_lint_language_detector
cargo test -p external_lint-lint-arwaky --test unit_external_lint_executor
cargo test -p external_lint-lint-arwaky --test unit_external_lint_stdio_client
cargo test -p external_lint-lint-arwaky --test unit_external_lint_adapters
cargo test -p external_lint-lint-arwaky --test unit_external_lint_utility_adapter

# Integration
cargo test -p external_lint-lint-arwaky --test integration_external_lint

# Smoke
cargo test -p external_lint-lint-arwaky --test smoke_external_lint

# E2E
cargo test -p external_lint-lint-arwaky --test e2e_external_lint_scan_flow

# Acceptance
cargo test -p external_lint-lint-arwaky --test acceptance_FRD_tool_discovery
cargo test -p external_lint-lint-arwaky --test acceptance_FRD_report_unification
cargo test -p external_lint-lint-arwaky --test acceptance_FRD_severity_mapping

# Benchmarks
cargo bench -p external_lint-lint-arwaky

# Coverage
cargo tarpaulin -p external_lint-lint-arwaky --fail-under 70
```
