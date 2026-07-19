// PURPOSE: IImportForbiddenProtocol — exclusive contract for forbidden import checks (AES201)

use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

/// Exclusive protocol for forbidden import checks (AES201).
#[async_trait]
pub trait IImportForbiddenProtocol: Send + Sync {
    /// Returns the rule identifier (e.g., "AES201").
    fn rule_name(&self) -> crate::taxonomy_layer_vo::Identity;

    /// Run both layer-level and scope-level forbidden import checks on every file.
    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
