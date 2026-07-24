// PURPOSE: IDummyImportCheckerProtocol — unified contract for AES204 dummy import checking
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;

pub trait IDummyImportCheckerProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;

    fn check_dummy_imports(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );
    fn check_dummy_functions(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );
    fn check_dummy_impls(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );
    fn check_taxonomy_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );
    fn check_layer_contract_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );
    fn check_surface_logic(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );

    /// Run all dummy checks in one call, pre-computing shared data once.
    fn check_all_dummy(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        self.check_dummy_imports(file, content, violations, root_dir, layer_map);
        self.check_dummy_functions(file, content, violations, root_dir, layer_map);
        self.check_dummy_impls(file, content, violations, root_dir, layer_map);
        self.check_taxonomy_intent(file, content, violations, root_dir, layer_map);
        self.check_surface_logic(file, content, violations, root_dir, layer_map);
    }
}
