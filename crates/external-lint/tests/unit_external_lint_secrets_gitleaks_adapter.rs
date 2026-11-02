// Unit tests for GitleaksAdapter — gitleaks stdout JSON parsing and severity mapping.
use external_lint_lint_arwaky::capabilities_secrets_gitleaks_adapter::{
    GitleaksAdapter, map_gitleaks_severity,
};

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::MockFilesystem;
use shared_common::taxonomy_adapter_error::LinterOperationError;
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_job_vo::ResponseData;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_external_lint::taxonomy_duration_vo::Timeout;
use std::sync::{Arc, Mutex};

/// Capturing mock executor that records the command passed to `exec_cmd_adapter`
/// and returns a configurable `ResponseData`.
#[derive(Default)]
struct CapturingGitleaksExecutor {
    last_cmd: Mutex<Option<Vec<String>>>,
    response: ResponseData,
}

impl CapturingGitleaksExecutor {
    fn new(response: ResponseData) -> Self {
        Self {
            last_cmd: Mutex::new(None),
            response,
        }
    }

    fn captured_cmd(&self) -> Vec<String> {
        self.last_cmd.lock().unwrap().take().unwrap()
    }
}

impl ICommandExecutorProtocol for CapturingGitleaksExecutor {
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
        cmd: Vec<String>,
        _: FilePath,
        _: f64,
        _: AdapterName,
    ) -> Result<ResponseData, LinterOperationError> {
        *self.last_cmd.lock().unwrap() = Some(cmd);
        Ok(self.response.clone())
    }
}

/// Builds a `GitleaksAdapter` over a `MockFilesystem` (which implements both
/// `IFileSystemIOProtocol` and `IToolResolutionProtocol`). The caller keeps the
/// concrete `executor` so it can inspect the captured command after `scan()`.
fn make_adapter_with_executor(
    executor: Arc<CapturingGitleaksExecutor>,
    history: bool,
) -> GitleaksAdapter {
    let fs = Arc::new(MockFilesystem::new());
    GitleaksAdapter::new(
        executor as Arc<dyn ICommandExecutorProtocol>,
        fs.clone(),
        fs,
        history,
    )
}

fn gitleaks_target() -> FilePath {
    FilePath::new("pkg".to_string()).unwrap()
}

// ─── Severity mapping ───────────────────────────────────

#[test]
fn aws_access_key_id_maps_to_critical() {
    assert_eq!(
        map_gitleaks_severity("aws-access-key-id", &[]),
        Severity::CRITICAL
    );
}

#[test]
fn gcp_service_account_maps_to_critical() {
    assert_eq!(
        map_gitleaks_severity("gcp-service-account", &[]),
        Severity::CRITICAL
    );
}

#[test]
fn secret_suffix_key_maps_to_critical() {
    assert_eq!(map_gitleaks_severity("some-key", &[]), Severity::CRITICAL);
}

#[test]
fn secret_suffix_secret_maps_to_critical() {
    assert_eq!(
        map_gitleaks_severity("some-secret", &[]),
        Severity::CRITICAL
    );
}

#[test]
fn secret_suffix_token_maps_to_critical() {
    assert_eq!(map_gitleaks_severity("some-token", &[]), Severity::CRITICAL);
}

#[test]
fn secret_suffix_password_maps_to_critical() {
    assert_eq!(
        map_gitleaks_severity("some-password", &[]),
        Severity::CRITICAL
    );
}

#[test]
fn secret_suffix_webhook_maps_to_critical() {
    assert_eq!(
        map_gitleaks_severity("some-webhook", &[]),
        Severity::CRITICAL
    );
}

#[test]
fn secret_suffix_api_key_maps_to_critical() {
    assert_eq!(
        map_gitleaks_severity("some-api-key", &[]),
        Severity::CRITICAL
    );
}

#[test]
fn private_key_family_maps_to_critical() {
    assert_eq!(
        map_gitleaks_severity("private-key", &[]),
        Severity::CRITICAL
    );
    assert_eq!(
        map_gitleaks_severity("ssh-private-key", &[]),
        Severity::CRITICAL
    );
    assert_eq!(
        map_gitleaks_severity("pgp-private-key", &[]),
        Severity::CRITICAL
    );
    assert_eq!(
        map_gitleaks_severity("rsa-private-key", &[]),
        Severity::CRITICAL
    );
    assert_eq!(
        map_gitleaks_severity("ecdsa-private-key", &[]),
        Severity::CRITICAL
    );
}

