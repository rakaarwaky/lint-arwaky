// PURPOSE: IImportMandatoryProtocol — exclusive contract for mandatory import checks (AES202)

use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::taxonomy_definition_vo::LayerDefinition;
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

    /// Check mandatory imports from layer definition (global layer rules).
    fn check_mandatory_imports(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );

    /// Check mandatory imports from per-rule scope definitions (fine-grained, per-suffix rules).
    fn check_scope_mandatory_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    );
}
