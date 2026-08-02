// PURPOSE: IImportMandatoryProtocol — exclusive contract for mandatory import checks (AES202)
// Verifies that every file imports from the layers it MUST depend on per its layer scope.
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::import_rules::taxonomy_import_error::ImportError;
use std::collections::HashMap;

/// For each file, check that at least one import targets each layer in the `mandatory` set.
/// Used by the import orchestrator as part of the AES202 gate.
///
/// `content_map` maps file path → file content. The orchestrator pre-reads files
/// and passes the map so capabilities don't do I/O directly.
pub trait IImportMandatoryProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;
    fn run_mandatory_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        content_map: &HashMap<String, String>,
    ) -> Result<LintResultList, ImportError>;
}
