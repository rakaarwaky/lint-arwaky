// Unit tests for BanditAdapter — bandit stdout JSON parsing.
use external_lint_lint_arwaky::capabilities_py_bandit_adapter::BanditAdapter;

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use shared_common::taxonomy_adapter_error::LinterOperationError;
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_job_vo::ResponseData;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::taxonomy_duration_vo::Timeout;
use std::sync::Arc;

use mock_filesystem::MockFilesystem;

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

fn make_adapter() -> BanditAdapter {
    let executor: Arc<dyn ICommandExecutorProtocol> = Arc::new(MockCmdExecutor);
    BanditAdapter::new(
        executor,
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    )
}

#[test]
fn high_confidence_high_severity_maps_to_critical() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("HIGH", "HIGH"), Severity::CRITICAL);
}

#[test]
fn high_severity_low_confidence_maps_to_high() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("HIGH", "LOW"), Severity::HIGH);
    assert_eq!(adapter.map_severity("HIGH", "MEDIUM"), Severity::HIGH);
}

#[test]
fn medium_severity_any_confidence_maps_to_medium() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("MEDIUM", "HIGH"), Severity::MEDIUM);
    assert_eq!(adapter.map_severity("MEDIUM", "LOW"), Severity::MEDIUM);
}

#[test]
fn low_severity_any_confidence_maps_to_low() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("LOW", "HIGH"), Severity::LOW);
    assert_eq!(adapter.map_severity("LOW", "LOW"), Severity::LOW);
}

#[test]
fn unknown_severity_defaults_to_medium() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("UNKNOWN", "HIGH"), Severity::MEDIUM);
}

/// Regression #913/#919: Bandit JSON has no column field. The adapter must
/// use the 1-based column default (not the `line_range` span) and must store
/// the tool-qualified `bandit::` code.
#[test]
fn scan_uses_column_default_and_qualified_code() {
    use shared_common::taxonomy_adapter_error::LinterOperationError;
    use shared_common::taxonomy_common_vo::PatternList;
    use shared_common::taxonomy_job_vo::ResponseData;
    use shared_common::taxonomy_path_vo::FilePath;
    use shared_external_lint::ICommandExecutorProtocol;
    use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
    use shared_external_lint::taxonomy_duration_vo::Timeout;
    use std::sync::Mutex;

    #[derive(Default)]
    struct FindingExecutor {
        used: Mutex<bool>,
    }
    impl ICommandExecutorProtocol for FindingExecutor {
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
            *self.used.lock().unwrap() = true;
            let json = r#"{
                "results": [{
                    "filename": "x.py",
                    "line_number": 42,
                    "line_range": [42, 44],
                    "test_id": "B104",
                    "issue_text": "Any() use",
                    "issue_severity": "HIGH",
                    "issue_confidence": "HIGH"
                }]
            }"#;
            Ok(ResponseData {
                stdout: json.to_string(),
                ..ResponseData::default()
            })
        }
    }

    let fs = Arc::new(MockFilesystem::with_python_flag(true));
    let executor: Arc<dyn ICommandExecutorProtocol> = Arc::new(FindingExecutor::default());
    let adapter = BanditAdapter::new(executor, None, fs.clone(), fs);
    let target = FilePath::new("pkg".to_string()).unwrap();
    let results = adapter.scan(&target).unwrap();
    assert_eq!(results.len(), 1);
    let r = results.values[0].clone();
    assert_eq!(r.line.value(), 42, "line comes from line_number");
    assert_eq!(
        r.column.value(),
        1,
        "column must be the 1-based default, not the line_range span (#913)"
    );
    assert_eq!(
        r.code.code(),
        "bandit::B104",
        "code must be tool-qualified bandit::B104 (#919)"
    );
    assert_eq!(r.severity, Severity::CRITICAL);
}
