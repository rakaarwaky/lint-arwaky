// PURPOSE: IImportForbiddenProtocol — exclusive contract for forbidden import checks (AES201)

use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::taxonomy_layer_vo::{Identity, LayerNameVO};
use async_trait::async_trait;

pub struct ForbiddenRuleConfig<'a> {
    pub forbidden_list: &'a [String],
    pub source_layer: &'a LayerNameVO,
    pub allowed_values: &'a [String],
}

/// Exclusive protocol for forbidden import checks (AES201).
#[async_trait]
pub trait IImportForbiddenProtocol: Send + Sync {
    /// Returns the rule identifier (e.g., "AES201").
    fn rule_name(&self) -> Identity;

    /// Run both layer-level and scope-level forbidden import checks on every file.
    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
