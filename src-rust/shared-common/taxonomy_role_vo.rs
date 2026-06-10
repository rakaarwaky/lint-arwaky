// PURPOSE: LayerSuffixConfig, SuffixRuleVO — VOs for role suffix policy configuration
use serde::{Deserialize, Serialize};

use crate::shared_common::taxonomy_suggestion_vo::DescriptionVO;

/// Defines which suffix roles exist and their behavior mandates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleDefinition {
    pub suffix: DescriptionVO,
    pub layer: DescriptionVO,
    pub mandate: DescriptionVO,
    pub stateless: bool,
    pub single_goal: bool,
    pub forbid_any_type: bool,
}
