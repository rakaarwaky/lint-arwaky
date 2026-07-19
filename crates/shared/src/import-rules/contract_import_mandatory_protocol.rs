// PURPOSE: IImportMandatoryProtocol — exclusive contract for mandatory import checks (AES202)

use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::taxonomy_layer_vo::Identity;
use async_trait::async_trait;

/// Exclusive protocol for mandatory import checks (AES202).
#[async_trait]
pub trait IImportMandatoryProtocol: Send + Sync {
    /// Returns the rule identifier (e.g., "AES202").
    fn rule_name(&self) -> Identity;

    /// Run both layer-level and scope-level mandatory import checks on every file.
    async fn run_mandatory_imports(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
