use crate::taxonomy::{
    file_path_vo::FilePath, path_collection_vo::FilePathList,
    lint_result_vo::LintResultList, common_error_vo::ModuleName,
    architecture_analysis_vo::{ImportGraph, ReachabilityResult, InboundLinkMap, GraphAnalysisContext, OrphanIndicatorResult, FileDefinitionMap},
    architecture_analysis_vo::{InheritanceMap, ModuleToFileMap},
    layer_definition_vo::LayerDefinition,
};
use crate::contract::{crate::contract::architecture_rule_protocol::IArchRuleProtocol, crate::contract::architecture_rule_protocol::IAnalyzer};
use async_trait::async_trait;

#[async_trait]
pub trait IArchOrphanProtocol: IArchRuleProtocol + Send + Sync {
    async fn check_orphans(
        &self,
        analyzer: &dyn IAnalyzer,
        all_files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

#[async_trait]
pub trait IOrphanGraphProtocol: Send + Sync {
    async fn build_graph_context(
        &self,
        analyzer: &dyn IAnalyzer,
        full_project_files: &FilePathList,
        root_dir: &FilePath,
    ) -> GraphAnalysisContext;

    async fn resolve_import_to_file(
        &self,
        analyzer: &dyn IAnalyzer,
        current_file: &FilePath,
        module_path: &ModuleName,
        root_dir: &FilePath,
        module_to_file: Option<&ModuleToFileMap>,
    ) -> Option<FilePath>;

    async fn identify_entry_points(
        &self,
        analyzer: &dyn IAnalyzer,
        all_files: &FilePathList,
        root_dir: &FilePath,
    ) -> FilePathList;

    async fn trace_reachability(
        &self, entry_points: &FilePathList, graph: &ImportGraph,
    ) -> ReachabilityResult;
}

#[async_trait]
pub trait IOrphanIndicatorProtocol: Send + Sync {
    async fn is_taxonomy_orphan(
        &self,
        analyzer: &dyn IAnalyzer,
        f: &FilePath,
        root_dir: &FilePath,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult;

    async fn is_contract_orphan(
        &self,
        analyzer: &dyn IAnalyzer,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
    ) -> OrphanIndicatorResult;

    async fn is_infra_cap_orphan(
        &self,
        analyzer: &dyn IAnalyzer,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;

    async fn is_agent_orphan(
        &self, analyzer: &dyn IAnalyzer, f: &FilePath, root_dir: &FilePath,
    ) -> OrphanIndicatorResult;

    async fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult;

    async fn is_generic_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult;
}
