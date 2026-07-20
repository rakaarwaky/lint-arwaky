// PURPOSE: IDummyImportCheckerProtocol — unified contract for AES204 dummy import checking
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::taxonomy_layer_vo::Identity;

pub trait IDummyImportCheckerProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;

    fn check_dummy_imports(
        &self, file: &FilePath, content: &ContentString,
        violations: &mut Vec<LintResult>, root_dir: &FilePath,
    );
    fn check_dummy_functions(
        &self, file: &FilePath, content: &ContentString,
        violations: &mut Vec<LintResult>, root_dir: &FilePath,
    );
    fn check_dummy_impls(
        &self, file: &FilePath, content: &ContentString,
        violations: &mut Vec<LintResult>, root_dir: &FilePath,
    );
    fn check_taxonomy_intent(
        &self, file: &FilePath, content: &ContentString,
        violations: &mut Vec<LintResult>, root_dir: &FilePath,
    );
    fn check_layer_contract_intent(
        &self, file: &FilePath, content: &ContentString,
        violations: &mut Vec<LintResult>, root_dir: &FilePath,
    );
    fn check_surface_logic(
        &self, file: &FilePath, content: &ContentString,
        violations: &mut Vec<LintResult>, root_dir: &FilePath,
    );
}
