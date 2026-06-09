//! contract_checker_aggregate — Aggregate trait bundling all checker operations.
//! Enables agent(orchestrator|coordinator) to use checkers through contracts
//! without directly importing capabilities/infrastructure implementations.

use crate::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::source_parsing::taxonomy_path_vo::FilePath;

/// Create a LintResult — shared by all inline checkers.
pub fn mk_result(file: &str, line: usize, code: &str, sev: Severity, msg: &str) -> LintResult {
    LintResult {
        file: FilePath::new(file.to_string()).unwrap_or_default(),
        line: LineNumber::new(line as i64),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(msg),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}

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
    fn check_surface_imports(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
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

    // Inline inspector checks (moved from capabilities inspectors for AES001 compliance)
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_mandatory_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_agent_any_bypass(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_agent_role(&self, file: &str, content: &str, layer: &str, violations: &mut Vec<LintResult>);
    fn check_surface_role(&self, file: &str, content: &str, layer: &str, violations: &mut Vec<LintResult>);
    fn check_single_bottleneck(&self, file: &str, content: &str, layer: &str, violations: &mut Vec<LintResult>);
    fn check_missing_vo(&self, file: &str, content: &str, layer: &str, violations: &mut Vec<LintResult>);
}
