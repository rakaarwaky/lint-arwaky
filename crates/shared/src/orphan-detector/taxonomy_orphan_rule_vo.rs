// PURPOSE: OrphanRuleVO — value object containing orphan compliance rule definitions
use crate::common::taxonomy_common_vo::PatternList;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OrphanRuleVO {
    #[serde(default, alias = "entry_points")]
    pub orphan_entry_points: PatternList,
}
