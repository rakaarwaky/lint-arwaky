// PURPOSE: CheckerContainer — DI wiring for all checker implementations (getters only, no orchestration)

use crate::code_analysis::capabilities_check_bypass_checker::BypassChecker;
use crate::code_analysis::capabilities_class_checker::ArchClassChecker;
use crate::code_analysis::capabilities_dead_inheritance_checker::DeadInheritanceChecker;
use crate::code_analysis::capabilities_inline_unused_checker::InlineUnusedChecker;
use crate::code_analysis::capabilities_line_checker::ArchLineChecker;
use crate::code_analysis::capabilities_mandatory_inheritance_checker::MandatoryInheritanceChecker;
use crate::code_analysis::capabilities_missing_vo_checker::MissingVoChecker;
use crate::code_analysis::capabilities_single_bottleneck_checker::SingleBottleneckChecker;
use crate::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::layer_rules::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use crate::layer_rules::capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
use crate::layer_rules::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer;
use crate::layer_rules::capabilities_naming_checker::ArchNamingChecker;
use crate::orphan_detector::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_infrastructure_analyzer::InfrastructureOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use crate::orphan_detector::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use crate::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use crate::role_rules::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use crate::role_rules::capabilities_contract_role_auditor::ContractRoleChecker;
use crate::role_rules::capabilities_surface_role_auditor::SurfaceRoleChecker;
use crate::role_rules::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
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
    surface_checker: SurfaceRoleChecker,
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
            surface_checker: SurfaceRoleChecker::new(),
            orphan_analyzer,
        }
    }

    // --- Getters only: no logic, no method calls to protocols ---

    pub fn analyzer(&self) -> &LayerDetectionAnalyzer {
        &self.analyzer
    }

    pub fn naming_checker(&self) -> &ArchNamingChecker {
        &self.naming_checker
    }

    pub fn import_mandatory_checker(&self) -> &ArchImportMandatoryChecker {
        &self.import_mandatory_checker
    }

    pub fn import_forbidden_checker(&self) -> &ArchImportForbiddenChecker {
        &self.import_forbidden_checker
    }

    pub fn line_checker(&self) -> &ArchLineChecker {
        &self.line_checker
    }

    pub fn class_checker(&self) -> &ArchClassChecker {
        &self.class_checker
    }

    pub fn bypass_checker(&self) -> &BypassChecker {
        &self.bypass_checker
    }

    pub fn missing_vo_checker(&self) -> &MissingVoChecker {
        &self.missing_vo_checker
    }

    pub fn inline_unused_checker(&self) -> &InlineUnusedChecker {
        &self.inline_unused_checker
    }

    pub fn dead_inheritance_checker(&self) -> &DeadInheritanceChecker {
        &self.dead_inheritance_checker
    }

    pub fn single_bottleneck_checker(&self) -> &SingleBottleneckChecker {
        &self.single_bottleneck_checker
    }

    pub fn mandatory_inheritance_checker(&self) -> &MandatoryInheritanceChecker {
        &self.mandatory_inheritance_checker
    }

    pub fn taxonomy_checker(&self) -> &TaxonomyRoleChecker {
        &self.taxonomy_checker
    }

    pub fn contract_checker(&self) -> &ContractRoleChecker {
        &self.contract_checker
    }

    pub fn capabilities_role_checker(&self) -> &CapabilitiesRoleChecker {
        &self.capabilities_role_checker
    }

    pub fn surface_checker(&self) -> &SurfaceRoleChecker {
        &self.surface_checker
    }

    pub fn orphan_aggregate(&self) -> &Arc<dyn IOrphanAggregate> {
        &self.orphan_analyzer
    }
}

impl ILayerDetectionAggregate for CheckerContainer {
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String> {
        self.analyzer.detect_layer(file_path, root_dir)
    }

    fn get_layer_def(&self, layer: &str) -> Option<LayerDefinition> {
        self.analyzer.get_layer_def(layer).cloned()
    }
}
