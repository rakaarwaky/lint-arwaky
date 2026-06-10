// PURPOSE: CheckerContainerAggregate — ICheckerAggregate DI wiring for all checker implementations

use crate::code_analysis::capabilities_check_bypass_checker::BypassChecker;
use crate::code_analysis::capabilities_class_checker::ArchClassChecker;
use crate::code_analysis::capabilities_dead_inheritance_checker::DeadInheritanceChecker;
use crate::code_analysis::capabilities_inline_unused_checker::InlineUnusedChecker;
use crate::code_analysis::capabilities_line_checker::ArchLineChecker;
use crate::code_analysis::capabilities_mandatory_inheritance_checker::MandatoryInheritanceChecker;
use crate::code_analysis::capabilities_missing_vo_checker::MissingVoChecker;
use crate::code_analysis::capabilities_single_bottleneck_checker::SingleBottleneckChecker;
use crate::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use crate::code_analysis::contract_checker_aggregate::ICheckerAggregate;
use crate::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use crate::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use crate::code_analysis::contract_inline_unused_protocol::IInlineUnusedProtocol;
use crate::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use crate::code_analysis::contract_mandatory_inheritance_protocol::IMandatoryInheritanceProtocol;
use crate::code_analysis::contract_missing_vo_protocol::IMissingVoProtocol;
use crate::code_analysis::contract_single_bottleneck_protocol::ISingleBottleneckProtocol;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::layer_rules::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer;
use crate::role_rules::capabilities_surface_role_auditor::SurfaceRoleChecker;
use crate::layer_rules::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use crate::layer_rules::capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
use crate::role_rules::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use crate::layer_rules::capabilities_naming_checker::ArchNamingChecker;
use crate::orphan_detector::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_infrastructure_analyzer::InfrastructureOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use crate::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::role_rules::capabilities_contract_role_auditor::ContractRoleChecker;
use crate::role_rules::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

pub struct CheckerContainer {
    analyzer: LayerDetectionAnalyzer,
    import_forbidden_checker: ArchImportForbiddenChecker,
    import_mandatory_checker: ArchImportMandatoryChecker,
    line_checker: ArchLineChecker,
    class_checker: ArchClassChecker,
    bypass_checker: BypassChecker,
    missing_vo_checker: MissingVoChecker,
    inline_unused_checker: InlineUnusedChecker,
    dead_inheritance_checker: DeadInheritanceChecker,
    single_bottleneck_checker: SingleBottleneckChecker,
    mandatory_inheritance_checker: MandatoryInheritanceChecker,
    taxonomy_checker: TaxonomyRoleChecker,
    contract_checker: ContractRoleChecker,
    naming_checker: ArchNamingChecker,
    capabilities_role_checker: CapabilitiesRoleChecker,
    orphan_analyzer: Arc<dyn IOrphanAggregate>,
}

impl CheckerContainer {
    pub fn new(config: ArchitectureConfig) -> Self {
        let orphan_analyzer: Arc<dyn IOrphanAggregate> = Arc::new(ArchOrphanAnalyzer::new(
            Arc::new(TaxonomyOrphanAnalyzer::new()),
            Arc::new(ContractOrphanAnalyzer::new()),
            Arc::new(CapabilitiesOrphanAnalyzer::new()),
            Arc::new(InfrastructureOrphanAnalyzer::new()),
            Arc::new(AgentOrphanAnalyzer::new()),
            Arc::new(SurfacesOrphanAnalyzer::new()),
        ));
        Self {
            analyzer: LayerDetectionAnalyzer::new(config),
            import_forbidden_checker: ArchImportForbiddenChecker::new(),
            import_mandatory_checker: ArchImportMandatoryChecker::new(),
            line_checker: ArchLineChecker::new(),
            class_checker: ArchClassChecker::new(),
            bypass_checker: BypassChecker::new(),
            missing_vo_checker: MissingVoChecker::new(),
            inline_unused_checker: InlineUnusedChecker::new(),
            dead_inheritance_checker: DeadInheritanceChecker::new(),
            single_bottleneck_checker: SingleBottleneckChecker::new(),
            mandatory_inheritance_checker: MandatoryInheritanceChecker::new(),
            taxonomy_checker: TaxonomyRoleChecker::new(),
            contract_checker: ContractRoleChecker::new(),
            naming_checker: ArchNamingChecker::new(),
            capabilities_role_checker: CapabilitiesRoleChecker::new(),
            orphan_analyzer,
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
        config: &crate::config_system::taxonomy_config_vo::ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        self.import_forbidden_checker
            .check_scope_forbidden_imports(file, config, violations);
    }

    fn check_scope_mandatory_imports(
        &self,
        file: &str,
        config: &crate::config_system::taxonomy_config_vo::ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        eprintln!("[DEBUG CONTAINER] check_scope_mandatory_imports called for file: {}", file);
        self.import_mandatory_checker
            .check_scope_mandatory_imports(file, config, violations);
    }

    fn check_legacy_import_rules(
        &self,
        file: &str,
        file_layer: &str,
        config: &crate::config_system::taxonomy_config_vo::ArchitectureConfig,
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
        self.capabilities_role_checker
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
        config: &crate::config_system::taxonomy_config_vo::ArchitectureConfig,
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

    fn check_bypass_comments(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        self.bypass_checker
            .check_bypass_comments(file, content, violations);
    }

    fn check_missing_vo(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        self.missing_vo_checker
            .check_missing_vo(file, content, layer, violations);
    }

    fn check_inline_unused_imports(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        self.inline_unused_checker
            .check_unused_imports(file, content, violations);
    }

    fn check_dead_inheritance(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        self.dead_inheritance_checker
            .check_dead_inheritance(file, content, violations);
    }

    fn check_single_bottleneck(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        self.single_bottleneck_checker
            .check_single_bottleneck(file, content, layer, violations);
    }

    fn check_mandatory_inheritance(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        self.mandatory_inheritance_checker
            .check_mandatory_inheritance(file, content, layer, violations);
    }

    fn check_surface_hierarchy(
        &self,
        files: &[FilePath],
        root_dir: &FilePath,
        violations: &mut LintResultList,
    ) {
        SurfaceRoleChecker::new().check_surface_hierarchy(files, root_dir, violations);
    }

    fn orphan_aggregate(
        &self,
    ) -> Arc<dyn crate::orphan_detector::contract_orphan_aggregate::IOrphanAggregate> {
        self.orphan_analyzer.clone()
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
