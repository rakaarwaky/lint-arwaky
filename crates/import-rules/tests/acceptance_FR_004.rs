// PURPOSE: Acceptance test for FR-004: Dummy or Forbidden Imports (AES204)
// Requirement: Imports pointing to dummy/stub code are detected.

use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::io::Write;

fn write_file(dir: &std::path::Path, name: &str, content: &str) {
    let mut file = std::fs::File::create(dir.join(name)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

/// FR-004: Dummy function detected with AES204
#[tokio::test]
async fn fr004_dummy_function_emits_aes204() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "capabilities_dummy.rs",
        r#"
use shared::common::taxonomy_path_vo::FilePath;

fn _use_mandatory_imports() {
    let _ = FilePath::new("x");
}

pub struct DummyCap;
"#,
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES204"),
        "FR-004: dummy function must emit AES204"
    );
}

/// FR-004: Dummy trait impl detected with AES204
#[tokio::test]
async fn fr004_dummy_trait_impl_emits_aes204() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "capabilities_stub_impl.rs",
        r#"
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::cli_commands::taxonomy_result_vo::LintResult;

pub struct StubChecker;

impl IUnusedImportProtocol for StubChecker {
    fn find_unused_imports(&self, _p: &FilePath) -> Vec<LintMessage> {
        todo!()
    }
    fn check_unused_imports(&self, _f: &str, _c: &str, _v: &mut Vec<LintResult>) {
        todo!()
    }
}
"#,
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES204"),
        "FR-004: dummy trait impl must emit AES204"
    );
}

/// FR-004: Real implementation does not emit AES204
#[tokio::test]
async fn fr004_real_impl_passes() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "capabilities_real.rs",
        r#"
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::cli_commands::taxonomy_result_vo::LintResult;

pub struct RealChecker;

impl IUnusedImportProtocol for RealChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage> {
        let content = std::fs::read_to_string(path.value()).unwrap_or_default();
        if content.is_empty() { return vec![]; }
        vec![LintMessage::new("found")]
    }
    fn check_unused_imports(&self, file: &str, content: &str, v: &mut Vec<LintResult>) {
        if content.contains("unused") {
            v.push(LintResult::new_arch(file, 1, "AES203",
                shared::common::taxonomy_severity_vo::Severity::MEDIUM, "unused"));
        }
    }
}
"#,
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let aes204: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES204")
        .collect();
    assert!(
        aes204.is_empty(),
        "FR-004: real implementation must not emit AES204"
    );
}