#[test]
fn pci_tag_maps_to_critical() {
    assert_eq!(
        map_gitleaks_severity("some-rule", &["pci".to_string()]),
        Severity::CRITICAL
    );
}

#[test]
fn pii_tag_maps_to_critical() {
    assert_eq!(
        map_gitleaks_severity("some-rule", &["pii".to_string()]),
        Severity::CRITICAL
    );
}

#[test]
fn unknown_rule_no_tags_defaults_to_high() {
    assert_eq!(map_gitleaks_severity("unknown-rule", &[]), Severity::HIGH);
}

// ─── JSON parsing → LintResultList ──────────────────────

#[test]
fn scan_parses_two_findings_from_json_stdout() {
    let json = r#"{
        "findings": [
            {
                "rule": "aws-access-key-id",
                "description": "AWS access key id",
                "start_line": 10,
                "start_column": 5,
                "tags": ["aws-"]
            },
            {
                "rule": "some-token",
                "description": "A token",
                "tags": ["pii"]
            }
        ]
    }"#;
    let executor = Arc::new(CapturingGitleaksExecutor::new(ResponseData {
        stdout: json.to_string(),
        ..ResponseData::default()
    }));
    let adapter = make_adapter_with_executor(executor.clone(), false);
    let target = gitleaks_target();
    let results = adapter.scan(&target).unwrap();
    assert_eq!(results.len(), 2);
    let r0 = results.values[0].clone();
    assert_eq!(r0.code.code(), "gitleaks::aws-access-key-id");
    assert_eq!(r0.file.value(), target.value());
    assert_eq!(r0.line.value(), 10);
    assert_eq!(r0.column.value(), 5);
    assert_eq!(r0.severity, Severity::CRITICAL);
    let r1 = results.values[1].clone();
    assert_eq!(r1.code.code(), "gitleaks::some-token");
    assert_eq!(r1.file.value(), target.value());
    assert_eq!(r1.line.value(), 1);
    assert_eq!(r1.column.value(), 1);
    assert_eq!(r1.severity, Severity::CRITICAL);
}

// ─── Missing-binary / error path ────────────────────────

#[test]
fn scan_returncode_above_one_returns_empty_results() {
    let executor = Arc::new(CapturingGitleaksExecutor::new(ResponseData {
        stdout: "[{}]".to_string(),
        returncode: 2,
        ..ResponseData::default()
    }));
    let adapter = make_adapter_with_executor(executor, false);
    let results = adapter.scan(&gitleaks_target()).unwrap();
    assert_eq!(results.len(), 0);
}

#[test]
fn scan_malformed_json_returns_empty_results() {
    let executor = Arc::new(CapturingGitleaksExecutor::new(ResponseData {
        stdout: "{not valid json".to_string(),
        ..ResponseData::default()
    }));
    let adapter = make_adapter_with_executor(executor, false);
    let results = adapter.scan(&gitleaks_target()).unwrap();
    assert_eq!(results.len(), 0);
}

// ─── --no-git flag gating ───────────────────────────────

#[test]
fn history_false_includes_no_git_and_redact() {
    let executor = Arc::new(CapturingGitleaksExecutor::new(ResponseData {
        stdout: "[]".to_string(),
        ..ResponseData::default()
    }));
    let adapter = make_adapter_with_executor(executor.clone(), false);
    let _ = adapter.scan(&gitleaks_target()).unwrap();
    let cmd = executor.captured_cmd();
    assert!(
        cmd.iter().any(|c| c == "--no-git"),
        "cmd must contain --no-git when history=false, got {:?}",
        cmd
    );
    assert!(
        cmd.iter().any(|c| c == "--redact"),
        "cmd must contain --redact, got {:?}",
        cmd
    );
}

#[test]
fn history_true_excludes_no_git_keeps_redact() {
    let executor = Arc::new(CapturingGitleaksExecutor::new(ResponseData {
        stdout: "[]".to_string(),
        ..ResponseData::default()
    }));
    let adapter = make_adapter_with_executor(executor.clone(), true);
    let _ = adapter.scan(&gitleaks_target()).unwrap();
    let cmd = executor.captured_cmd();
    assert!(
        !cmd.iter().any(|c| c == "--no-git"),
        "cmd must not contain --no-git when history=true, got {:?}",
        cmd
    );
    assert!(
        cmd.iter().any(|c| c == "--redact"),
        "cmd must still contain --redact, got {:?}",
        cmd
    );
}
