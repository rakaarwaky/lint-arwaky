// PURPOSE: IDummyImportCheckerProtocol — unified contract for AES204 dummy import checking
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::import_rules::taxonomy_import_error::ImportError;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;

pub trait IDummyImportCheckerProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;

    fn check_dummy_imports(
        &self,
        file: &FilePath,
        content: &ContentString,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError>;

    fn check_dummy_functions(
        &self,
        file: &FilePath,
        content: &ContentString,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError>;

    fn check_dummy_impls(
        &self,
        file: &FilePath,
        content: &ContentString,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError>;

    fn check_taxonomy_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError>;

    fn check_layer_contract_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError>;

    fn check_surface_logic(
        &self,
        file: &FilePath,
        content: &ContentString,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError>;

    /// Run all dummy checks in one call, pre-computing shared data once.
    fn check_all_dummy(
        &self,
        file: &FilePath,
        content: &ContentString,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        let mut all = Vec::new();
        all.extend(self.check_dummy_imports(file, content, root_dir, layer_map)?);
        all.extend(self.check_dummy_functions(file, content, root_dir, layer_map)?);
        all.extend(self.check_dummy_impls(file, content, root_dir, layer_map)?);
        all.extend(self.check_taxonomy_intent(file, content, root_dir, layer_map)?);
        all.extend(self.check_layer_contract_intent(file, content, root_dir, layer_map)?);
        all.extend(self.check_surface_logic(file, content, root_dir, layer_map)?);
        Ok(all)
    }
}
