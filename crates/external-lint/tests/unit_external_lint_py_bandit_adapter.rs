// Unit tests for BanditAdapter — bandit stdout JSON parsing.
use external_lint_lint_arwaky::capabilities_py_bandit_adapter::BanditAdapter;

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use shared::common::taxonomy_severity_vo::Severity;
use std::sync::Arc;

use mock_filesystem::MockFilesystem;

fn make_adapter() -> BanditAdapter {
    use shared::common::taxonomy_adapter_name_vo::AdapterName;
    use shared::common::taxonomy_message_vo::ComplianceStatus;
    use shared::common::taxonomy_operation_error::LinterOperationError;
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::common::taxonomy_response_data_vo::ResponseData;
    use shared::external_lint::IExternalLintExecutorProtocol;

    struct EmptyLintExecutor;
    impl IExternalLintExecutorProtocol for EmptyLintExecutor {
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
        fn js_apply_fix(
            &self,
            _: &FilePath,
            _: &str,
            _: &str,
        ) -> Result<ComplianceStatus, LinterOperationError> {
            Ok(ComplianceStatus::new(false))
        }
    }

    let executor: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(EmptyLintExecutor);
    BanditAdapter::new(executor, None, Arc::new(MockFilesystem::new()))
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
