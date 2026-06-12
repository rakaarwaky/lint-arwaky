// PURPOSE: LocalContainer — defines CheckerContainer and RoleOrchestrator for code-analysis feature

use std::sync::Arc;

use crate::IBypassCheckerProtocol;
use crate::IMandatoryClassProtocol;
use crate::IDeadInheritanceProtocol;
use crate::IInlineUnusedProtocol;
use crate::ILayerDetectionAggregate;
use crate::ILineCheckerProtocol;
use crate::IMandatoryInheritanceProtocol;
use crate::ArchitectureConfig;
use crate::LintResult;
use crate::LintResultList;
use crate::IRoleAggregate;
use crate::IAnalyzer;
use crate::ICycleAnalysisProtocol;
use crate::capabilities_check_bypass_checker::BypassChecker;
use crate::capabilities_inline_unused_checker::InlineUnusedChecker;
use crate::capabilities_dead_inheritance_checker::DeadInheritanceChecker;
use crate::capabilities_mandatory_inheritance_checker::MandatoryInheritanceChecker;
use crate::capabilities_line_checker::ArchLineChecker;
use crate::capabilities_class_checker::ArchClassChecker;
use crate::capabilities_cycle_analyzer::DependencyCycleAnalyzer;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;

/// CheckerContainer holds all the protocol implementations for AES checking
pub struct CheckerContainer {
    bypass_checker: Arc<dyn IBypassCheckerProtocol>,
    inline_unused_checker: Arc<dyn IInlineUnusedProtocol>,
    dead_inheritance_checker: Arc<dyn IDeadInheritanceProtocol>,
    mandatory_inheritance_checker: Arc<dyn IMandatoryInheritanceProtocol>,
    capabilities_role_checker: Arc<dyn ICapabilitiesRoleProtocol>,
    line_checker: Arc<dyn ILineCheckerProtocol>,
    taxonomy_checker: Arc<dyn ITaxonomyProtocol>,
    contract_checker: Arc<dyn IContractProtocol>,
    class_checker: Arc<dyn IMandatoryClassProtocol>,
    naming_checker: Arc<dyn INamingProtocol>,
    import_mandatory_checker: Arc<dyn IImportMandatoryProtocol>,
    import_intent_checker: Arc<dyn IImportIntentProtocol>,
    import_forbidden_checker: Arc<dyn IImportForbiddenProtocol>,
    cycle_analyzer: Arc<dyn ICycleAnalysisProtocol>,
    surface_checker: Arc<dyn ISurfaceProtocol>,
    orphan_aggregate: Arc<dyn IOrphanAggregate>,
}

impl CheckerContainer {
    pub fn new() -> Self {
        // These will be initialized by init_global_checker
        // For now, panic if accessed before initialization
        Self {
            bypass_checker: Arc::new(BypassChecker {}),
            inline_unused_checker: Arc::new(InlineUnusedChecker {}),
            dead_inheritance_checker: Arc::new(DeadInheritanceChecker {}),
            mandatory_inheritance_checker: Arc::new(MandatoryInheritanceChecker {}),
            capabilities_role_checker: panic!("CheckerContainer not initialized"),
            line_checker: Arc::new(ArchLineChecker {}),
            taxonomy_checker: panic!("CheckerContainer not initialized"),
            contract_checker: panic!("CheckerContainer not initialized"),
            class_checker: Arc::new(ArchClassChecker {}),
            naming_checker: panic!("CheckerContainer not initialized"),
            import_mandatory_checker: panic!("CheckerContainer not initialized"),
            import_intent_checker: panic!("CheckerContainer not initialized"),
            import_forbidden_checker: panic!("CheckerContainer not initialized"),
            cycle_analyzer: Arc::new(DependencyCycleAnalyzer::new(ArchitectureConfig::default())),
            surface_checker: panic!("CheckerContainer not initialized"),
            orphan_aggregate: panic!("CheckerContainer not initialized"),
        }
    }

    pub fn bypass_checker(&self) -> &Arc<dyn IBypassCheckerProtocol> {
        &self.bypass_checker
    }

    pub fn inline_unused_checker(&self) -> &Arc<dyn IInlineUnusedProtocol> {
        &self.inline_unused_checker
    }

    pub fn dead_inheritance_checker(&self) -> &Arc<dyn IDeadInheritanceProtocol> {
        &self.dead_inheritance_checker
    }

    pub fn mandatory_inheritance_checker(&self) -> &Arc<dyn IMandatoryInheritanceProtocol> {
        &self.mandatory_inheritance_checker
    }

    pub fn capabilities_role_checker(&self) -> &Arc<dyn ICapabilitiesRoleProtocol> {
        &self.capabilities_role_checker
    }

    pub fn line_checker(&self) -> &Arc<dyn ILineCheckerProtocol> {
        &self.line_checker
    }

    pub fn taxonomy_checker(&self) -> &Arc<dyn ITaxonomyProtocol> {
        &self.taxonomy_checker
    }

    pub fn contract_checker(&self) -> &Arc<dyn IContractProtocol> {
        &self.contract_checker
    }

    pub fn class_checker(&self) -> &Arc<dyn IMandatoryClassProtocol> {
        &self.class_checker
    }

    pub fn naming_checker(&self) -> &Arc<dyn INamingProtocol> {
        &self.naming_checker
    }

    pub fn import_mandatory_checker(&self) -> &Arc<dyn IImportMandatoryProtocol> {
        &self.import_mandatory_checker
    }

