// PURPOSE: ITaxonomyOrphanProtocol + layer-specific orphan indicator protocols (agent, contract, capabilities, infra, surfaces)
use crate::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use crate::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use crate::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_path_vo::FilePath;

use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_source_vo::ContentString;
use crate::orphan_detector::taxonomy_contract_detection_utility;
use crate::orphan_detector::taxonomy_contract_regex_utility;
use crate::orphan_detector::taxonomy_filename_regex_utility;
use crate::orphan_detector::taxonomy_graph_regex_utility;
use crate::orphan_detector::taxonomy_surface_utility;

pub trait ITaxonomyOrphanProtocol: Send + Sync {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult;
}

pub trait IContractOrphanProtocol: Send + Sync {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
        all_files: &[FilePath],
    ) -> OrphanIndicatorResult;
}

pub trait ICapabilitiesOrphanProtocol: Send + Sync {
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}

pub trait IInfrastructureOrphanProtocol: Send + Sync {
    fn is_infrastructure_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}

pub trait IAgentOrphanProtocol: Send + Sync {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        all_files: &[FilePath],
    ) -> OrphanIndicatorResult;
}

pub trait ISurfacesOrphanProtocol: Send + Sync {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult;
}

pub trait IOrphanFileCachePort: Send + Sync {
    fn read_cached(&self, path: &FilePath) -> ContentString;
    fn read_dir(&self, dir_path: &FilePath) -> Vec<FilePath>;
    fn path_exists(&self, path: &FilePath) -> BooleanVO;
    fn is_symlink(&self, path: &FilePath) -> BooleanVO;
    fn clear_cache(&self);
}

pub trait IOrphanFilenameExtractorProtocol: Send + Sync {
    fn file_basename(&self, fp: &FilePath) -> Identity;
    fn file_stem(&self, fp: &FilePath) -> Identity;
    fn file_suffix(&self, fp: &FilePath) -> Identity;
    fn extract_struct_names(&self, content: &ContentString) -> Vec<SymbolName>;
    fn extract_trait_names(&self, content: &ContentString) -> Vec<SymbolName>;
}
