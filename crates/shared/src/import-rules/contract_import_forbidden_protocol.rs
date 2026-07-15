// PURPOSE: IImportForbiddenProtocol — exclusive contract for forbidden import checks (AES201)

use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::taxonomy_definition_vo::LayerDefinition;
use crate::taxonomy_layer_vo::{Identity, LayerNameVO};
use async_trait::async_trait;

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

    /// Check forbidden imports from layer definition (global layer rules).
    fn check_forbidden_imports_layer(
        &self,
        file_path: &FilePath,
        layer_name: &str,
        definition: &LayerDefinition,
        default_forbidden: &[String],
        violations: &mut Vec<LintResult>,
        processed: &mut std::collections::HashSet<(String, usize, String)>,
    );

    /// Check forbidden imports from per-rule scope definitions (fine-grained, per-suffix rules).
    fn check_scope_forbidden_imports(
        &self,
        file_path: &FilePath,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
        processed: &mut std::collections::HashSet<(String, usize, String)>,
    );

    /// Core: scan import lines against a list of forbidden layers and build violations.
    fn check_imports_against_forbidden(
        &self,
        file_path: &FilePath,
        import_lines: &[(LineNumber, LineContentVO)],
        forbidden_list: &[String],
        source_layer: &LayerNameVO,
        allowed_values: &[String],
        violations: &mut Vec<LintResult>,
        processed: &mut std::collections::HashSet<(String, usize, String)>,
    );
}
