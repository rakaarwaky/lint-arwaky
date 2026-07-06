// PURPOSE: OrphanRuleVO — value object containing orphan compliance rule definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::PatternList;
use serde::{Deserialize, Serialize};

fn default_check_orphan() -> BooleanVO {
    BooleanVO::new(true)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrphanRuleVO {
    #[serde(default = "default_check_orphan")]
    pub check_orphan: BooleanVO,
    #[serde(default, alias = "entry_points")]
    pub orphan_entry_points: PatternList,
}

impl Default for OrphanRuleVO {
    fn default() -> Self {
        Self {
            check_orphan: default_check_orphan(),
            orphan_entry_points: PatternList::default(),
        }
    }
}
