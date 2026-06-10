//! agent_checker_container — Agent(container) implementation of ICheckerAggregate.
//! WIRES concrete capabilities implementations; allowed to import capabilities/infrastructure.
//! agent(orchestrator|coordinator) uses this through the contract trait.

use crate::code_analysis::capabilities_class_checker::ArchClassChecker;
use crate::code_analysis::capabilities_line_checker::ArchLineChecker;
use crate::code_analysis::contract_checker_aggregate::ICheckerAggregate;
use crate::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use crate::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::layer_rules::capabilities_compliance_analyzer::ArchComplianceAnalyzer;
use crate::layer_rules::capabilities_hierarchy_checker::SurfaceHierarchyChecker;
use crate::layer_rules::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use crate::layer_rules::capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
use crate::layer_rules::capabilities_layer_checker::ArchLayerChecker;
use crate::naming_rules::capabilities_naming_checker::ArchNamingChecker;
use crate::orphan_detector::capabilities_orphan_analyzer::OrphanGraphResolver;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::role_rules::capabilities_contract_role_auditor::ContractRoleChecker;
use crate::role_rules::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::collections::HashSet;

pub struct CheckerContainer {
    analyzer: ArchComplianceAnalyzer,
    import_forbidden_checker: ArchImportForbiddenChecker,
    import_mandatory_checker: ArchImportMandatoryChecker,
    line_checker: ArchLineChecker,
    class_checker: ArchClassChecker,
    taxonomy_checker: TaxonomyRoleChecker,
    contract_checker: ContractRoleChecker,
    naming_checker: ArchNamingChecker,
    layer_checker: ArchLayerChecker,
    orphan_resolver: OrphanGraphResolver,
}

impl CheckerContainer {
    pub fn new(config: ArchitectureConfig) -> Self {
        Self {
            analyzer: ArchComplianceAnalyzer::new(config),
            import_forbidden_checker: ArchImportForbiddenChecker::new(),
            import_mandatory_checker: ArchImportMandatoryChecker::new(),
            line_checker: ArchLineChecker::new(),
            class_checker: ArchClassChecker::new(),
            taxonomy_checker: TaxonomyRoleChecker::new(),
            contract_checker: ContractRoleChecker::new(),
            naming_checker: ArchNamingChecker::new(),
            layer_checker: ArchLayerChecker::new(),
            orphan_resolver: OrphanGraphResolver::new(),
        }
    }
}

impl ICheckerAggregate for CheckerContainer {
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String> {
        self.analyzer.detect_layer(file_path, root_dir)
    }

    fn get_layer_def(&self, layer: &str) -> Option<LayerDefinition> {
        self.analyzer.get_layer_def(layer).cloned()
    }

    fn check_mandatory_imports(
        &self,
        file: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        self.import_mandatory_checker
            .check_mandatory_imports(file, def, violations);
    }

    fn check_forbidden_imports(
        &self,
        file: &str,
        layer: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        self.import_forbidden_checker
            .check_forbidden_imports(file, layer, def, violations);
    }

    fn check_scope_forbidden_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        self.import_forbidden_checker
            .check_scope_forbidden_imports(file, config, violations);
    }

    fn check_legacy_import_rules(
        &self,
        file: &str,
        file_layer: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        self.import_forbidden_checker
            .check_legacy_import_rules(file, file_layer, config, violations);
    }

    fn check_line_counts(
        &self,
        file: &str,
        def: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        self.line_checker.check_line_counts(file, def, violations);
    }

    fn check_capability_routing(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        self.layer_checker
            .check_capability_routing(file, content, layer, violations);
    }

    fn check_mandatory_class_definition(
        &self,
        file: &str,
        def: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        self.class_checker
            .check_mandatory_class_definition(file, def, violations);
    }
    fn check_file_naming(
        &self,
        file: &str,
        filename: &str,
        layer: &Option<String>,
        def: Option<&LayerDefinition>,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        self.naming_checker
            .check_file_naming(file, filename, layer, def, config, violations);
    }

    fn check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        def: Option<&LayerDefinition>,
        layer: &Option<String>,
        violations: &mut Vec<LintResult>,
    ) {
        self.naming_checker
            .check_domain_suffixes(file, filename, def, layer, violations);
    }

    fn check_entity(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        self.taxonomy_checker
            .check_entity(file, content, violations);
    }

    fn check_error(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        self.taxonomy_checker.check_error(file, content, violations);
    }

    fn check_event(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        self.taxonomy_checker.check_event(file, content, violations);
    }

    fn check_constant(&self, file: &str, violations: &mut Vec<LintResult>) {
        self.taxonomy_checker.check_constant(file, violations);
    }

    fn check_aggregate(
        &self,
        file: &str,
        content: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        self.contract_checker
            .check_aggregate(file, content, def, violations);
    }

    fn check_surface_hierarchy(
        &self,
        files: &[FilePath],
        root_dir: &FilePath,
        violations: &mut LintResultList,
    ) {
        SurfaceHierarchyChecker::new().check_surface_hierarchy(files, root_dir, violations);
    }

    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        self.orphan_resolver.build_graph_context(files, root_dir)
    }

    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String> {
        self.orphan_resolver
            .identify_entry_points(files)
            .into_iter()
            .collect()
    }

    fn detect_cycle_edges(&self, edges: &[(String, String)]) -> bool {
        let deps: Vec<_> = edges
            .iter()
            .map(|(s, t)| {
                crate::layer_rules::capabilities_cycle_analyzer::DependencyEdge::new(
                    s.clone(),
                    t.clone(),
                )
            })
            .collect();
        !crate::layer_rules::capabilities_cycle_analyzer::detect_cycle_edges(&deps).is_empty()
    }
}
