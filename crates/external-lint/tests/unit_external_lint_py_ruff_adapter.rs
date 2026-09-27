// Unit tests for RuffAdapter — ruff JSON output parsing.
use external_lint_lint_arwaky::capabilities_py_ruff_adapter::RuffAdapter;

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::MockFilesystem;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::common::taxonomy_severity_vo::Severity;
use shared::external_lint::contract_external_lint_protocol::ICommandExecutorProtocol;
use std::sync::Arc;

/// Backs the FR-006 `ICommandExecutorProtocol` seam: raw execution plus the
/// `exec_cmd_*` error-mapping wrappers. Returns empty output for every call.
struct MockCmdExecutor;

impl ICommandExecutorProtocol for MockCmdExecutor {
    fn execute_command(
        &self,
        _: PatternList,
        _: FilePath,
        _: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::default())
    }
    fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::default())
    }
    fn exec_cmd_scan(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: Option<AdapterName>,
        _: &FilePath,
    ) -> Result<ResponseData, LinterOperationError> {
        Ok(ResponseData::default())
    }
    fn exec_cmd_adapter(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: AdapterName,
    ) -> Result<ResponseData, LinterOperationError> {
        Ok(ResponseData::default())
    }
}

fn make_adapter() -> RuffAdapter {
    let executor: Arc<dyn ICommandExecutorProtocol> = Arc::new(MockCmdExecutor);
    RuffAdapter::new(
        executor,
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    )
}

// ─── FRD-004: Ruff severity mapping per code ───

#[test]
fn e999_syntax_error_maps_to_critical() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("error", "E999"), Severity::CRITICAL);
}

#[test]
fn security_rules_map_to_critical() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "S105"), Severity::CRITICAL);
    assert_eq!(adapter.map_severity("warning", "S602"), Severity::CRITICAL);
    assert_eq!(adapter.map_severity("error", "S101"), Severity::CRITICAL);
}

#[test]
fn f8xx_undefined_name_maps_to_high() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "F821"), Severity::HIGH);
    assert_eq!(adapter.map_severity("warning", "F811"), Severity::HIGH);
}

#[test]
fn b0xx_bugbear_maps_to_high() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "B006"), Severity::HIGH);
    assert_eq!(adapter.map_severity("warning", "B007"), Severity::HIGH);
}

#[test]
fn f401_unused_import_maps_to_medium() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "F401"), Severity::MEDIUM);
}

#[test]
fn e1xx_indentation_maps_to_low() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "E111"), Severity::LOW);
    assert_eq!(adapter.map_severity("warning", "E117"), Severity::LOW);
}

#[test]
fn e5xx_line_length_maps_to_low() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "E501"), Severity::LOW);
}

#[test]
fn w2xx_whitespace_maps_to_low() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "W291"), Severity::LOW);
    assert_eq!(adapter.map_severity("warning", "W292"), Severity::LOW);
}

#[test]
fn unknown_code_defaults_to_medium() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("warning", "C999"), Severity::MEDIUM);
    assert_eq!(adapter.map_severity("error", "XXXX"), Severity::MEDIUM);
}

/// Regression: when the scan target is a relative path (e.g. `workspaces-good/modules`),
/// the adapter must pass an absolute path to the tool so the tool does not resolve it
/// against its own CWD and produce a doubled path like
/// `workspaces-good/modules/workspaces-good/modules`.
/// The ruff adapter calls `self.io.canonicalize_path_str(path)` before passing the path
/// as a command argument.
#[test]
fn relative_scan_target_is_canonicalized_to_absolute_in_cmd() {
    use shared::common::taxonomy_adapter_name_vo::AdapterName;
    use shared::common::taxonomy_operation_error::LinterOperationError;
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::common::taxonomy_response_data_vo::ResponseData;
    use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
    use std::sync::Mutex;

    #[derive(Default)]
    struct CapturingLintExecutor {
        last_cmd: Mutex<Option<Vec<String>>>,
    }
    impl ICommandExecutorProtocol for CapturingLintExecutor {
        fn execute_command(
            &self,
            _: shared::common::taxonomy_common_vo::PatternList,
            _: FilePath,
            _: Option<shared::common::taxonomy_duration_vo::Timeout>,
        ) -> anyhow::Result<ResponseData> {
            Ok(ResponseData::default())
        }
        fn health_check(&self) -> anyhow::Result<ResponseData> {
            Ok(ResponseData::default())
        }
        fn exec_cmd_scan(
            &self,
            _: Vec<String>,
            _: FilePath,
            _: f64,
            _: Option<AdapterName>,
            _: &FilePath,
        ) -> Result<ResponseData, LinterOperationError> {
            Ok(ResponseData::default())
        }
        fn exec_cmd_adapter(
            &self,
            cmd: Vec<String>,
            _: FilePath,
            _: f64,
            _: AdapterName,
        ) -> Result<ResponseData, LinterOperationError> {
            *self.last_cmd.lock().unwrap() = Some(cmd);
            Ok(ResponseData::default())
        }
    }

    let executor = Arc::new(CapturingLintExecutor::default());
    let fs = Arc::new(MockFilesystem::with_canonicalize_prefix("/abs"));
    let adapter =
        external_lint_lint_arwaky::RuffAdapter::new(executor.clone(), None, fs.clone(), fs);
    let rel_path = FilePath::new("modules".to_string()).unwrap();
    let _ = adapter.scan(&rel_path).unwrap();

    let cmd = executor.last_cmd.lock().unwrap().take().unwrap();
    // The path argument to `ruff check` must be the canonicalized absolute path,
    // never the raw relative string `modules`. A relative arg is re-resolved
    // against the tool's working directory (the target itself), producing the
    // doubled path `workspaces-good/modules/workspaces-good/modules` and an
    // E902 io-error finding.
    assert!(
        cmd.contains(&"/abs/modules".to_string()),
        "ruff cmd must carry the canonicalized path, got {:?}",
        cmd
    );
    assert!(
        !cmd.contains(&"modules".to_string()),
        "ruff cmd must not carry the raw relative path, got {:?}",
        cmd
    );
}
