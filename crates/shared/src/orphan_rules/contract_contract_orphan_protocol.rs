// PURPOSE: IContractOrphanProtocol — orphan indicator for the contract layer (AES502)
use crate::common::taxonomy_path_vo::FilePath;
use crate::quality_rules::taxonomy_analysis_vo::{
    InheritanceMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

pub trait IContractOrphanProtocol: Send + Sync {
    /// Detect contract-layer orphans: traits/aggregates never implemented or exported.
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        inheritance_map: &InheritanceMap,
        all_files: &[String],
        content_map: &HashMap<String, String>,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}
