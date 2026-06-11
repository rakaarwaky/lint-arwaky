// PURPOSE: ICheckerContainer — trait for DI container that provides all AES checkers
// All traits are defined locally to avoid dependency cycles.
use crate::{
    IBypassCheckerProtocol, IDeadInheritanceProtocol, IInlineUnusedProtocol,
    IMandatoryClassProtocol, IMandatoryInheritanceProtocol, ILineCheckerProtocol,
    ICycleAnalysisProtocol,
};
use shared::contract_rule_protocol::IAnalyzer;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::taxonomy_result_vo::{LintResult, LintResultList};
use shared::{FilePath, FilePathList, LayerDefinition, SourceContentVO};
use std::sync::Arc;

pub trait ILocalNamingCheckerProtocol: Send + Sync {
    fn check_file_naming(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn check_domain_suffixes(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) -> impl std::future::Future<Output = ()> + Send;
}

pub trait ILocalArchImportProtocol: Send + Sync {
    fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn check_forbidden_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn check_legacy_import_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) -> impl std::future::Future<Output = ()> + Send;
}

pub trait ILocalCapabilitiesRoleChecker: Send + Sync {
    fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}

pub trait ILocalTaxonomyRoleChecker: Send + Sync {
    fn check_entity(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_error(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_event(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_constant(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}

pub trait ILocalContractRoleChecker: Send + Sync {
    fn check_aggregate(
        &self,
        source: &SourceContentVO,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );
}

pub trait ILocalSurfaceRoleChecker: Send + Sync {
    fn check_surface_hierarchy(
        &self,
        file_paths: &[FilePath],
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

pub trait ICheckerContainer: Send + Sync {
    fn bypass_checker(&self) -> &dyn IBypassCheckerProtocol;
    fn inline_unused_checker(&self) -> &dyn IInlineUnusedProtocol;
    fn dead_inheritance_checker(&self) -> &dyn IDeadInheritanceProtocol;
    fn mandatory_inheritance_checker(&self) -> &dyn IMandatoryInheritanceProtocol;
    fn line_checker(&self) -> &dyn ILineCheckerProtocol;
    fn class_checker(&self) -> &dyn IMandatoryClassProtocol;
    fn cycle_analyzer(&self) -> &dyn ICycleAnalysisProtocol;
    fn analyzer(&self) -> &dyn IAnalyzer;
    fn naming_checker(&self) -> &dyn ILocalNamingCheckerProtocol;
    fn import_mandatory_checker(&self) -> &dyn ILocalArchImportProtocol;
    fn import_intent_checker(&self) -> &dyn ILocalArchImportProtocol;
    fn import_forbidden_checker(&self) -> &dyn ILocalArchImportProtocol;
    fn capabilities_role_checker(&self) -> &dyn ILocalCapabilitiesRoleChecker;
    fn taxonomy_checker(&self) -> &dyn ILocalTaxonomyRoleChecker;
    fn contract_checker(&self) -> &dyn ILocalContractRoleChecker;
    fn surface_checker(&self) -> &dyn ILocalSurfaceRoleChecker;
    fn orphan_aggregate(&self) -> &Arc<dyn IOrphanAggregate>;
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String>;
    fn get_layer_def(&self, layer: &str) -> Option<LayerDefinition>;
}
