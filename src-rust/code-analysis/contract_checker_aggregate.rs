// PURPOSE: ICheckerAggregate — Contract trait bundling ALL checker operations for agent layer.

use crate::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::source_parsing::taxonomy_path_vo::FilePath;

/// The checker protocol that agents implementing this aggregate must use.
pub type CheckerProtocol = Box<dyn ILineCheckerProtocol>;

pub trait ICheckerAggregate: Send + Sync {
    // Layer detection
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String>;
    fn get_layer_def(&self, layer: &str) -> Option<LayerDefinition>;

    // Import rule checks
    fn check_mandatory_imports(
        &self,
        file: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );
    fn check_forbidden_imports(
        &self,
        file: &str,
        layer: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );
    fn check_scope_forbidden_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    );
    fn check_legacy_import_rules(
        &self,
        file: &str,
        file_layer: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    );

    // Line checks
    fn check_line_counts(
        &self,
        file: &str,
        def: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    );

    // Layer checks
    fn check_capability_routing(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );

    // Class & naming checks
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        def: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    );
    fn check_file_naming(
        &self,
        file: &str,
        filename: &str,
        layer: &Option<String>,
        def: Option<&LayerDefinition>,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    );
    fn check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        def: Option<&LayerDefinition>,
        layer: &Option<String>,
        violations: &mut Vec<LintResult>,
    );

    // Taxonomy & contract role checks
    fn check_entity(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_error(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_event(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_constant(&self, file: &str, violations: &mut Vec<LintResult>);
    fn check_aggregate(
        &self,
        file: &str,
        content: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );

    // Surface hierarchy
    fn check_surface_hierarchy(
        &self,
        files: &[FilePath],
        root_dir: &FilePath,
        violations: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    );

    // Orphan detection
    fn build_orphan_graph_context(
        &self,
        files: &[String],
        root_dir: &str,
    ) -> crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &[String]) -> std::collections::HashSet<String>;

    // Cycle detection
    fn detect_cycle_edges(&self, edges: &[(String, String)]) -> bool;
}
