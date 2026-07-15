// PURPOSE: IDummyImportCheckerProtocol — unified contract for AES204 dummy import checking
// Implementation: crates/import-rules/src/capabilities_dummy_import_checker.rs → DummyImportChecker
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use crate::common::taxonomy_path_vo::FilePath;
use crate::taxonomy_layer_vo::Identity;

/// Unified Contract / Port for the AES204 rule (Dummy Import Checker).
/// This trait contains all AES204 sub-checks and helpers.
pub trait IDummyImportCheckerProtocol: Send + Sync {
    // ─── Architectural Protocol Methods ───

    fn rule_name(&self) -> Identity;

    // ─── Sub-check Methods ───

    fn check_dummy_imports(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn ILayerDetectionProtocol,
        root_dir: &FilePath,
    );

    fn check_dummy_functions(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn ILayerDetectionProtocol,
        root_dir: &FilePath,
    );

    fn check_dummy_impls(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn ILayerDetectionProtocol,
        root_dir: &FilePath,
    );

    fn check_taxonomy_intent(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn ILayerDetectionProtocol,
        root_dir: &FilePath,
    );

    fn check_layer_contract_intent(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn ILayerDetectionProtocol,
        root_dir: &FilePath,
    );

    fn check_surface_logic(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
