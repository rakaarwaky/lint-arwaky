// PURPOSE: IAgentOrphanProtocol — orphan indicator for the agent layer (AES505)
use crate::common::taxonomy_path_vo::FilePath;
use crate::quality_rules::taxonomy_analysis_vo::{OrphanIndicatorResult, ReachabilityResult};
use std::collections::HashMap;

pub trait IAgentOrphanProtocol: Send + Sync {
    /// Detect agent-layer orphans: orchestrators never composed into a root container.
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        all_files: &[String],
        content_map: &HashMap<String, String>,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}
