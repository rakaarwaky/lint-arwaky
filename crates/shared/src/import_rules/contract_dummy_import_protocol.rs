// PURPOSE: IDummyImportCheckerProtocol — unified contract for AES204 dummy import checking
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::filesystem::taxonomy_filesystem_vo::ImportEntry;
use crate::import_rules::taxonomy_import_error::ImportError;
use std::collections::HashMap;

pub trait IDummyImportCheckerProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;

    fn check_dummy_imports(
        &self,
        file: &FilePath,
        content: &ContentString,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
        import_entries: &[ImportEntry],
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
        import_entries: &[ImportEntry],
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
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<Vec<LintResult>, ImportError> {
        let import_entries = imports_map
            .get(file.value())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        let mut all = Vec::new();
        all.extend(self.check_dummy_imports(file, content, root_dir, layer_map, import_entries)?);
        all.extend(self.check_dummy_functions(file, content, root_dir, layer_map)?);
        all.extend(self.check_dummy_impls(file, content, root_dir, layer_map)?);
        all.extend(self.check_taxonomy_intent(
            file,
            content,
            root_dir,
            layer_map,
            import_entries,
        )?);
        all.extend(self.check_surface_logic(file, content, root_dir, layer_map)?);
        Ok(all)
    }
}
