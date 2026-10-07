// Unit tests for MyPyAdapter — mypy stdout parsing.
use external_lint_lint_arwaky::capabilities_py_mypy_adapter::MyPyAdapter;

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::MockFilesystem;
use shared_common::taxonomy_adapter_error::LinterOperationError;
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_job_vo::ResponseData;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::taxonomy_duration_vo::Timeout;
use std::sync::Arc;

/// Backs the FR-006 `ICommandExecutorProtocol` seam: raw execution plus the
/// `exec_cmd_*` error-mapping wrappers. Returns empty output for every call.
#[allow(dead_code)]
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

#[allow(dead_code)]
fn make_adapter() -> MyPyAdapter {
    let executor: Arc<dyn ICommandExecutorProtocol> = Arc::new(MockCmdExecutor);
    MyPyAdapter::new(
        executor,
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    )
}

#[test]
fn notes_map_to_low() {
    assert_eq!(
        MyPyAdapter::map_severity("note", "module is installed, but cannot be found"),
        Severity::LOW
    );
}

#[test]
fn syntax_errors_map_to_critical() {
    assert_eq!(
        MyPyAdapter::map_severity("error", "syntax error in x.py"),
        Severity::CRITICAL
    );
}

#[test]
fn warnings_map_to_medium() {
    assert_eq!(
        MyPyAdapter::map_severity("warning", "returning a value from a function"),
        Severity::MEDIUM
    );
}

#[test]
fn errors_default_to_high() {
    assert_eq!(
        MyPyAdapter::map_severity("error", "incompatible return value type"),
        Severity::HIGH
    );
}

/// Regression #919: mypy codes must be stored tool-qualified as
/// `mypy::<code>` so tool-qualified `ignored_rules` entries match.
#[test]
fn scan_stores_tool_qualified_mypy_code() {
    use shared_common::taxonomy_adapter_error::LinterOperationError;
    use shared_common::taxonomy_job_vo::ResponseData;
    use shared_common::taxonomy_path_vo::FilePath;
    use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
    use std::sync::Mutex;

    #[derive(Default)]
    struct MypyFindingExecutor {
        used: Mutex<bool>,
    }
    impl ICommandExecutorProtocol for MypyFindingExecutor {
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
            let stdout = "x.py:3:10: error: Argument 1 has incompatible type [arg-type]\n";
            Ok(ResponseData {
                stdout: stdout.to_string(),
                ..ResponseData::default()
            })
        }
    }

    let executor: Arc<dyn ICommandExecutorProtocol> = Arc::new(MypyFindingExecutor::default());
    let fs = Arc::new(MockFilesystem::with_python_flag(true));
    let adapter = MyPyAdapter::new(executor, None, fs.clone(), fs);
    let target = FilePath::new("pkg".to_string()).unwrap();
    let results = adapter.scan(&target).unwrap();
    assert_eq!(results.len(), 1);
    let r = results.values[0].clone();
    assert_eq!(
        r.code.code(),
        "mypy::arg-type",
        "code must be tool-qualified mypy::arg-type (#919)"
    );
    assert_eq!(r.line.value(), 3);
    assert_eq!(r.column.value(), 10);
    assert_eq!(r.severity, Severity::HIGH);
}