    pub fn import_intent_checker(&self) -> &Arc<dyn IImportIntentProtocol> {
        &self.import_intent_checker
    }

    pub fn import_forbidden_checker(&self) -> &Arc<dyn IImportForbiddenProtocol> {
        &self.import_forbidden_checker
    }

    pub fn cycle_analyzer(&self) -> &Arc<dyn ICycleAnalysisProtocol> {
        &self.cycle_analyzer
    }

    pub fn surface_checker(&self) -> &Arc<dyn ISurfaceProtocol> {
        &self.surface_checker
    }

    pub fn orphan_aggregate(&self) -> &Arc<dyn IOrphanAggregate> {
        &self.orphan_aggregate
    }

    pub fn detect_layer(&self, _file: &str, _root_dir: &str) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }

    pub fn get_layer_def(&self, _layer: &shared::taxonomy_layer_vo::LayerNameVO) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        None
    }

    pub fn analyzer(&self) -> &Arc<dyn IAnalyzer> {
        panic!("analyzer not initialized")
    }

    pub fn as_ref(&self) -> &dyn CheckerContainerRef {
        self
    }
}

/// Trait for dynamic dispatch of CheckerContainer
pub trait CheckerContainerRef {
    fn detect_layer(&self, file: &str, root_dir: &str) -> Option<shared::taxonomy_layer_vo::LayerNameVO>;
    fn get_layer_def(&self, layer: &shared::taxonomy_layer_vo::LayerNameVO) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition>;
}

// Local protocols that aren't in shared
pub trait ICapabilitiesRoleProtocol: Send + Sync {
    fn check_capability_routing(&self, source: &shared::config_system::taxonomy_source_vo::SourceContentVO, layer: &shared::taxonomy_layer_vo::LayerNameVO, violations: &mut Vec<LintResult>);
}

pub trait ITaxonomyProtocol: Send + Sync {
    fn check_entity(&self, source: &shared::config_system::taxonomy_source_vo::SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_error(&self, source: &shared::config_system::taxonomy_source_vo::SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_event(&self, source: &shared::config_system::taxonomy_source_vo::SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_constant(&self, source: &shared::config_system::taxonomy_source_vo::SourceContentVO, violations: &mut Vec<LintResult>);
}

pub trait IContractProtocol: Send + Sync {
    fn check_aggregate(&self, source: &shared::config_system::taxonomy_source_vo::SourceContentVO, def: &shared::common::taxonomy_definition_vo::LayerDefinition, violations: &mut Vec<LintResult>);
}

pub trait INamingProtocol: Send + Sync {
    fn check_file_naming(&self, analyzer: &Arc<dyn IAnalyzer>, files: &FilePathList, root: &FilePath, violations: &mut LintResultList);
    fn check_domain_suffixes(&self, analyzer: &Arc<dyn IAnalyzer>, files: &FilePathList, root: &FilePath, violations: &mut LintResultList);
}

pub trait IImportMandatoryProtocol: Send + Sync {
    fn check_mandatory_imports(&self, analyzer: &Arc<dyn IAnalyzer>, files: &FilePathList, root: &FilePath, violations: &mut LintResultList);
}

pub trait IImportIntentProtocol: Send + Sync {
    fn check_mandatory_imports(&self, analyzer: &Arc<dyn IAnalyzer>, files: &FilePathList, root: &FilePath, violations: &mut LintResultList);
}

pub trait IImportForbiddenProtocol: Send + Sync {
    fn check_forbidden_imports(&self, analyzer: &Arc<dyn IAnalyzer>, files: &FilePathList, root: &FilePath, violations: &mut LintResultList);
    fn check_legacy_import_rules(&self, analyzer: &Arc<dyn IAnalyzer>, files: &FilePathList, root: &FilePath, violations: &mut LintResultList);
}

pub trait ISurfaceProtocol: Send + Sync {
    fn check_surface_hierarchy(&self, files: &[FilePath], root: &FilePath, violations: &mut LintResultList);
}

pub trait IOrphanAggregate: Send + Sync {
    fn check_orphans(&self, container: &dyn CheckerContainerRef, files: &[String], root_dir: &str) -> Vec<LintResult>;
}

/// RoleOrchestrator for agent and surface role checks
pub struct RoleOrchestrator {
    _aggregate: Arc<dyn IRoleAggregate>,
}

impl RoleOrchestrator {
    pub fn new(aggregate: Arc<dyn IRoleAggregate>) -> Self {
        Self { _aggregate: aggregate }
    }

    pub fn run_all_role_checks(&self, files: &[String], max_lines: usize, violations: &mut Vec<LintResult>) {
        // Placeholder implementation
        // In a full implementation, this would check AES0305 and AES0306
        for file in files {
            let content = std::fs::read_to_string(file).unwrap_or_default();
            let line_count = content.lines().count();
            if line_count > max_lines {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES0305",
                    shared::output_report::taxonomy_severity_vo::Severity::CRITICAL,
                    "Role violation: agent file exceeds max lines",
                ));
            }
        }
    }
}

impl CheckerContainerRef for CheckerContainer {
    fn detect_layer(&self, file: &str, root_dir: &str) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        self.detect_layer(file, root_dir)
    }
    fn get_layer_def(&self, layer: &shared::taxonomy_layer_vo::LayerNameVO) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.get_layer_def(layer)
    }
}

impl Default for CheckerContainer {
    fn default() -> Self {
        Self::new()
    }
}