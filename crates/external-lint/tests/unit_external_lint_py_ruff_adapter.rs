// Unit tests for RuffAdapter — ruff JSON output parsing.
use external_lint_lint_arwaky::capabilities_py_ruff_adapter::RuffAdapter;

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use shared::common::taxonomy_severity_vo::Severity;
use std::sync::Arc;

use mock_filesystem::MockFilesystem;

fn make_adapter() -> RuffAdapter {
    use shared::external_lint::IExternalLintExecutorProtocol;

    struct EmptyLintExecutor;
    impl IExternalLintExecutorProtocol for EmptyLintExecutor {
        fn exec_cmd_scan(
            &self,
            _: Vec<String>,
            _: shared::common::taxonomy_path_vo::FilePath,
            _: f64,
            _: Option<shared::common::taxonomy_adapter_name_vo::AdapterName>,
            _: &shared::common::taxonomy_path_vo::FilePath,
        ) -> Result<
            shared::common::taxonomy_response_data_vo::ResponseData,
            shared::common::taxonomy_operation_error::LinterOperationError,
        > {
            Ok(shared::common::taxonomy_response_data_vo::ResponseData::default())
        }
        fn exec_cmd_adapter(
            &self,
            _: Vec<String>,
            _: shared::common::taxonomy_path_vo::FilePath,
            _: f64,
            _: shared::common::taxonomy_adapter_name_vo::AdapterName,
        ) -> Result<
            shared::common::taxonomy_response_data_vo::ResponseData,
            shared::common::taxonomy_operation_error::LinterOperationError,
        > {
            Ok(shared::common::taxonomy_response_data_vo::ResponseData::default())
        }
        fn js_apply_fix(
            &self,
            _: &shared::common::taxonomy_path_vo::FilePath,
            _: &str,
            _: &str,
        ) -> Result<
            shared::common::taxonomy_message_vo::ComplianceStatus,
            shared::common::taxonomy_operation_error::LinterOperationError,
        > {
            Ok(shared::common::taxonomy_message_vo::ComplianceStatus::new(
                false,
            ))
        }
    }

    let executor: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(EmptyLintExecutor);
    RuffAdapter::new(executor, None, Arc::new(MockFilesystem::new()))
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
