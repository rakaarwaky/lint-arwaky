// PURPOSE: import-domain capability contracts (AES102 `_protocol`).
//
// One file for the import feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_lint_result_vo::LintResultList;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_source_vo::ContentString;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::filesystem::taxonomy_filesystem_vo::ImportEntry;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::import_rules::taxonomy_import_error::ImportError;
use std::collections::HashMap;

pub trait ICycleImportProtocol: Send + Sync {
    fn scan(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &[FilePath],
        root_dir: &FilePath,
        content_map: &HashMap<String, String>,
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Vec<LintResult>;

    fn check_cycles(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &crate::common::taxonomy_paths_vo::FilePathList,
        root_dir: &FilePath,
        content_map: &HashMap<String, String>,
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<Vec<LintResult>, ImportError>;

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;
    fn normalize_to_layer(&self, name: &str) -> LayerNameVO;
}

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

/// For each file, verify its imports do not reach any layer listed in the `forbidden` set.
/// Used by the import orchestrator as part of the AES201 gate.
///
/// `content_map` maps file path → file content. The orchestrator pre-reads files
/// and passes the map so capabilities don't do I/O directly.
/// `imports_map` maps file path → parsed ImportEntry list from filesystem crate's AST parser.
pub trait IImportForbiddenProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;
    fn check_forbidden_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        content_map: &HashMap<String, String>,
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<LintResultList, ImportError>;
}

/// For each file, check that at least one import targets each layer in the `mandatory` set.
/// Used by the import orchestrator as part of the AES202 gate.
///
/// `content_map` maps file path → file content. The orchestrator pre-reads files
/// and passes the map so capabilities don't do I/O directly.
/// `imports_map` maps file path → parsed ImportEntry list from filesystem crate's AST parser.
pub trait IImportMandatoryProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;
    fn run_mandatory_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        content_map: &HashMap<String, String>,
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<LintResultList, ImportError>;
}

pub trait IUnusedImportProtocol: Send + Sync {
    /// Find unused imports in a file. `content` is pre-read file content.
    /// `used_identifiers` — pre-extracted identifiers from filesystem's tree-sitter AST
    /// (from ParseMetadata). Empty slice when no AST data is available.
    fn find_unused_imports(
        &self,
        path: &FilePath,
        content: &str,
        import_entries: &[ImportEntry],
        used_identifiers: &[String],
    ) -> Result<Vec<LintMessage>, ImportError>;

    /// Check unused imports given file path and content.
    /// file_path is needed for AST parser dispatch (language detection by extension).
    /// `used_identifiers` — pre-extracted identifiers from filesystem's tree-sitter AST.
    /// `implemented_traits` — cross-file map: trait_name → [type_names that implement it].
    /// Used for implicit Rust trait usage detection (method dispatch scope).
    fn check_unused_imports(
        &self,
        file: &str,
        content: &str,
        import_entries: &[ImportEntry],
        used_identifiers: &[String],
        implemented_traits: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<LintResult>, ImportError>;
}
